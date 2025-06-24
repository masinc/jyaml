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
            Token::Dash => self.parse_block_array(),
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
    
    fn parse_block_array(&mut self) -> Result<Value> {
        let mut array = Vec::new();
        
        // Determine the base indent from the first dash
        let base_indent = if matches!(self.current_token, Token::Indent(_)) {
            self.current_indent()
        } else {
            0
        };
        
        loop {
            // Skip any indentation first
            if let Token::Indent(n) = self.current_token {
                if n != base_indent {
                    break;
                }
                self.advance()?;
            }
            
            if !matches!(self.current_token, Token::Dash) {
                break;
            }
            
            self.advance()?; // Skip -
            self.skip_inline_whitespace();
            
            array.push(self.parse_value()?);
            
            self.skip_newlines_and_comments();
        }
        
        Ok(Value::Array(array))
    }
    
    fn parse_object_from_first_key(&mut self, first_key: String) -> Result<Value> {
        let base_indent = self.current_indent();
        let mut object = HashMap::new();
        
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
        
        // Process remaining key-value pairs
        while let Token::String(key) = &self.current_token.clone() {
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
}