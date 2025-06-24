//! JYAML (JSON-YAML Adaptive Markup Language) parser and serializer for Rust
//!
//! This crate provides a complete implementation of the JYAML specification,
//! with full serde integration for seamless serialization and deserialization.

pub mod de;
pub mod error;
pub mod ser;
pub mod value;

mod lexer;
mod parser;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use parser::parse;
pub use ser::{to_string, to_string_pretty, Serializer};
pub use value::Value;

// Re-export for convenience
pub use serde::{Deserialize, Serialize};