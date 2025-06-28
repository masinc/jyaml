//! JYAML (JSON-YAML Adaptive Markup Language) parser and serializer for Rust
//!
//! This crate provides a complete implementation of the JYAML specification,
//! with full serde integration for seamless serialization and deserialization.
//!
//! # Quick Start
//!
//! ```
//! use jyaml::{to_string, from_str};
//! use serde::{Serialize, Deserialize};
//! use std::collections::HashMap;
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Config {
//!     name: String,
//!     version: u32,
//!     features: Vec<String>,
//! }
//!
//! let config = Config {
//!     name: "MyApp".to_string(),
//!     version: 1,
//!     features: vec!["logging".to_string(), "metrics".to_string()],
//! };
//!
//! // Serialize to JYAML
//! let jyaml_string = to_string(&config).unwrap();
//! println!("{}", jyaml_string);
//!
//! // Deserialize from JYAML
//! let parsed_config: Config = from_str(&jyaml_string).unwrap();
//! assert_eq!(config, parsed_config);
//! ```
//!
//! # Features
//!
//! - **JSON-compatible**: All valid JSON is valid JYAML
//! - **YAML-inspired**: Supports YAML-style block formatting and comments
//! - **Serde integration**: Works with any type that implements `Serialize`/`Deserialize`
//! - **Flexible options**: Extensive customization for both parsing and serialization
//! - **Unicode support**: Proper handling of Unicode characters and surrogate pairs
//! - **Error handling**: Detailed error messages with line/column information
//!
//! # API Overview
//!
//! ## Basic Functions
//!
//! - [`to_string()`] - Serialize with default options
//! - [`to_string_pretty()`] - Serialize with pretty-printing
//! - [`from_str()`] - Deserialize with default options
//! - [`parse()`] - Parse to [`Value`] enum (low-level)
//!
//! ## Advanced Functions with Options
//!
//! - [`to_string_with_options()`] - Serialize with custom [`SerializeOptions`]
//! - [`from_str_with_options()`] - Deserialize with custom [`DeserializeOptions`]
//! - [`parse_with_options()`] - Parse to [`Value`] with custom options
//!
//! ## Options and Builders
//!
//! - [`SerializeOptions`] and [`SerializeOptionsBuilder`] - Control output formatting
//! - [`DeserializeOptions`] and [`DeserializeOptionsBuilder`] - Control parsing behavior
//! - [`OutputStyle`], [`QuoteStyle`], [`LineEnding`] - Formatting enums
//!
//! # Examples
//!
//! ## Different Output Styles
//!
//! ```
//! use jyaml::{to_string_with_options, SerializeOptions};
//! use std::collections::HashMap;
//!
//! let data = HashMap::from([("name", "Alice"), ("age", "30")]);
//!
//! // Compact (minimal whitespace)
//! let compact = SerializeOptions::compact();
//! let result = to_string_with_options(&data, &compact).unwrap();
//! // Output: {"name": "Alice", "age": "30"}
//!
//! // Pretty (human-readable)
//! let pretty = SerializeOptions::pretty();
//! let result = to_string_with_options(&data, &pretty).unwrap();
//! // Output:
//! //   "name": "Alice"
//! //   "age": "30"
//!
//! // Block style (YAML-like)
//! let block = SerializeOptions::block();
//! let result = to_string_with_options(&data, &block).unwrap();
//! ```
//!
//! ## Custom Parsing Options
//!
//! ```
//! use jyaml::{from_str_with_options, DeserializeOptions};
//! use std::collections::HashMap;
//!
//! let jyaml_content = r#"
//! "name": "Alice"
//! "age": "30"
//! "#;
//!
//! // Permissive parsing with comments preserved
//! let options = DeserializeOptions::permissive();
//! let result: Result<HashMap<String, String>, _> = from_str_with_options(jyaml_content, &options);
//! assert!(result.is_ok());
//! ```
//!
//! ## Builder Pattern
//!
//! ```
//! use jyaml::{SerializeOptions, OutputStyle, QuoteStyle};
//!
//! let options = SerializeOptions::builder()
//!     .style(OutputStyle::Block)
//!     .indent(4)
//!     .sort_keys(true)
//!     .escape_unicode(false)
//!     .pretty(true)
//!     .build();
//! ```

pub mod de;
pub mod error;
pub mod options;
pub mod ser;
pub mod value;

mod lexer;
mod parser;
mod test_multiline;

// Core deserialization functions and types
pub use de::{from_str, from_str_with_options, Deserializer};

// Error types
pub use error::{Error, Result};

// Configuration options and builders
pub use options::{
    DeserializeOptions, DeserializeOptionsBuilder, 
    SerializeOptions, SerializeOptionsBuilder,
    OutputStyle, QuoteStyle, LineEnding
};

// Low-level parsing functions
pub use parser::{parse, parse_with_options};

// Core serialization functions and types
pub use ser::{to_string, to_string_pretty, to_string_with_options, Serializer};

// Value enum for untyped data
pub use value::Value;

// Re-export for convenience
pub use serde::{Deserialize, Serialize};