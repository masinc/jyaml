use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Syntax error at line {line}, column {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Unexpected token '{found}' at line {line}, column {column}, expected {expected}")]
    UnexpectedToken {
        line: usize,
        column: usize,
        found: String,
        expected: String,
    },

    #[error("Invalid escape sequence '\\{sequence}' at line {line}, column {column}")]
    InvalidEscape {
        line: usize,
        column: usize,
        sequence: char,
    },

    #[error("Inconsistent indentation at line {line}: expected {expected} spaces, found {found}")]
    InconsistentIndentation {
        line: usize,
        expected: usize,
        found: usize,
    },

    #[error("Tab character in indentation at line {line}, column {column}")]
    TabInIndentation { line: usize, column: usize },

    #[error("Invalid number format '{value}' at line {line}, column {column}")]
    InvalidNumber {
        line: usize,
        column: usize,
        value: String,
    },

    #[error("Duplicate key '{key}' at line {line}, column {column}")]
    DuplicateKey {
        line: usize,
        column: usize,
        key: String,
    },

    #[error("Block style not allowed in flow context at line {line}, column {column}")]
    BlockInFlow { line: usize, column: usize },

    #[error("Invalid UTF-8 sequence at byte {position}")]
    InvalidUtf8 { position: usize },

    #[error("BOM (Byte Order Mark) not allowed at beginning of file")]
    BomNotAllowed,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Invalid options: {0}")]
    InvalidOptions(String),
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Deserialization(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Serialization(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::Error as DeError;
    use std::error::Error as StdError;

    #[test]
    fn test_error_display() {
        // Test SyntaxError display
        let error = Error::SyntaxError {
            line: 5,
            column: 10,
            message: "Invalid character".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("line 5"));
        assert!(display.contains("column 10"));
        assert!(display.contains("Invalid character"));

        // Test UnexpectedToken display
        let error = Error::UnexpectedToken {
            line: 2,
            column: 3,
            found: "{".to_string(),
            expected: "string".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Unexpected token '{'"));
        assert!(display.contains("expected string"));

        // Test InvalidEscape display
        let error = Error::InvalidEscape {
            line: 1,
            column: 5,
            sequence: 'x',
        };
        let display = format!("{}", error);
        assert!(display.contains("Invalid escape sequence '\\x'"));

        // Test InconsistentIndentation display
        let error = Error::InconsistentIndentation {
            line: 3,
            expected: 4,
            found: 2,
        };
        let display = format!("{}", error);
        assert!(display.contains("Inconsistent indentation"));
        assert!(display.contains("expected 4 spaces"));
        assert!(display.contains("found 2"));

        // Test TabInIndentation display
        let error = Error::TabInIndentation { line: 1, column: 1 };
        let display = format!("{}", error);
        assert!(display.contains("Tab character in indentation"));

        // Test InvalidNumber display
        let error = Error::InvalidNumber {
            line: 2,
            column: 5,
            value: "01234".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Invalid number format '01234'"));

        // Test DuplicateKey display
        let error = Error::DuplicateKey {
            line: 4,
            column: 1,
            key: "name".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Duplicate key 'name'"));

        // Test BlockInFlow display
        let error = Error::BlockInFlow {
            line: 1,
            column: 10,
        };
        let display = format!("{}", error);
        assert!(display.contains("Block style not allowed in flow context"));

        // Test InvalidUtf8 display
        let error = Error::InvalidUtf8 { position: 15 };
        let display = format!("{}", error);
        assert!(display.contains("Invalid UTF-8 sequence"));
        assert!(display.contains("byte 15"));

        // Test BomNotAllowed display
        let error = Error::BomNotAllowed;
        let display = format!("{}", error);
        assert!(display.contains("BOM (Byte Order Mark) not allowed"));

        // Test Serialization display
        let error = Error::Serialization("Test serialization error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Serialization error"));
        assert!(display.contains("Test serialization error"));

        // Test Deserialization display
        let error = Error::Deserialization("Test deserialization error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Deserialization error"));
        assert!(display.contains("Test deserialization error"));
    }

    #[test]
    fn test_error_debug() {
        let error = Error::SyntaxError {
            line: 1,
            column: 1,
            message: "Test".to_string(),
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("SyntaxError"));
        assert!(debug.contains("line: 1"));
        assert!(debug.contains("column: 1"));
        assert!(debug.contains("message: \"Test\""));
    }

    #[test]
    fn test_error_source() {
        // Test that IO error preserves source
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = Error::Io(io_error);
        assert!(StdError::source(&error).is_some());

        // Test that format error preserves source
        let fmt_error = std::fmt::Error;
        let error = Error::Fmt(fmt_error);
        assert!(StdError::source(&error).is_some());

        // Test that custom errors don't have source
        let error = Error::SyntaxError {
            line: 1,
            column: 1,
            message: "Test".to_string(),
        };
        assert!(StdError::source(&error).is_none());
    }

    #[test]
    fn test_serde_error_integration() {
        // Test serde::de::Error implementation
        let custom_error = DeError::custom("Custom deserialization error");
        if let Error::Deserialization(msg) = custom_error {
            assert_eq!(msg, "Custom deserialization error");
        } else {
            panic!("Expected Deserialization error");
        }

        // Test serde::ser::Error implementation
        let custom_error = <Error as serde::ser::Error>::custom("Custom serialization error");
        if let Error::Serialization(msg) = custom_error {
            assert_eq!(msg, "Custom serialization error");
        } else {
            panic!("Expected Serialization error");
        }
    }

    #[test]
    fn test_error_conversion() {
        // Test From<std::io::Error>
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io(_)));

        // Test From<std::fmt::Error>
        let fmt_error = std::fmt::Error;
        let error: Error = fmt_error.into();
        assert!(matches!(error, Error::Fmt(_)));
    }

    #[test]
    fn test_result_type() {
        // Test that Result<T> is an alias for std::result::Result<T, Error>
        let success: Result<i32> = Ok(42);
        assert_eq!(success.unwrap(), 42);

        let failure: Result<i32> = Err(Error::BomNotAllowed);
        assert!(failure.is_err());
    }

    #[test]
    fn test_error_categories() {
        // Test that errors can be categorized
        let syntax_errors = vec![
            Error::SyntaxError {
                line: 1,
                column: 1,
                message: "Test".to_string(),
            },
            Error::UnexpectedToken {
                line: 1,
                column: 1,
                found: "found".to_string(),
                expected: "expected".to_string(),
            },
            Error::InvalidEscape {
                line: 1,
                column: 1,
                sequence: 'x',
            },
        ];

        for error in syntax_errors {
            let display = format!("{}", error);
            // All syntax errors should mention line and column
            assert!(display.contains("line"));
            assert!(display.contains("column"));
        }

        let structure_errors = vec![
            Error::InconsistentIndentation {
                line: 1,
                expected: 2,
                found: 4,
            },
            Error::TabInIndentation { line: 1, column: 1 },
            Error::BlockInFlow { line: 1, column: 1 },
            Error::DuplicateKey {
                line: 1,
                column: 1,
                key: "test".to_string(),
            },
        ];

        for error in structure_errors {
            let display = format!("{}", error);
            // All structure errors should mention line
            assert!(display.contains("line"));
        }
    }
}
