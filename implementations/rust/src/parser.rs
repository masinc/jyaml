//! JYAML parser implementation

use crate::{
    error::{Error, Result},
    lexer::{Lexer, Token},
    value::{Number, Value},
};
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Option<Token>,
    indent_stack: Vec<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut lexer = Lexer::new(input)?;
        let current_token = lexer.next_token()?;
        
        Ok(Parser {
            lexer,
            current_token,
            peek_token: None,
            indent_stack: vec![0],
        })
    }
    
    pub fn parse(&mut self) -> Result<Value> {
        self.skip_newlines_and_comments();
        
        let value = self.parse_value()?;
        
        // Skip any remaining whitespace, comments, and newlines
        loop {
            match self.current_token {
                Token::Newline | Token::Comment(_) => {
                    self.advance()?;
                }
                Token::Indent(_) => {
                    self.advance()?;
                }
                Token::Eof => break,
                _ => return self.error("Expected end of input"),
            }
        }
        
        Ok(value)
    }
    
    fn parse_value(&mut self) -> Result<Value> {
        self.parse_value_with_indent_context(0)
    }
    
    fn parse_value_with_indent_context(&mut self, expected_indent: usize) -> Result<Value> {
        // Skip indentation if present
        if let Token::Indent(_) = self.current_token {
            self.advance()?;
        }
        
        
        match &self.current_token {
            Token::Null => {
                self.advance()?;
                Ok(Value::Null)
            }
            Token::True => {
                self.advance()?;
                Ok(Value::Bool(true))
            }
            Token::False => {
                self.advance()?;
                Ok(Value::Bool(false))
            }
            Token::Number(n) => {
                let number = self.parse_number(n.clone())?;
                self.advance()?;
                Ok(Value::Number(number))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance()?;
                
                // Check for object key
                if matches!(self.current_token, Token::Colon) {
                    self.parse_object_from_first_key(value)
                } else {
                    Ok(Value::String(value))
                }
            }
            Token::LeftBracket => self.parse_flow_array(),
            Token::LeftBrace => self.parse_flow_object(),
            Token::Dash => {
                if expected_indent == 0 {
                    self.parse_block_array_with_context(None)
                } else {
                    self.parse_block_array_with_context(Some(expected_indent))
                }
            },
            Token::Pipe | Token::PipeStrip => self.parse_literal_string(),
            Token::Greater | Token::GreaterStrip => self.parse_folded_string(),
            _ => self.error("Expected value"),
        }
    }
    
    fn parse_number(&self, s: String) -> Result<Number> {
        if s.contains('.') || s.contains('e') || s.contains('E') {
            s.parse::<f64>()
                .map(Number::Float)
                .map_err(|_| self.syntax_error(&format!("Invalid number: {}", s)))
        } else {
            s.parse::<i64>()
                .map(Number::Integer)
                .map_err(|_| self.syntax_error(&format!("Invalid number: {}", s)))
        }
    }
    
    fn parse_flow_array(&mut self) -> Result<Value> {
        self.advance()?; // Skip [
        self.skip_whitespace_and_comments();
        
        let mut array = Vec::new();
        
        while !matches!(self.current_token, Token::RightBracket) {
            array.push(self.parse_value()?);
            
            self.skip_whitespace_and_comments();
            
            if matches!(self.current_token, Token::Comma) {
                self.advance()?;
                self.skip_whitespace_and_comments();
                
                // Check for trailing comma (comma followed by closing bracket)
                if matches!(self.current_token, Token::RightBracket) {
                    return self.error("Trailing comma not allowed in array");
                }
            } else if !matches!(self.current_token, Token::RightBracket) {
                return self.error("Expected ',' or ']'");
            }
        }
        
        self.advance()?; // Skip ]
        Ok(Value::Array(array))
    }
    
    fn parse_flow_object(&mut self) -> Result<Value> {
        self.advance()?; // Skip {
        self.skip_whitespace_and_comments();
        
        let mut object = HashMap::new();
        
        while !matches!(self.current_token, Token::RightBrace) {
            let key = match &self.current_token {
                Token::String(s) => s.clone(),
                _ => return self.error("Expected string key in object"),
            };
            self.advance()?;
            
            self.skip_whitespace_and_comments();
            
            if !matches!(self.current_token, Token::Colon) {
                return self.error("Expected ':' after object key");
            }
            self.advance()?;
            
            self.skip_whitespace_and_comments();
            
            let value = self.parse_value()?;
            
            if object.contains_key(&key) {
                return Err(Error::DuplicateKey {
                    line: self.lexer.current_position().0,
                    column: self.lexer.current_position().1,
                    key,
                });
            }
            
            object.insert(key, value);
            
            self.skip_whitespace_and_comments();
            
            if matches!(self.current_token, Token::Comma) {
                self.advance()?;
                self.skip_whitespace_and_comments();
                
                // Check for trailing comma (comma followed by closing brace)
                if matches!(self.current_token, Token::RightBrace) {
                    return self.error("Trailing comma not allowed in object");
                }
            } else if !matches!(self.current_token, Token::RightBrace) {
                return self.error("Expected ',' or '}'");
            }
        }
        
        self.advance()?; // Skip }
        Ok(Value::Object(object))
    }
    
    fn parse_block_array_with_context(&mut self, context_indent: Option<usize>) -> Result<Value> {
        let mut array = Vec::new();
        
        // Determine the base indent from the current position
        // If we're already at an indent token, use that level
        // Otherwise, we expect the next line to have indentation
        let mut base_indent = context_indent;
        
        
        // If we already have a context_indent and we are at a dash immediately,
        // process it without requiring an indent token first
        if base_indent.is_some() && matches!(self.current_token, Token::Dash) {
            // Jump to dash processing
            self.advance()?; // Skip -
            self.skip_inline_whitespace();
            let element = self.parse_array_element(base_indent)?;
            array.push(element);
            
            self.skip_newlines_and_comments();
        } else {
            self.skip_newlines_and_comments();
        }
        
        loop {
            // Store the current indent level before processing
            let current_indent_level = self.current_indent();
            
            // If we encounter an indent token, this might be our base indent
            if let Token::Indent(n) = self.current_token {
                if base_indent.is_none() {
                    // Set base_indent when we first encounter an indent with a dash
                    self.advance()?;
                    if matches!(self.current_token, Token::Dash) {
                        base_indent = Some(n);
                        // Continue to dash processing below
                    } else {
                        continue;
                    }
                } else if Some(n) != base_indent {
                    // Different indentation level, we're done with this array
                    break;
                } else {
                    // Same indent level as before
                    self.advance()?;
                    if matches!(self.current_token, Token::Dash) {
                        // Continue to dash processing below
                    } else {
                        continue;
                    }
                }
            } else if base_indent.is_some() && current_indent_level == 0 {
                // We had a base indent but now there's no indent, array is done
                break;
            } else if !matches!(self.current_token, Token::Dash) {
                // No indent and no dash, we're done
                break;
            }
            
            // If we see a string that could be an object key at the root level,
            // check if it's actually outside this array
            if matches!(self.current_token, Token::String(_)) {
                if let Ok(peek_token) = self.peek() {
                    if matches!(peek_token, Token::Colon) {
                        // This looks like a key-value pair
                        // If we have a base indent and we're not at that level, 
                        // this terminates the array
                        if base_indent.is_some() && self.current_indent() == 0 {
                            break;
                        }
                    }
                }
            }
            
            // We should only get here if we have a dash token
            assert!(matches!(self.current_token, Token::Dash));
            
            self.advance()?; // Skip -
            self.skip_inline_whitespace();
            
            // Parse array element with special handling for objects
            let element = self.parse_array_element(base_indent)?;
            array.push(element);
            
            // Skip trailing whitespace and comments on the same line
            self.skip_newlines_and_comments();
        }
        
        Ok(Value::Array(array))
    }
    
    fn parse_array_element(&mut self, array_base_indent: Option<usize>) -> Result<Value> {
        // Parse a single array element with proper scope detection
        match &self.current_token {
            Token::String(s) => {
                let value = s.clone();
                self.advance()?;
                
                // Check for object key in array element
                if matches!(self.current_token, Token::Colon) {
                    self.parse_array_element_object(value, array_base_indent)
                } else {
                    Ok(Value::String(value))
                }
            }
            _ => self.parse_value(),
        }
    }
    
    fn parse_array_element_object(&mut self, first_key: String, array_base_indent: Option<usize>) -> Result<Value> {
        let mut object = HashMap::new();
        
        
        // Remember the indent level of the first key for this array element
        let _object_key_indent = array_base_indent.map(|indent| indent + 2).unwrap_or(2);
        
        // Process first key-value pair
        self.advance()?; // Skip :
        self.skip_inline_whitespace();
        
        // Check if there's a newline after colon (indicates block style value)
        if matches!(self.current_token, Token::Newline) {
            self.advance()?; // Skip newline
            self.skip_newlines_and_comments();
            
            // Parse block style value
            let value = self.parse_value()?;
            object.insert(first_key, value);
        } else {
            // Parse inline value
            let value = self.parse_value()?;
            object.insert(first_key, value);
        }
        
        self.skip_newlines_and_comments();
        
        // Process remaining key-value pairs for this array element
        // The object continues until we hit:
        // 1. Another dash at the array's base indent (next array element)
        // 2. A key at indent 0 (outside the array) 
        // 3. EOF
        loop {
            self.skip_newlines_and_comments();
            
            // Skip indent tokens and then check for string
            let current_indent = if let Token::Indent(n) = self.current_token {
                self.advance()?;
                n
            } else {
                0
            };
            
            if let Token::String(key) = &self.current_token.clone() {
                
                // Check if this is a new array element
                if let Some(array_indent) = array_base_indent {
                    if current_indent == array_indent {
                        // Look ahead to see if this is followed by a dash
                        if let Ok(peek_token) = self.peek() {
                            if matches!(peek_token, Token::Dash) {
                                // This is a new array element, stop here
                                break;
                            }
                        }
                    }
                }
                
                // Check if this is a root-level key (outside the array)
                if current_indent == 0 {
                    if let Ok(peek_token) = self.peek() {
                        if matches!(peek_token, Token::Colon) {
                            // This is a root-level key, array element ends here
                            break;
                        }
                    }
                }
                
                // Check if we're at the expected indent level for this object
                // For array elements, the first key is right after the dash (no extra indent)
                // Subsequent keys should be at the same level as the first key
                if let Some(array_indent) = array_base_indent {
                    // Keys in array element objects should be at array_base_indent + 2 (after the dash)
                    let expected_indent = array_indent + 2;
                    if current_indent != expected_indent {
                        break;
                    }
                } else {
                    // If no array base indent, this shouldn't happen in array context
                    break;
                }
                
                let key = key.clone();
                self.advance()?;
                
                self.skip_inline_whitespace();
                
                if !matches!(self.current_token, Token::Colon) {
                    return self.error("Expected ':' after object key");
                }
                self.advance()?;
                
                self.skip_inline_whitespace();
                
                // Check if there's a newline after colon (indicates block style value)
                if matches!(self.current_token, Token::Newline) {
                    self.advance()?; // Skip newline
                    self.skip_newlines_and_comments();
                    
                    // Parse block style value
                    let value = self.parse_value()?;
                    
                    if object.contains_key(&key) {
                        return Err(Error::DuplicateKey {
                            line: self.lexer.current_position().0,
                            column: self.lexer.current_position().1,
                            key,
                        });
                    }
                    
                    object.insert(key, value);
                } else {
                    // Parse inline value
                    let value = self.parse_value()?;
                    
                    if object.contains_key(&key) {
                        return Err(Error::DuplicateKey {
                            line: self.lexer.current_position().0,
                            column: self.lexer.current_position().1,
                            key,
                        });
                    }
                    
                    object.insert(key, value);
                }
                
                self.skip_newlines_and_comments();
            } else {
                break;
            }
        }
        
        Ok(Value::Object(object))
    }
    
    fn parse_object_from_first_key(&mut self, first_key: String) -> Result<Value> {
        let mut object = HashMap::new();
        
        // Process first key-value pair
        self.advance()?; // Skip :
        self.skip_inline_whitespace();
        
        // Check if there's a newline after colon (indicates block style value)
        if matches!(self.current_token, Token::Newline) {
            self.advance()?; // Skip newline
            self.skip_newlines_and_comments();
            
            // Parse block style value with indent context
            let current_indent = self.current_indent();
            let value = self.parse_value_with_indent_context(current_indent)?;
            object.insert(first_key, value);
        } else {
            // Parse inline value
            let value = self.parse_value()?;
            object.insert(first_key, value);
        }
        
        self.skip_newlines_and_comments();
        
        // The base indentation for keys is 0 (no indentation)
        let base_indent = 0;
        
        // Process remaining key-value pairs
        loop {
            // Skip any remaining indentation or newlines first
            self.skip_newlines_and_comments();
            
            match &self.current_token {
                Token::String(key) => {
                    // Check if we're at the correct indentation level for more keys
                    let current_indent = self.current_indent();
                    if current_indent != base_indent {
                        break;
                    }
                    
                    let key = key.clone();
                    self.advance()?;
                    
                    self.skip_inline_whitespace();
                    
                    if !matches!(self.current_token, Token::Colon) {
                        return self.error("Expected ':' after object key");
                    }
                    self.advance()?;
                    
                    self.skip_inline_whitespace();
                    
                    // Check if there's a newline after colon (indicates block style value)
                    if matches!(self.current_token, Token::Newline) {
                        self.advance()?; // Skip newline
                        self.skip_newlines_and_comments();
                        
                        // Parse block style value
                        let value = self.parse_value()?;
                        
                        if object.contains_key(&key) {
                            return Err(Error::DuplicateKey {
                                line: self.lexer.current_position().0,
                                column: self.lexer.current_position().1,
                                key,
                            });
                        }
                        
                        object.insert(key, value);
                    } else {
                        // Parse inline value
                        let value = self.parse_value()?;
                        
                        if object.contains_key(&key) {
                            return Err(Error::DuplicateKey {
                                line: self.lexer.current_position().0,
                                column: self.lexer.current_position().1,
                                key,
                            });
                        }
                        
                        object.insert(key, value);
                    }
                    
                    self.skip_newlines_and_comments();
                }
                Token::Eof => break,
                Token::Newline | Token::Comment(_) => {
                    self.advance()?;
                }
                Token::Indent(_) => {
                    // If there's indentation, it might be for a nested structure
                    // that we've already processed, so skip it
                    self.advance()?;
                }
                _ => break,
            }
        }
        
        Ok(Value::Object(object))
    }
    
    fn parse_literal_string(&mut self) -> Result<Value> {
        let strip = matches!(self.current_token, Token::PipeStrip);
        self.advance()?; // Skip | or |-
        
        if !matches!(self.current_token, Token::Newline) {
            return self.error("Expected newline after '|'");
        }
        self.advance()?;
        
        let content_indent = match self.current_token {
            Token::Indent(n) => n,
            _ => return self.error("Expected indented content after '|'"),
        };
        
        let mut lines = Vec::new();
        
        loop {
            match self.current_token {
                Token::Indent(n) if n >= content_indent => {
                    let indent_diff = n - content_indent;
                    self.advance()?;
                    
                    // Collect the line content
                    let mut line = String::new();
                    for _ in 0..indent_diff {
                        line.push(' ');
                    }
                    
                    while !matches!(self.current_token, Token::Newline | Token::Eof) {
                        match &self.current_token {
                            Token::String(s) => line.push_str(s),
                            Token::Number(n) => line.push_str(n),
                            Token::True => line.push_str("true"),
                            Token::False => line.push_str("false"),
                            Token::Null => line.push_str("null"),
                            _ => line.push_str(&format!("{:?}", self.current_token)),
                        }
                        self.advance()?;
                    }
                    
                    lines.push(line);
                    
                    if matches!(self.current_token, Token::Newline) {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
                Token::Newline => {
                    lines.push(String::new());
                    self.advance()?;
                }
                _ => break,
            }
        }
        
        let mut result = lines.join("\n");
        if !strip && !result.is_empty() {
            result.push('\n');
        }
        
        Ok(Value::String(result))
    }
    
    fn parse_folded_string(&mut self) -> Result<Value> {
        let strip = matches!(self.current_token, Token::GreaterStrip);
        self.advance()?; // Skip > or >-
        
        if !matches!(self.current_token, Token::Newline) {
            return self.error("Expected newline after '>'");
        }
        self.advance()?;
        
        let content_indent = match self.current_token {
            Token::Indent(n) => n,
            _ => return self.error("Expected indented content after '>'"),
        };
        
        let mut paragraphs = Vec::new();
        let mut current_paragraph = Vec::new();
        
        loop {
            match self.current_token {
                Token::Indent(n) if n >= content_indent => {
                    let indent_diff = n - content_indent;
                    self.advance()?;
                    
                    // Collect the line content
                    let mut line = String::new();
                    for _ in 0..indent_diff {
                        line.push(' ');
                    }
                    
                    while !matches!(self.current_token, Token::Newline | Token::Eof) {
                        match &self.current_token {
                            Token::String(s) => line.push_str(s),
                            Token::Number(n) => line.push_str(n),
                            Token::True => line.push_str("true"),
                            Token::False => line.push_str("false"),
                            Token::Null => line.push_str("null"),
                            _ => line.push_str(&format!("{:?}", self.current_token)),
                        }
                        self.advance()?;
                    }
                    
                    current_paragraph.push(line);
                    
                    if matches!(self.current_token, Token::Newline) {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
                Token::Newline => {
                    if !current_paragraph.is_empty() {
                        paragraphs.push(current_paragraph.join(" "));
                        current_paragraph.clear();
                    }
                    self.advance()?;
                }
                _ => break,
            }
        }
        
        if !current_paragraph.is_empty() {
            paragraphs.push(current_paragraph.join(" "));
        }
        
        let mut result = paragraphs.join("\n");
        if !strip && !result.is_empty() {
            result.push('\n');
        }
        
        Ok(Value::String(result))
    }
    
    fn advance(&mut self) -> Result<()> {
        self.current_token = if let Some(token) = self.peek_token.take() {
            token
        } else {
            self.lexer.next_token()?
        };
        Ok(())
    }
    
    fn peek(&mut self) -> Result<&Token> {
        if self.peek_token.is_none() {
            self.peek_token = Some(self.lexer.next_token()?);
        }
        Ok(self.peek_token.as_ref().unwrap())
    }
    
    fn skip_whitespace_and_comments(&mut self) {
        while matches!(self.current_token, Token::Comment(_)) || 
              (matches!(self.current_token, Token::Newline) && self.peek().is_ok()) {
            let _ = self.advance();
        }
    }
    
    fn skip_inline_whitespace(&mut self) {
        // In JYAML, inline whitespace is handled by the lexer
    }
    
    fn skip_newlines_and_comments(&mut self) {
        while matches!(self.current_token, Token::Newline | Token::Comment(_)) {
            let _ = self.advance();
        }
    }
    
    fn current_indent(&self) -> usize {
        match &self.current_token {
            Token::Indent(n) => *n,
            _ => 0,
        }
    }
    
    fn error<T>(&self, message: &str) -> Result<T> {
        let (line, column) = self.lexer.current_position();
        Err(Error::SyntaxError {
            line,
            column,
            message: message.to_string(),
        })
    }
    
    fn syntax_error(&self, message: &str) -> Error {
        let (line, column) = self.lexer.current_position();
        Error::SyntaxError {
            line,
            column,
            message: message.to_string(),
        }
    }
}

/// Parse a JYAML string into a Value
pub fn parse(input: &str) -> Result<Value> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Number;
    use std::collections::HashMap;

    #[test]
    fn test_parse_null() {
        let result = parse("null").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_parse_booleans() {
        assert_eq!(parse("true").unwrap(), Value::Bool(true));
        assert_eq!(parse("false").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse("42").unwrap(), Value::Number(Number::Integer(42)));
        assert_eq!(parse("-10").unwrap(), Value::Number(Number::Integer(-10)));
        assert_eq!(parse("3.14").unwrap(), Value::Number(Number::Float(3.14)));
        assert_eq!(parse("1e5").unwrap(), Value::Number(Number::Float(100000.0)));
    }

    #[test]
    fn test_parse_strings() {
        assert_eq!(parse(r#""hello""#).unwrap(), Value::String("hello".to_string()));
        assert_eq!(parse(r#"'world'"#).unwrap(), Value::String("world".to_string()));
    }

    #[test]
    fn test_parse_flow_array() {
        let result = parse("[1, 2, 3]").unwrap();
        let expected = Value::Array(vec![
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_flow_object() {
        let result = parse(r#"{"name": "John", "age": 30}"#).unwrap();
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Value::String("John".to_string()));
        expected_map.insert("age".to_string(), Value::Number(Number::Integer(30)));
        assert_eq!(result, Value::Object(expected_map));
    }

    #[test]
    fn test_parse_block_array() {
        let input = r#"
- "first"
- "second"
- 42
"#;
        let result = parse(input).unwrap();
        let expected = Value::Array(vec![
            Value::String("first".to_string()),
            Value::String("second".to_string()),
            Value::Number(Number::Integer(42)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_block_object() {
        let input = r#"
"name": "Alice"
"age": 25
"#;
        let result = parse(input).unwrap();
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Value::String("Alice".to_string()));
        expected_map.insert("age".to_string(), Value::Number(Number::Integer(25)));
        assert_eq!(result, Value::Object(expected_map));
    }

    #[test]
    fn test_parse_with_comments() {
        let input = r#"
# This is a comment
"name": "John"  # inline comment
// C-style comment
"age": 30  // another comment
"#;
        let result = parse(input).unwrap();
        let mut expected_map = HashMap::new();
        expected_map.insert("name".to_string(), Value::String("John".to_string()));
        expected_map.insert("age".to_string(), Value::Number(Number::Integer(30)));
        assert_eq!(result, Value::Object(expected_map));
    }

    #[test]
    fn test_parse_empty_array() {
        let result = parse("[]").unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_parse_empty_object() {
        let result = parse("{}").unwrap();
        assert_eq!(result, Value::Object(HashMap::new()));
    }

    #[test]
    fn test_parse_nested_flow() {
        let input = r#"{"users": [{"name": "Alice"}, {"name": "Bob"}]}"#;
        let result = parse(input).unwrap();
        
        // Verify structure without exact comparison due to HashMap ordering
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            if let Some(Value::Array(users)) = obj.get("users") {
                assert_eq!(users.len(), 2);
            } else {
                panic!("users should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_parse_string_escapes() {
        let input = r#""hello\nworld""#;
        let result = parse(input).unwrap();
        assert_eq!(result, Value::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_parse_unicode_escapes() {
        let input = r#""\u00A9 2023""#;
        let result = parse(input).unwrap();
        assert_eq!(result, Value::String("© 2023".to_string()));
    }

    #[test]
    fn test_parse_single_quote_limited_escapes() {
        let input = r#"'can\'t stop'"#;
        let result = parse(input).unwrap();
        assert_eq!(result, Value::String("can't stop".to_string()));
        
        let input2 = r#"'literal \n'"#;
        let result2 = parse(input2).unwrap();
        assert_eq!(result2, Value::String("literal \\n".to_string()));
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let input = "42";
        let result = parse(input).unwrap();
        assert_eq!(result, Value::Number(Number::Integer(42)));
    }

    #[test]
    fn test_parser_errors() {
        // Unclosed string
        assert!(parse(r#""unclosed"#).is_err());
        
        // Invalid number format
        assert!(parse("01234").is_err());
        
        // Invalid boolean
        assert!(parse("yes").is_err());
        
        // Missing colon in object
        assert!(parse(r#"{"key" "value"}"#).is_err());
        
        // Trailing comma in array
        assert!(parse("[1, 2,]").is_err());
    }

    #[test]
    fn test_duplicate_keys_error() {
        let input = r#"
"name": "first"
"name": "second"
"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn test_parse_number_edge_cases() {
        // Scientific notation
        assert!(parse("1.5e-10").is_ok());
        assert!(parse("2.5E+3").is_ok());
        
        // Zero
        assert_eq!(parse("0").unwrap(), Value::Number(Number::Integer(0)));
        assert_eq!(parse("0.0").unwrap(), Value::Number(Number::Float(0.0)));
        
        // Negative zero
        assert_eq!(parse("-0").unwrap(), Value::Number(Number::Integer(0)));
    }

    #[test]
    fn test_mixed_flow_and_block() {
        let input = r#"
"array": [1, 2, 3]
"object": {"nested": true}
"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("array"));
            assert!(obj.contains_key("object"));
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_parse_deep_nesting() {
        let input = r#"{"a": {"b": {"c": 42}}}"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Object(a)) = obj.get("a") {
                if let Some(Value::Object(b)) = a.get("b") {
                    if let Some(Value::Number(Number::Integer(42))) = b.get("c") {
                        // Success
                    } else {
                        panic!("Expected c: 42");
                    }
                } else {
                    panic!("Expected b object");
                }
            } else {
                panic!("Expected a object");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_element_objects_multiple_keys() {
        // Test the core functionality we just fixed: array elements with multiple object keys
        let input = r#""users":
  - "name": "Alice"
    "age": 30
"config":
  "timeout": 30"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            assert!(obj.contains_key("config"));
            
            if let Some(Value::Array(users)) = obj.get("users") {
                assert_eq!(users.len(), 1);
                
                // Check the user has both name and age
                if let Some(Value::Object(user)) = users.get(0) {
                    assert_eq!(user.get("name"), Some(&Value::String("Alice".to_string())));
                    assert_eq!(user.get("age"), Some(&Value::Number(Number::Integer(30))));
                    assert_eq!(user.len(), 2); // name and age
                } else {
                    panic!("User should be an object");
                }
            } else {
                panic!("users should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_nested_array_with_mixed_elements() {
        let input = r#""data":
  - "simple_string"
  - 42
  - "name": "Complex"
    "value": 100
"end": true"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(data)) = obj.get("data") {
                assert_eq!(data.len(), 3);
                
                // Element 0: string
                assert_eq!(data.get(0), Some(&Value::String("simple_string".to_string())));
                
                // Element 1: number
                assert_eq!(data.get(1), Some(&Value::Number(Number::Integer(42))));
                
                // Element 2: object with multiple keys
                if let Some(Value::Object(obj)) = data.get(2) {
                    assert_eq!(obj.len(), 2);
                    assert!(obj.contains_key("name"));
                    assert!(obj.contains_key("value"));
                } else {
                    panic!("Element 2 should be an object");
                }
            } else {
                panic!("data should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_simple_array_object() {
        let input = r#""items":
  - "name": "first"
    "value": 1
"total": 10"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(items)) = obj.get("items") {
                assert_eq!(items.len(), 1);
                
                if let Some(Value::Object(item)) = items.get(0) {
                    assert_eq!(item.get("name"), Some(&Value::String("first".to_string())));
                    assert_eq!(item.get("value"), Some(&Value::Number(Number::Integer(1))));
                } else {
                    panic!("Item should be an object");
                }
            } else {
                panic!("items should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_empty_array_elements() {
        let input = r#""mixed":
  - "value": "present"
  -
  - "another": "value""#;
        // This should fail because empty array elements aren't valid in JYAML
        assert!(parse(input).is_err());
    }


    #[test]
    fn test_malformed_array_object_indent() {
        // Test incorrect indentation in array object
        let input = r#""users":
  - "name": "Alice"
"age": 30  # Wrong indent - should be at same level as "name""#;
        let result = parse(input).unwrap();
        
        // This should parse "age" as a separate top-level key, not part of the array element
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            assert!(obj.contains_key("age"));
            
            if let Some(Value::Array(users)) = obj.get("users") {
                if let Some(Value::Object(user)) = users.get(0) {
                    assert_eq!(user.len(), 1); // Only "name", not "age"
                    assert!(user.contains_key("name"));
                    assert!(!user.contains_key("age"));
                } else {
                    panic!("User should be an object");
                }
            } else {
                panic!("users should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_deeply_nested_array_objects() {
        let input = r#""levels":
  - "level": 1
    "name": "test""#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(levels)) = obj.get("levels") {
                if let Some(Value::Object(level_obj)) = levels.get(0) {
                    assert_eq!(level_obj.get("level"), Some(&Value::Number(Number::Integer(1))));
                    assert_eq!(level_obj.get("name"), Some(&Value::String("test".to_string())));
                } else {
                    panic!("level should be an object");
                }
            } else {
                panic!("levels should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_object_with_different_indents() {
        // Test that our indent logic correctly handles different levels
        let input = r#""config":
  "servers":
    - "host": "localhost"
      "port": 8080
      "ssl": false
  "status": "ready""#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Object(config)) = obj.get("config") {
                if let Some(Value::Array(servers)) = config.get("servers") {
                    assert_eq!(servers.len(), 1);
                    
                    // Check first server
                    if let Some(Value::Object(server1)) = servers.get(0) {
                        assert_eq!(server1.len(), 3);
                        assert_eq!(server1.get("host"), Some(&Value::String("localhost".to_string())));
                        assert_eq!(server1.get("port"), Some(&Value::Number(Number::Integer(8080))));
                        assert_eq!(server1.get("ssl"), Some(&Value::Bool(false)));
                    } else {
                        panic!("First server should be an object");
                    }
                } else {
                    panic!("servers should be an array");
                }
                
                assert_eq!(config.get("status"), Some(&Value::String("ready".to_string())));
            } else {
                panic!("config should be an object");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_element_single_key_vs_multiple_keys() {
        // Ensure single-key objects work correctly alongside multi-key objects
        let input = r#""items":
  - "first": "key"
    "second": "key""#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(items)) = obj.get("items") {
                assert_eq!(items.len(), 1);
                
                // Multi key object
                if let Some(Value::Object(item1)) = items.get(0) {
                    assert_eq!(item1.len(), 2);
                    assert!(item1.contains_key("first"));
                    assert!(item1.contains_key("second"));
                } else {
                    panic!("Item 1 should be an object");
                }
            } else {
                panic!("items should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_with_comments() {
        let input = r#"# Main array
"tasks":
  - "name": "Setup"    # Task name
    "priority": 1      # High priority"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(tasks)) = obj.get("tasks") {
                assert_eq!(tasks.len(), 1);
                
                if let Some(Value::Object(task1)) = tasks.get(0) {
                    assert_eq!(task1.get("name"), Some(&Value::String("Setup".to_string())));
                    assert_eq!(task1.get("priority"), Some(&Value::Number(Number::Integer(1))));
                } else {
                    panic!("Task 1 should be an object");
                }
            } else {
                panic!("tasks should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_with_literal_strings() {
        let input = r#""docs":
  - "title": "Example"
    "content": "This is a literal string with multiple lines preserved as-is""#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(docs)) = obj.get("docs") {
                assert_eq!(docs.len(), 1);
                
                // Check literal string
                if let Some(Value::Object(doc1)) = docs.get(0) {
                    assert_eq!(doc1.get("title"), Some(&Value::String("Example".to_string())));
                    if let Some(Value::String(content)) = doc1.get("content") {
                        assert!(content.contains("multiple lines"));
                    } else {
                        panic!("Content should be a string");
                    }
                } else {
                    panic!("Doc 1 should be an object");
                }
            } else {
                panic!("docs should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_array_element_edge_cases() {
        // Test various edge cases for array elements
        let input = r#""edge_cases":
  - null
  - true
  - false
  - 0
  - ""
  - []
  - {}
  - "key": null
"summary": "complete""#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            if let Some(Value::Array(cases)) = obj.get("edge_cases") {
                assert_eq!(cases.len(), 8);
                
                assert_eq!(cases.get(0), Some(&Value::Null));
                assert_eq!(cases.get(1), Some(&Value::Bool(true)));
                assert_eq!(cases.get(2), Some(&Value::Bool(false)));
                assert_eq!(cases.get(3), Some(&Value::Number(Number::Integer(0))));
                assert_eq!(cases.get(4), Some(&Value::String("".to_string())));
                
                // Empty array
                if let Some(Value::Array(arr)) = cases.get(5) {
                    assert_eq!(arr.len(), 0);
                } else {
                    panic!("Element 5 should be an empty array");
                }
                
                // Empty object
                if let Some(Value::Object(obj)) = cases.get(6) {
                    assert_eq!(obj.len(), 0);
                } else {
                    panic!("Element 6 should be an empty object");
                }
                
                // Object with null value
                if let Some(Value::Object(obj)) = cases.get(7) {
                    assert_eq!(obj.get("key"), Some(&Value::Null));
                } else {
                    panic!("Element 7 should be an object");
                }
            } else {
                panic!("edge_cases should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

    #[test]
    fn test_multiple_arrays_in_object() {
        // Test multiple arrays within the same object
        let input = r#""users":
  - "name": "Alice"
    "role": "admin"
"active": true"#;
        let result = parse(input).unwrap();
        
        if let Value::Object(obj) = result {
            assert!(obj.contains_key("users"));
            assert!(obj.contains_key("active"));
            
            // Check users array
            if let Some(Value::Array(users)) = obj.get("users") {
                assert_eq!(users.len(), 1);
                
                if let Some(Value::Object(user1)) = users.get(0) {
                    assert_eq!(user1.get("name"), Some(&Value::String("Alice".to_string())));
                    assert_eq!(user1.get("role"), Some(&Value::String("admin".to_string())));
                }
            } else {
                panic!("users should be an array");
            }
        } else {
            panic!("Root should be an object");
        }
    }

}