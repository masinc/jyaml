//! JYAML lexer implementation

use crate::{Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum ChompingIndicator {
    Clip,  // default (preserve final newline)
    Strip, // remove all trailing newlines (-)
    Keep,  // preserve all trailing newlines (+)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Null,
    True,
    False,
    Number(String),
    String(String),

    // Punctuation
    Colon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dash,
    Pipe,
    PipeStrip,
    PipeKeep,
    Greater,
    GreaterStrip,
    GreaterKeep,

    // Special
    Newline,
    Indent(usize),
    Comment(String),
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current: Option<char>,
    position: usize,
    line: usize,
    column: usize,
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        // Check for BOM
        if input.starts_with('\u{FEFF}') {
            return Err(Error::BomNotAllowed);
        }

        // Validate UTF-8
        if let Err(e) = std::str::from_utf8(input.as_bytes()) {
            return Err(Error::InvalidUtf8 {
                position: e.valid_up_to(),
            });
        }

        let mut chars = input.chars();
        let current = chars.next();

        Ok(Lexer {
            input,
            chars,
            current,
            position: 0,
            line: 1,
            column: 1,
            at_line_start: true,
        })
    }

    pub fn next_token(&mut self) -> Result<Token> {
        // Handle indentation at line start
        if self.at_line_start {
            self.at_line_start = false;
            let indent = self.count_indent()?;
            if indent > 0 {
                return Ok(Token::Indent(indent));
            }
        }

        self.skip_whitespace();

        match self.current {
            None => Ok(Token::Eof),
            Some('\n') => {
                self.advance();
                self.at_line_start = true;
                Ok(Token::Newline)
            }
            Some('\t') => {
                let line = self.line;
                let column = self.column;
                self.advance(); // Important: advance past the tab character
                Err(Error::TabInIndentation { line, column })
            }
            Some('#') => self.read_comment(),
            Some('/') => {
                if self.peek() == Some('/') {
                    self.read_comment()
                } else {
                    self.error("Unexpected character '/'")
                }
            }
            Some(':') => {
                self.advance();
                Ok(Token::Colon)
            }
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some('[') => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            Some(']') => {
                self.advance();
                Ok(Token::RightBracket)
            }
            Some('{') => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RightBrace)
            }
            Some('-') => {
                self.advance();
                // Check if it's a number
                if self.current.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    self.read_number(true)
                } else {
                    Ok(Token::Dash)
                }
            }
            Some('|') => {
                self.advance();
                if self.current == Some('-') {
                    self.advance();
                    Ok(Token::PipeStrip)
                } else if self.current == Some('+') {
                    self.advance();
                    Ok(Token::PipeKeep)
                } else {
                    Ok(Token::Pipe)
                }
            }
            Some('>') => {
                self.advance();
                if self.current == Some('-') {
                    self.advance();
                    Ok(Token::GreaterStrip)
                } else if self.current == Some('+') {
                    self.advance();
                    Ok(Token::GreaterKeep)
                } else {
                    Ok(Token::Greater)
                }
            }
            Some('"') => self.read_double_quoted_string(),
            Some('\'') => self.read_single_quoted_string(),
            Some(c) if c.is_ascii_digit() => self.read_number(false),
            Some('+') => {
                self.advance();
                if self.current.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    self.read_number(false)
                } else {
                    self.error("Expected digit after '+'")
                }
            }
            Some(c) if c.is_ascii_alphabetic() => self.read_identifier(),
            Some(c) => self.error(&format!("Unexpected character '{c}'")),
        }
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current {
            self.position += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current = self.chars.next();
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current {
            if ch == ' ' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn count_indent(&mut self) -> Result<usize> {
        let mut count = 0;
        while let Some(ch) = self.current {
            match ch {
                ' ' => {
                    count += 1;
                    self.advance();
                }
                '\t' => {
                    // Tabs are not allowed in JYAML - error immediately
                    return Err(Error::TabInIndentation {
                        line: self.line,
                        column: self.column,
                    });
                }
                _ => break,
            }
        }
        Ok(count)
    }

    fn read_comment(&mut self) -> Result<Token> {
        // Skip # or //
        if self.current == Some('#') {
            self.advance();
        } else if self.current == Some('/') && self.peek() == Some('/') {
            self.advance();
            self.advance();
        }

        // Skip space after comment marker
        if self.current == Some(' ') {
            self.advance();
        }

        let mut comment = String::new();
        while let Some(ch) = self.current {
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.advance();
        }

        Ok(Token::Comment(comment))
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let mut ident = String::new();

        while let Some(ch) = self.current {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "null" => Ok(Token::Null),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            _ => self.error(&format!("Invalid identifier '{ident}'")),
        }
    }

    fn read_number(&mut self, negative: bool) -> Result<Token> {
        let mut number = String::new();
        if negative {
            number.push('-');
        }

        // Check for leading zero
        if self.current == Some('0') && self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            return self.error("Leading zeros are not allowed");
        }

        // Read integer part
        if !self.read_digits(&mut number) {
            return self.error("Expected digits in number");
        }

        // Read decimal part
        if self.current == Some('.') {
            number.push('.');
            self.advance();
            if !self.read_digits(&mut number) {
                return self.error("Expected digits after decimal point");
            }
        }

        // Read exponent part
        if self.current == Some('e') || self.current == Some('E') {
            number.push(self.current.unwrap());
            self.advance();

            if self.current == Some('+') || self.current == Some('-') {
                number.push(self.current.unwrap());
                self.advance();
            }

            if !self.read_digits(&mut number) {
                return self.error("Expected digits in exponent");
            }
        }

        Ok(Token::Number(number))
    }

    fn read_digits(&mut self, buffer: &mut String) -> bool {
        let mut found = false;
        while let Some(ch) = self.current {
            if ch.is_ascii_digit() {
                buffer.push(ch);
                self.advance();
                found = true;
            } else {
                break;
            }
        }
        found
    }

    fn read_double_quoted_string(&mut self) -> Result<Token> {
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.current {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(Token::String(string));
                }
                '\\' => {
                    self.advance();
                    match self.current {
                        Some('"') => string.push('"'),
                        Some('\\') => string.push('\\'),
                        Some('/') => string.push('/'),
                        Some('b') => string.push('\u{0008}'),
                        Some('f') => string.push('\u{000C}'),
                        Some('n') => string.push('\n'),
                        Some('r') => string.push('\r'),
                        Some('t') => string.push('\t'),
                        Some('u') => {
                            self.advance();
                            let code = self.read_unicode_escape()?;
                            string.push(code);
                            continue;
                        }
                        Some(ch) => {
                            return Err(Error::InvalidEscape {
                                line: self.line,
                                column: self.column,
                                sequence: ch,
                            });
                        }
                        None => return self.error("Unexpected end of input in string"),
                    }
                    self.advance();
                }
                '\n' | '\r' => {
                    return self.error("Unescaped newline in string");
                }
                ch if ch.is_control() => {
                    return self.error(&format!(
                        "Unescaped control character in string: \\u{:04x}",
                        ch as u32
                    ));
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        self.error("Unclosed string")
    }

    fn read_single_quoted_string(&mut self) -> Result<Token> {
        self.advance(); // Skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.current {
            match ch {
                '\'' => {
                    self.advance();
                    return Ok(Token::String(string));
                }
                '\\' => {
                    self.advance();
                    match self.current {
                        Some('\'') => {
                            string.push('\'');
                            self.advance();
                        }
                        Some('\\') => {
                            string.push('\\');
                            self.advance();
                        }
                        _ => {
                            // In single quotes, other escapes are literal
                            string.push('\\');
                        }
                    }
                }
                '\n' | '\r' => {
                    return self.error("Unescaped newline in string");
                }
                ch if ch.is_control() => {
                    return self.error(&format!(
                        "Unescaped control character in string: \\u{:04x}",
                        ch as u32
                    ));
                }
                _ => {
                    string.push(ch);
                    self.advance();
                }
            }
        }

        self.error("Unclosed string")
    }

    fn read_unicode_escape(&mut self) -> Result<char> {
        let mut code = 0u32;
        for _ in 0..4 {
            match self.current {
                Some(ch) if ch.is_ascii_hexdigit() => {
                    code = code * 16 + ch.to_digit(16).unwrap();
                    self.advance();
                }
                _ => return self.error("Invalid unicode escape sequence"),
            }
        }

        // Check if this is a high surrogate (D800-DBFF)
        if (0xD800..=0xDBFF).contains(&code) {
            // This is a high surrogate, we need to read the low surrogate
            match self.read_surrogate_pair(code) {
                Ok(ch) => Ok(ch),
                Err(_) => Err(Error::SyntaxError {
                    line: self.line,
                    column: self.column,
                    message: format!("Invalid surrogate pair starting with U+{code:04X}"),
                }),
            }
        } else if (0xDC00..=0xDFFF).contains(&code) {
            // This is a low surrogate without a high surrogate
            Err(Error::SyntaxError {
                line: self.line,
                column: self.column,
                message: format!("Unexpected low surrogate U+{code:04X}"),
            })
        } else {
            // Regular Unicode character
            char::from_u32(code).ok_or_else(|| Error::SyntaxError {
                line: self.line,
                column: self.column,
                message: format!("Invalid unicode code point U+{code:04X}"),
            })
        }
    }

    fn read_surrogate_pair(&mut self, high_surrogate: u32) -> Result<char> {
        // Expect "\u" for the low surrogate
        if self.current != Some('\\') {
            return self.error("Expected '\\' for low surrogate");
        }
        self.advance();

        if self.current != Some('u') {
            return self.error("Expected 'u' for low surrogate");
        }
        self.advance();

        // Read the low surrogate
        let mut low_code = 0u32;
        for _ in 0..4 {
            match self.current {
                Some(ch) if ch.is_ascii_hexdigit() => {
                    low_code = low_code * 16 + ch.to_digit(16).unwrap();
                    self.advance();
                }
                _ => return self.error("Invalid unicode escape sequence in low surrogate"),
            }
        }

        // Validate that it's actually a low surrogate
        if !(0xDC00..=0xDFFF).contains(&low_code) {
            return Err(Error::SyntaxError {
                line: self.line,
                column: self.column,
                message: format!("Expected low surrogate (DC00-DFFF), got U+{low_code:04X}"),
            });
        }

        // Convert surrogate pair to Unicode code point
        let code_point = 0x10000 + ((high_surrogate - 0xD800) << 10) + (low_code - 0xDC00);

        char::from_u32(code_point).ok_or_else(|| Error::SyntaxError {
            line: self.line,
            column: self.column,
            message: format!("Invalid unicode code point from surrogate pair U+{code_point:04X}"),
        })
    }

    fn error<T>(&self, message: &str) -> Result<T> {
        Err(Error::SyntaxError {
            line: self.line,
            column: self.column,
            message: message.to_string(),
        })
    }

    pub fn current_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Read multiline literal string content starting at a specific indent level
    /// When this is called, lexer should be positioned at the first content character
    pub fn read_multiline_block(
        &mut self,
        content_indent: usize,
        chomping: ChompingIndicator,
    ) -> Result<String> {
        let mut lines = Vec::new();

        // First, read the current line since we're already positioned at the content
        if self.current.is_some() && self.current != Some('\n') {
            let mut line_content = String::new();
            while let Some(ch) = self.current {
                if ch == '\n' {
                    self.advance();
                    self.at_line_start = true;
                    break;
                }
                line_content.push(ch);
                self.advance();
            }
            lines.push(line_content);
        }

        loop {
            // Read line-by-line, checking indentation manually
            let line_start_pos = self.position;

            // Count indentation on this line
            let mut line_indent = 0;
            while let Some(ch) = self.current {
                if ch == ' ' {
                    line_indent += 1;
                    self.advance();
                } else if ch == '\t' {
                    return Err(Error::TabInIndentation {
                        line: self.line,
                        column: self.column,
                    });
                } else {
                    break;
                }
            }

            // Check if we're at the end of the multiline block
            if self.current.is_none() {
                break;
            }

            // If line starts with less indentation than content_indent, we're done
            if line_indent < content_indent {
                // Reset position to start of this line
                self.position = line_start_pos;
                // Reset character iterator to current position
                self.chars = self.input[self.position..].chars();
                self.current = self.chars.next();
                // Recalculate line and column numbers
                let mut line = 1;
                let mut column = 1;
                for ch in self.input[..self.position].chars() {
                    if ch == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
                self.line = line;
                self.column = column;
                // Mark that we're at line start for proper tokenization
                self.at_line_start = true;
                break;
            }

            // If this is a comment line, skip it
            if self.current == Some('#') {
                while let Some(ch) = self.current {
                    if ch == '\n' {
                        self.advance();
                        self.at_line_start = true;
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            // If it's just a newline (empty line), handle it
            if self.current == Some('\n') {
                // For empty lines, preserve any spaces that were counted as indentation
                // beyond the content_indent level
                let spaces_beyond_indent = line_indent.saturating_sub(content_indent);
                lines.push(" ".repeat(spaces_beyond_indent));
                self.advance();
                self.at_line_start = true;
                continue;
            }

            // Read the rest of the line as content
            let mut line_content = String::new();
            while let Some(ch) = self.current {
                if ch == '\n' {
                    self.advance();
                    self.at_line_start = true;
                    break;
                }
                line_content.push(ch);
                self.advance();
            }

            // Build line with proper relative indentation
            let indent_diff = line_indent.saturating_sub(content_indent);
            let mut line = " ".repeat(indent_diff);
            line.push_str(&line_content);
            lines.push(line);
        }

        let mut result = lines.join("\n");

        // Apply chomping indicator
        match chomping {
            ChompingIndicator::Clip => {
                // Default: preserve final newline if content exists
                if !result.is_empty() {
                    result.push('\n');
                }
            }
            ChompingIndicator::Strip => {
                // Strip: remove all trailing newlines (do nothing, already stripped)
            }
            ChompingIndicator::Keep => {
                // Keep: preserve all trailing newlines
                if !result.is_empty() {
                    result.push('\n');
                }
                // Count additional trailing newlines from the final empty lines
                let trailing_newlines = lines.iter().rev()
                    .take_while(|line| line.trim().is_empty())
                    .count();
                
                // Add additional newlines for keep mode
                // For keep mode, we need at least one additional newline if there are any trailing empty lines
                if trailing_newlines > 0 {
                    for _ in 0..trailing_newlines {
                        result.push('\n');
                    }
                } else {
                    // Even without explicit empty lines, keep mode adds an extra newline
                    result.push('\n');
                }
            }
        }

        Ok(result)
    }

    /// Read multiline folded string content starting at a specific indent level
    /// When this is called, lexer should be positioned at the first content character
    pub fn read_folded_block(
        &mut self,
        content_indent: usize,
        chomping: ChompingIndicator,
    ) -> Result<String> {
        let mut paragraphs = Vec::new();
        let mut current_paragraph = Vec::new();
        let mut trailing_empty_lines = 0;

        // First, read the current line since we're already positioned at the content
        if self.current.is_some() && self.current != Some('\n') {
            let mut line_content = String::new();
            while let Some(ch) = self.current {
                if ch == '\n' {
                    self.advance();
                    self.at_line_start = true;
                    break;
                }
                line_content.push(ch);
                self.advance();
            }

            if !line_content.trim().is_empty() {
                current_paragraph.push(line_content.trim().to_string());
                trailing_empty_lines = 0; // Reset empty line count
            }
        }

        loop {
            // Read line-by-line, checking indentation manually
            let line_start_pos = self.position;

            // Count indentation on this line
            let mut line_indent = 0;
            while let Some(ch) = self.current {
                if ch == ' ' {
                    line_indent += 1;
                    self.advance();
                } else if ch == '\t' {
                    return Err(Error::TabInIndentation {
                        line: self.line,
                        column: self.column,
                    });
                } else {
                    break;
                }
            }

            // Check if we're at the end of the multiline block
            if self.current.is_none() {
                break;
            }

            // If line starts with less indentation than content_indent, check if it's an empty line
            if line_indent < content_indent {
                // If it's a newline (empty line), handle it
                if self.current == Some('\n') {
                    trailing_empty_lines += 1;
                    self.advance();
                    self.at_line_start = true;
                    continue;
                }

                // Otherwise, we're done with this block
                // Reset position to start of this line
                self.position = line_start_pos;
                // Reset character iterator to current position
                self.chars = self.input[self.position..].chars();
                self.current = self.chars.next();
                // Recalculate line and column numbers
                let mut line = 1;
                let mut column = 1;
                for ch in self.input[..self.position].chars() {
                    if ch == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
                self.line = line;
                self.column = column;
                // Mark that we're at line start for proper tokenization
                self.at_line_start = true;
                break;
            }

            // If this is a comment line, skip it
            if self.current == Some('#') {
                while let Some(ch) = self.current {
                    if ch == '\n' {
                        self.advance();
                        self.at_line_start = true;
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            // If it's just a newline (empty line), handle it
            if self.current == Some('\n') {
                if !current_paragraph.is_empty() {
                    // For Keep chomping indicator, preserve line breaks within paragraphs
                    let separator = if matches!(chomping, ChompingIndicator::Keep) {
                        "\n"
                    } else {
                        " "
                    };
                    paragraphs.push(current_paragraph.join(separator));
                    current_paragraph.clear();
                }
                trailing_empty_lines += 1;
                self.advance();
                self.at_line_start = true;
                continue;
            }

            // Read the rest of the line as content
            let mut line_content = String::new();
            while let Some(ch) = self.current {
                if ch == '\n' {
                    self.advance();
                    self.at_line_start = true;
                    break;
                }
                line_content.push(ch);
                self.advance();
            }

            // Build line with proper relative indentation
            let indent_diff = line_indent.saturating_sub(content_indent);
            let mut line = " ".repeat(indent_diff);
            line.push_str(&line_content);

            if line.trim().is_empty() {
                if !current_paragraph.is_empty() {
                    // For Keep chomping indicator, preserve line breaks within paragraphs
                    let separator = if matches!(chomping, ChompingIndicator::Keep) {
                        "\n"
                    } else {
                        " "
                    };
                    paragraphs.push(current_paragraph.join(separator));
                    current_paragraph.clear();
                }
                trailing_empty_lines += 1;
            } else {
                current_paragraph.push(line.trim_start().to_string());
                trailing_empty_lines = 0; // Reset empty line count
            }
        }

        if !current_paragraph.is_empty() {
            // Folded strings always join lines with spaces within paragraphs
            // The chomping indicator only affects trailing newlines, not line folding
            paragraphs.push(current_paragraph.join(" "));
        }

        let mut result = paragraphs.join("\n");

        // Apply chomping indicator
        match chomping {
            ChompingIndicator::Clip => {
                // Default: preserve final newline if content exists
                if !result.is_empty() {
                    result.push('\n');
                }
            }
            ChompingIndicator::Strip => {
                // Strip: remove all trailing newlines (do nothing, already stripped)
            }
            ChompingIndicator::Keep => {
                // Keep: preserve all trailing newlines
                if !result.is_empty() {
                    result.push('\n');
                    // Add all the trailing empty lines that were found
                    for _ in 0..trailing_empty_lines {
                        result.push('\n');
                    }
                    // For JYAML Keep mode, if no explicit trailing empty lines were found,
                    // add an extra trailing newline to preserve the "end of block" semantics
                    if trailing_empty_lines == 0 {
                        result.push('\n');
                    }
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("null true false").unwrap();

        assert_eq!(lexer.next_token().unwrap(), Token::Null);
        assert_eq!(lexer.next_token().unwrap(), Token::True);
        assert_eq!(lexer.next_token().unwrap(), Token::False);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 -10 1.5e-3").unwrap();

        assert_eq!(lexer.next_token().unwrap(), Token::Number("42".to_string()));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Number("3.14".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Number("-10".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Number("1.5e-3".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_strings() {
        let mut lexer = Lexer::new(r#""hello" 'world'"#).unwrap();

        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("hello".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("world".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_string_escapes() {
        let mut lexer = Lexer::new(r#""hello\nworld" "unicode: \u00A9""#).unwrap();

        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("hello\nworld".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("unicode: ©".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_single_quote_escapes() {
        let mut lexer = Lexer::new(r#"'can\'t stop' 'literal \n'"#).unwrap();

        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("can't stop".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::String("literal \\n".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::new(":,[]{}").unwrap();

        assert_eq!(lexer.next_token().unwrap(), Token::Colon);
        assert_eq!(lexer.next_token().unwrap(), Token::Comma);
        assert_eq!(lexer.next_token().unwrap(), Token::LeftBracket);
        assert_eq!(lexer.next_token().unwrap(), Token::RightBracket);
        assert_eq!(lexer.next_token().unwrap(), Token::LeftBrace);
        assert_eq!(lexer.next_token().unwrap(), Token::RightBrace);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_multiline_indicators() {
        let mut lexer = Lexer::new("| |- > >-").unwrap();

        assert_eq!(lexer.next_token().unwrap(), Token::Pipe);
        assert_eq!(lexer.next_token().unwrap(), Token::PipeStrip);
        assert_eq!(lexer.next_token().unwrap(), Token::Greater);
        assert_eq!(lexer.next_token().unwrap(), Token::GreaterStrip);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("# YAML comment\n// C comment").unwrap();

        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Comment("YAML comment".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Newline);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::Comment("C comment".to_string())
        );
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn test_indentation() {
        let input = "  item1\n    item2\n  item3";
        let mut lexer = Lexer::new(input).unwrap();

        assert_eq!(lexer.next_token().unwrap(), Token::Indent(2));
        // Note: lexer doesn't tokenize arbitrary words, so "item1" would be invalid
        // This test focuses on indentation detection
    }

    #[test]
    fn test_newlines() {
        let mut lexer = Lexer::new("line1\nline2\n").unwrap();

        // Skip to newlines (assuming line1/line2 are handled as identifiers)
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Newline => break,
                Token::Eof => panic!("Expected newline"),
                _ => continue,
            }
        }

        // Should find another newline
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Newline => break,
                Token::Eof => break,
                _ => continue,
            }
        }
    }

    #[test]
    fn test_bom_rejection() {
        let input_with_bom = "\u{FEFF}test";
        assert!(Lexer::new(input_with_bom).is_err());
    }

    #[test]
    fn test_tab_in_indentation() {
        let input = "\tindented";
        let mut lexer = Lexer::new(input).unwrap();

        // Should get an error when trying to parse indentation with tab
        let result = lexer.next_token();
        assert!(result.is_err());

        // Should be specifically a TabInIndentation error
        match result.unwrap_err() {
            Error::TabInIndentation { line, column } => {
                assert_eq!(line, 1);
                assert_eq!(column, 1);
            }
            _ => panic!("Expected TabInIndentation error"),
        }
    }

    #[test]
    fn test_tab_in_count_indent() {
        let input = "  \ttest";
        let mut lexer = Lexer::new(input).unwrap();

        // Should get an error when encountering tab during indentation counting
        let result = lexer.next_token();
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::TabInIndentation { line, column } => {
                assert_eq!(line, 1);
                assert_eq!(column, 3); // Tab is at the third position
            }
            _ => panic!("Expected TabInIndentation error"),
        }
    }

    #[test]
    fn test_tab_anywhere_in_line() {
        let input = "\"valid\"\ttest";
        let mut lexer = Lexer::new(input).unwrap();

        // Skip the first string token
        lexer.next_token().unwrap();

        // Should get tab error on the tab character
        let result = lexer.next_token();
        assert!(result.is_err());

        match result.unwrap_err() {
            Error::TabInIndentation { line, column } => {
                assert_eq!(line, 1);
                assert_eq!(column, 8); // Tab is after "valid" (including quotes)
            }
            _ => panic!("Expected TabInIndentation error"),
        }
    }

    #[test]
    fn test_invalid_number_leading_zero() {
        let mut lexer = Lexer::new("01234").unwrap();
        assert!(lexer.next_token().is_err());
    }

    #[test]
    fn test_unclosed_string() {
        let mut lexer = Lexer::new(r#""unclosed string"#).unwrap();
        assert!(lexer.next_token().is_err());
    }

    #[test]
    fn test_invalid_escape() {
        let mut lexer = Lexer::new(r#""invalid \x escape""#).unwrap();
        assert!(lexer.next_token().is_err());
    }

    #[test]
    fn test_invalid_unicode_escape() {
        let mut lexer = Lexer::new(r#""invalid \uGGGG""#).unwrap();
        assert!(lexer.next_token().is_err());
    }

    #[test]
    fn test_line_column_tracking() {
        let mut lexer = Lexer::new("line1\nline2").unwrap();

        assert_eq!(lexer.current_position(), (1, 1));

        // Advance through tokens and check position tracking
        while let Ok(token) = lexer.next_token() {
            if matches!(token, Token::Eof) {
                break;
            }
            // Position should advance
            let (line, col) = lexer.current_position();
            assert!(line >= 1);
            assert!(col >= 1);
        }
    }
}

#[cfg(test)]
mod unicode_tests {
    use super::*;

    #[test]
    fn test_emoji_current_state() {
        // Test current state: emoji without escaping
        let mut lexer = Lexer::new(r#""🚀""#).unwrap();
        let token = lexer.next_token().unwrap();
        if let Token::String(s) = token {
            println!("Emoji direct: {}", s);
            assert_eq!(s, "🚀");
        } else {
            panic!("Expected string token");
        }
    }

    #[test]
    fn test_surrogate_pair_needed() {
        // This should work with JYAML 0.4 spec but currently fails
        let result = Lexer::new(r#""\uD83D\uDE80""#);
        match result {
            Ok(mut lexer) => match lexer.next_token() {
                Ok(Token::String(s)) => {
                    println!("Surrogate pair result: {}", s);
                    assert_eq!(s, "🚀", "Should parse surrogate pair as emoji");
                }
                Ok(other) => panic!("Expected string, got {:?}", other),
                Err(e) => println!("Surrogate pair parse error: {}", e),
            },
            Err(e) => println!("Surrogate pair lexer error: {}", e),
        }
    }

    #[test]
    fn test_unicode_escapes_bmp() {
        // Test BMP characters (should work)
        let mut lexer = Lexer::new(r#""\u00A9\u00AE\u2603""#).unwrap();
        let token = lexer.next_token().unwrap();
        if let Token::String(s) = token {
            println!("BMP Unicode: {}", s);
            assert_eq!(s, "©®☃");
        } else {
            panic!("Expected string token");
        }
    }
}
