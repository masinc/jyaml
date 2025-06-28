//! JYAML parsing and serialization options
//!
//! This module provides comprehensive options for configuring JYAML parsing and serialization.
//! The options system includes:
//!
//! - **Preset configurations** for common use cases
//! - **Builder patterns** for convenient option construction
//! - **Validation** to ensure option consistency
//! - **Serialization support** for saving/loading configurations
//!
//! # Basic Usage
//!
//! ```
//! use jyaml::{from_str_with_options, to_string_with_options, DeserializeOptions, SerializeOptions};
//! use std::collections::HashMap;
//!
//! // Use presets for common configurations
//! let permissive = DeserializeOptions::permissive();
//! let pretty = SerializeOptions::pretty();
//!
//! // Or use builders for custom configurations
//! let custom_parse = DeserializeOptions::builder()
//!     .strict_mode(false)
//!     .max_depth(500)
//!     .allow_duplicate_keys(true)
//!     .build();
//!
//! let custom_serialize = SerializeOptions::builder()
//!     .pretty(true)
//!     .indent(4)
//!     .sort_keys(true)
//!     .build();
//! ```
//!
//! # Available Presets
//!
//! ## DeserializeOptions Presets
//! - `strict()` - Strict JYAML spec compliance
//! - `permissive()` - Lenient parsing allowing duplicates
//! - `fast()` - Fast parsing with minimal features
//! - `debug()` - Debug mode with maximum information
//!
//! ## SerializeOptions Presets
//! - `compact()` - Minimal whitespace for storage/transmission
//! - `pretty()` - Human-readable with proper formatting
//! - `block()` - YAML-style block format
//! - `json_compatible()` - JSON-compatible output
//! - `debug()` - Debug-friendly output with verbose formatting

use crate::{Result, Error};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Output style for JYAML serialization
/// 
/// Controls how JYAML data is formatted when serialized to text.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{to_string_with_options, SerializeOptions, OutputStyle};
/// use std::collections::HashMap;
/// 
/// let data = HashMap::from([
///     ("name", "Alice"),
///     ("age", "30"),
/// ]);
/// 
/// // Flow style: compact JSON-like
/// let flow_opts = SerializeOptions::builder().style(OutputStyle::Flow).build();
/// let flow_result = to_string_with_options(&data, &flow_opts).unwrap();
/// // Output: {"name": "Alice", "age": "30"}
/// 
/// // Block style: YAML-like with indentation
/// let block_opts = SerializeOptions::builder().style(OutputStyle::Block).pretty(true).build();
/// let block_result = to_string_with_options(&data, &block_opts).unwrap();
/// // Output:
/// // "name": "Alice"
/// // "age": "30"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputStyle {
    /// Compact JSON-like format with minimal whitespace
    /// 
    /// Uses `{}` for objects and `[]` for arrays. Best for network transmission
    /// and storage where size matters.
    Flow,
    /// YAML-like block format with proper indentation
    /// 
    /// Uses indentation and newlines for structure. More readable for humans
    /// and better for configuration files.
    Block,
    /// Automatically choose based on content complexity
    /// 
    /// Uses Flow for simple/small structures and Block for complex/large ones.
    /// Provides a good balance between readability and compactness.
    Auto,
}

impl Default for OutputStyle {
    fn default() -> Self {
        OutputStyle::Auto
    }
}

/// Quote style for string values
/// 
/// Controls how string values are quoted in JYAML output.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{to_string_with_options, SerializeOptions, QuoteStyle};
/// 
/// let text = "Hello World";
/// 
/// // Double quotes (default)
/// let double_opts = SerializeOptions::builder().quote_style(QuoteStyle::Double).build();
/// let result = to_string_with_options(&text, &double_opts).unwrap();
/// assert_eq!(result, "\"Hello World\"");
/// 
/// // Single quotes (Note: actual implementation may vary)
/// let single_opts = SerializeOptions::builder().quote_style(QuoteStyle::Single).build();
/// // Would output: 'Hello World' (if implemented)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QuoteStyle {
    /// Always use double quotes (`"`)
    /// 
    /// Standard JSON-compatible quoting. Recommended for interoperability.
    Double,
    /// Always use single quotes (`'`)
    /// 
    /// YAML-style quoting. May not be supported by all parsers.
    /// Note: Current implementation uses double quotes regardless.
    Single,
    /// Choose automatically based on content
    /// 
    /// Uses the most appropriate quote style based on the string content
    /// (e.g., double quotes if string contains single quotes).
    Auto,
}

impl Default for QuoteStyle {
    fn default() -> Self {
        QuoteStyle::Double
    }
}

/// Line ending style for serialized output
/// 
/// Controls how line endings are normalized in JYAML output.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{to_string_with_options, SerializeOptions, LineEnding};
/// use std::collections::HashMap;
/// 
/// let data = HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// 
/// // No normalization (preserve original)
/// let none_opts = SerializeOptions::builder()
///     .line_ending(LineEnding::None)
///     .pretty(true)
///     .build();
/// 
/// // Unix-style LF
/// let lf_opts = SerializeOptions::builder()
///     .line_ending(LineEnding::Lf)
///     .pretty(true)
///     .build();
/// 
/// // Windows-style CRLF
/// let crlf_opts = SerializeOptions::builder()
///     .line_ending(LineEnding::Crlf)
///     .pretty(true)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LineEnding {
    /// No normalization - preserve original line endings
    /// 
    /// This is the safest option as it doesn't modify the data unexpectedly.
    /// Recommended for most use cases.
    None,
    /// Unix-style LF (`\n`)
    /// 
    /// Standard on Unix, Linux, and macOS systems.
    /// Also used by most web protocols and development tools.
    Lf,
    /// Windows-style CRLF (`\r\n`)
    /// 
    /// Standard on Windows systems. Use when targeting Windows environments
    /// or when required by specific protocols.
    Crlf,
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::None
    }
}

/// Options for deserializing JYAML from text
/// 
/// Controls how JYAML text is parsed and converted to Rust values.
/// Use [`DeserializeOptionsBuilder`] for a convenient way to create options.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{from_str_with_options, DeserializeOptions};
/// use std::collections::HashMap;
/// 
/// let jyaml_text = r#"
/// "name": "Alice"
/// "name": "Bob"  # Duplicate key
/// "age": 30
/// "#;
/// 
/// // Strict mode (default) - rejects duplicate keys
/// let strict = DeserializeOptions::strict();
/// let result: Result<HashMap<String, serde_json::Value>, _> = from_str_with_options(jyaml_text, &strict);
/// assert!(result.is_err());
/// 
/// // Permissive mode - allows duplicate keys (last value wins)
/// let permissive = DeserializeOptions::permissive();
/// let result: Result<HashMap<String, serde_json::Value>, _> = from_str_with_options(jyaml_text, &permissive);
/// assert!(result.is_ok());
/// ```
/// 
/// # Available Presets
/// 
/// - [`strict()`](DeserializeOptions::strict) - Strict parsing with error on duplicates
/// - [`permissive()`](DeserializeOptions::permissive) - Lenient parsing allowing duplicates
/// - [`fast()`](DeserializeOptions::fast) - Fast parsing with minimal features
/// - [`debug()`](DeserializeOptions::debug) - Debug mode with maximum information
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeserializeOptions {
    /// Enable strict mode parsing
    /// 
    /// When true, parsing is more restrictive and follows the specification closely.
    /// When false, allows more lenient parsing that may accept invalid input.
    pub strict_mode: bool,
    
    /// Maximum parsing depth to prevent stack overflow
    /// 
    /// Limits how deeply nested structures can be. Prevents infinite recursion
    /// and stack overflow on maliciously crafted input. Must be at least 1.
    pub max_depth: usize,
    
    /// Allow duplicate keys in objects (last one wins)
    /// 
    /// When true, duplicate keys are allowed and the last value overwrites previous ones.
    /// When false, duplicate keys cause a parsing error.
    pub allow_duplicate_keys: bool,
    
    /// Preserve comments in parsed output
    /// 
    /// When true, comments are preserved during parsing (implementation-dependent).
    /// When false, comments are ignored and discarded.
    /// Note: Current implementation doesn't store comments in the Value structure.
    pub preserve_comments: bool,
    
    /// Include comment positions in output
    /// 
    /// When true, the positions of comments are tracked and made available.
    /// Requires `preserve_comments` to be true to have any effect.
    pub include_comment_positions: bool,
    
    /// Normalize line endings during parsing
    /// 
    /// Controls how line endings in string values are normalized.
    /// See [`LineEnding`] for details.
    pub normalize_line_endings: LineEnding,
}

/// Builder for creating [`DeserializeOptions`] with a fluent API
/// 
/// Provides a convenient way to construct `DeserializeOptions` with method chaining.
/// All methods return `self` to allow chaining, and the final `build()` or `try_build()`
/// creates the actual options struct.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{DeserializeOptionsBuilder, DeserializeOptions};
/// 
/// // Basic usage
/// let options = DeserializeOptionsBuilder::new()
///     .strict_mode(false)
///     .allow_duplicate_keys(true)
///     .max_depth(100)
///     .build();
/// 
/// // With validation
/// let options = DeserializeOptions::builder()
///     .max_depth(50)
///     .preserve_comments(true)
///     .try_build()
///     .expect("Valid options");
/// ```
#[derive(Debug, Clone)]
pub struct DeserializeOptionsBuilder {
    strict_mode: bool,
    max_depth: usize,
    allow_duplicate_keys: bool,
    preserve_comments: bool,
    include_comment_positions: bool,
    normalize_line_endings: LineEnding,
}

impl Default for DeserializeOptions {
    fn default() -> Self {
        Self {
            strict_mode: true,
            max_depth: 1000,
            allow_duplicate_keys: false,
            preserve_comments: true,
            include_comment_positions: false,
            normalize_line_endings: LineEnding::None,
        }
    }
}

/// Options for serializing Rust values to JYAML text
/// 
/// Controls the format and style of the generated JYAML output.
/// Use [`SerializeOptionsBuilder`] for a convenient way to create options.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{to_string_with_options, SerializeOptions, OutputStyle};
/// use std::collections::HashMap;
/// 
/// let data = HashMap::from([
///     ("name", "Alice"),
///     ("age", "30"),
///     ("city", "Tokyo"),
/// ]);
/// 
/// // Compact output
/// let compact = SerializeOptions::compact();
/// let result = to_string_with_options(&data, &compact).unwrap();
/// // Output: {"name": "Alice", "age": "30", "city": "Tokyo"}
/// 
/// // Pretty-printed output
/// let pretty = SerializeOptions::pretty();
/// let result = to_string_with_options(&data, &pretty).unwrap();
/// // Output:
/// //   "name": "Alice"
/// //   "age": "30"
/// //   "city": "Tokyo"
/// 
/// // Custom options with builder
/// let custom = SerializeOptions::builder()
///     .style(OutputStyle::Block)
///     .indent(4)
///     .sort_keys(true)
///     .pretty(true)
///     .build();
/// ```
/// 
/// # Available Presets
/// 
/// - [`compact()`](SerializeOptions::compact) - Minimal whitespace for storage/transmission
/// - [`pretty()`](SerializeOptions::pretty) - Human-readable with proper formatting
/// - [`block()`](SerializeOptions::block) - YAML-style block format
/// - [`json_compatible()`](SerializeOptions::json_compatible) - JSON-compatible output
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SerializeOptions {
    /// Output style (flow, block, or auto)
    /// 
    /// Determines the overall formatting approach. See [`OutputStyle`] for details.
    pub style: OutputStyle,
    
    /// Number of spaces for indentation (0-8)
    /// 
    /// Used when `pretty` is true or `style` is `Block`. Values greater than 8
    /// will be clamped to 8 in `build()` or cause an error in `try_build()`.
    pub indent: usize,
    
    /// Quote style for strings
    /// 
    /// Controls whether strings use double quotes, single quotes, or auto-selection.
    /// Note: Current implementation always uses double quotes.
    pub quote_style: QuoteStyle,
    
    /// Escape non-ASCII Unicode characters
    /// 
    /// When true, characters outside the ASCII range are escaped as `\uXXXX`.
    /// When false, Unicode characters are output directly (UTF-8).
    pub escape_unicode: bool,
    
    /// Sort object keys alphabetically
    /// 
    /// When true, object/map keys are sorted before serialization.
    /// Useful for consistent output and diffs.
    pub sort_keys: bool,
    
    /// Line ending style
    /// 
    /// Controls how line endings are normalized in the output.
    /// See [`LineEnding`] for details.
    pub line_ending: LineEnding,
    
    /// Enable pretty printing
    /// 
    /// When true, adds proper indentation and newlines for readability.
    /// When false, uses minimal whitespace.
    pub pretty: bool,
}

/// Builder for creating [`SerializeOptions`] with a fluent API
/// 
/// Provides a convenient way to construct `SerializeOptions` with method chaining.
/// All methods return `self` to allow chaining, and the final `build()` or `try_build()`
/// creates the actual options struct.
/// 
/// # Examples
/// 
/// ```
/// use jyaml::{SerializeOptionsBuilder, SerializeOptions, OutputStyle, QuoteStyle};
/// 
/// // Basic usage
/// let options = SerializeOptionsBuilder::new()
///     .style(OutputStyle::Block)
///     .indent(4)
///     .pretty(true)
///     .build();
/// 
/// // With validation
/// let options = SerializeOptions::builder()
///     .indent(2)
///     .sort_keys(true)
///     .escape_unicode(false)
///     .try_build()
///     .expect("Valid options");
/// 
/// // Chaining many options
/// let custom = SerializeOptionsBuilder::new()
///     .style(OutputStyle::Auto)
///     .quote_style(QuoteStyle::Double)
///     .escape_unicode(true)
///     .sort_keys(true)
///     .pretty(true)
///     .indent(2)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct SerializeOptionsBuilder {
    style: OutputStyle,
    indent: usize,
    quote_style: QuoteStyle,
    escape_unicode: bool,
    sort_keys: bool,
    line_ending: LineEnding,
    pretty: bool,
}

impl Default for SerializeOptions {
    fn default() -> Self {
        Self {
            style: OutputStyle::Auto,
            indent: 2,
            quote_style: QuoteStyle::Double,
            escape_unicode: false,
            sort_keys: false,
            line_ending: LineEnding::None,
            pretty: false,
        }
    }
}

impl SerializeOptions {
    /// Create a new builder for SerializeOptions
    pub fn builder() -> SerializeOptionsBuilder {
        SerializeOptionsBuilder::new()
    }
    
    /// Create options from a preset name
    /// 
    /// Available presets:
    /// - "compact" - Minimal whitespace for storage/transmission
    /// - "pretty" - Human-readable with proper formatting
    /// - "block" - YAML-style block format
    /// - "json_compatible" - JSON-compatible output
    /// - "debug" - Debug-friendly output with verbose formatting
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptions;
    /// 
    /// let opts = SerializeOptions::from_preset("debug").unwrap();
    /// assert!(opts.pretty);
    /// assert!(opts.sort_keys);
    /// assert_eq!(opts.indent, 4);
    /// ```
    pub fn from_preset(preset: &str) -> Result<Self> {
        SerializeOptionsBuilder::from_preset(preset).map(|b| b.build())
    }
    
    /// Create options for compact output
    pub fn compact() -> Self {
        Self::builder()
            .style(OutputStyle::Flow)
            .pretty(false)
            .indent(0)
            .build()
    }
    
    /// Create options for pretty formatted output
    pub fn pretty() -> Self {
        Self::builder()
            .style(OutputStyle::Auto)
            .pretty(true)
            .indent(2)
            .build()
    }
    
    /// Create options for YAML-like block format
    pub fn block() -> Self {
        Self::builder()
            .style(OutputStyle::Block)
            .pretty(true)
            .indent(2)
            .build()
    }
    
    /// Create options for JSON-compatible output
    pub fn json_compatible() -> Self {
        Self::builder()
            .style(OutputStyle::Flow)
            .quote_style(QuoteStyle::Double)
            .escape_unicode(true)
            .sort_keys(false)
            .pretty(false)
            .indent(0)
            .build()
    }
    
    /// Create options for debug output
    pub fn debug() -> Self {
        Self::builder()
            .style(OutputStyle::Block)
            .pretty(true)
            .indent(4)
            .sort_keys(true)
            .build()
    }
    
    /// Set the output style
    pub fn with_style(mut self, style: OutputStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the indentation
    pub fn with_indent(mut self, indent: usize) -> Result<Self> {
        if indent > 8 {
            return Err(Error::InvalidOptions("Indent must be 0-8 spaces".to_string()));
        }
        self.indent = indent;
        Ok(self)
    }
    
    /// Set the quote style
    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }
    
    /// Enable or disable Unicode escaping
    pub fn with_escape_unicode(mut self, escape: bool) -> Self {
        self.escape_unicode = escape;
        self
    }
    
    /// Enable or disable key sorting
    pub fn with_sort_keys(mut self, sort: bool) -> Self {
        self.sort_keys = sort;
        self
    }
    
    /// Set line ending style
    pub fn with_line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }
    
    
    /// Enable pretty printing
    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        if pretty && self.indent == 0 {
            self.indent = 2;
        }
        self
    }
    
    /// Validate options for consistency
    /// 
    /// Checks for incompatible option combinations and returns an error
    /// if any are found.
    /// 
    /// # Errors
    /// 
    /// - `InvalidOptions` if indent > 8
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptions;
    /// 
    /// let valid_opts = SerializeOptions::default();
    /// assert!(valid_opts.validate().is_ok());
    /// 
    /// let invalid_opts = SerializeOptions {
    ///     indent: 10,
    ///     ..Default::default()
    /// };
    /// assert!(invalid_opts.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.indent > 8 {
            return Err(Error::InvalidOptions("Indent must be 0-8 spaces".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if this configuration produces compact output
    pub fn is_compact(&self) -> bool {
        !self.pretty && self.style == OutputStyle::Flow
    }
    
    /// Check if this configuration produces human-readable output
    pub fn is_readable(&self) -> bool {
        self.pretty && (self.style == OutputStyle::Block || self.style == OutputStyle::Auto)
    }
    
    /// Get effective line ending (resolves None to platform default)
    pub fn effective_line_ending(&self) -> LineEnding {
        match self.line_ending {
            LineEnding::None => {
                // Default to platform-appropriate line ending
                #[cfg(windows)]
                return LineEnding::Crlf;
                #[cfg(not(windows))]
                return LineEnding::Lf;
            }
            other => other,
        }
    }
}

impl SerializeOptionsBuilder {
    /// Create a new builder with default values
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let builder = SerializeOptionsBuilder::new();
    /// let options = builder.build();
    /// ```
    pub fn new() -> Self {
        Self {
            style: OutputStyle::Auto,
            indent: 2,
            quote_style: QuoteStyle::Double,
            escape_unicode: false,
            sort_keys: false,
            line_ending: LineEnding::None,
            pretty: false,
        }
    }
    
    /// Create builder from a preset configuration
    /// 
    /// Available presets:
    /// - "compact" - Minimal whitespace for storage/transmission
    /// - "pretty" - Human-readable with proper formatting
    /// - "block" - YAML-style block format
    /// - "json_compatible" - JSON-compatible output
    /// - "debug" - Debug-friendly output with verbose formatting
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let pretty_builder = SerializeOptionsBuilder::from_preset("pretty");
    /// let options = pretty_builder.build();
    /// ```
    pub fn from_preset(preset: &str) -> Result<Self> {
        match preset {
            "compact" => Ok(Self::new()
                .style(OutputStyle::Flow)
                .pretty(false)
                .indent(0)),
            "pretty" => Ok(Self::new()
                .style(OutputStyle::Auto)
                .pretty(true)
                .indent(2)),
            "block" => Ok(Self::new()
                .style(OutputStyle::Block)
                .pretty(true)
                .indent(2)),
            "json_compatible" => Ok(Self::new()
                .style(OutputStyle::Flow)
                .quote_style(QuoteStyle::Double)
                .escape_unicode(true)
                .sort_keys(false)
                .pretty(false)
                .indent(0)),
            "debug" => Ok(Self::new()
                .style(OutputStyle::Block)
                .pretty(true)
                .indent(4)
                .sort_keys(true)),
            _ => Err(Error::InvalidOptions(
                format!("Unknown preset: {}. Available: compact, pretty, block, json_compatible, debug", preset)
            ))
        }
    }
    
    /// Set the output style
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::{SerializeOptionsBuilder, OutputStyle};
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .style(OutputStyle::Block)
    ///     .build();
    /// ```
    pub fn style(mut self, style: OutputStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Set the indentation (0-8 spaces)
    /// 
    /// Validation occurs during `build()` or `try_build()`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .indent(4)
    ///     .build();
    /// ```
    pub fn indent(mut self, indent: usize) -> Self {
        // Note: Validation happens during build()
        self.indent = indent;
        self
    }
    
    /// Set the quote style for strings
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::{SerializeOptionsBuilder, QuoteStyle};
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .quote_style(QuoteStyle::Single)
    ///     .build();
    /// ```
    pub fn quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }
    
    /// Enable or disable Unicode escaping
    /// 
    /// When enabled, non-ASCII Unicode characters are escaped as `\uXXXX`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .escape_unicode(true)
    ///     .build();
    /// ```
    pub fn escape_unicode(mut self, escape: bool) -> Self {
        self.escape_unicode = escape;
        self
    }
    
    /// Enable or disable key sorting
    /// 
    /// When enabled, object keys are sorted alphabetically.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .sort_keys(true)
    ///     .build();
    /// ```
    pub fn sort_keys(mut self, sort: bool) -> Self {
        self.sort_keys = sort;
        self
    }
    
    /// Set line ending style
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::{SerializeOptionsBuilder, LineEnding};
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .line_ending(LineEnding::Lf)
    ///     .build();
    /// ```
    pub fn line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }
    
    
    /// Enable or disable pretty printing
    /// 
    /// When enabled, adds proper indentation and newlines for readability.
    /// If enabled and indent is 0, it will be automatically set to 2.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .pretty(true)
    ///     .build();
    /// ```
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        if pretty && self.indent == 0 {
            self.indent = 2;
        }
        self
    }
    
    /// Build the SerializeOptions, performing validation
    /// 
    /// This method clamps invalid values to valid ranges (e.g., indent > 8 becomes 8).
    /// For strict validation with errors, use `try_build()` instead.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::{SerializeOptionsBuilder, OutputStyle};
    /// 
    /// let options = SerializeOptionsBuilder::new()
    ///     .style(OutputStyle::Block)
    ///     .indent(4)
    ///     .pretty(true)
    ///     .build();
    /// ```
    pub fn build(self) -> SerializeOptions {
        SerializeOptions {
            style: self.style,
            indent: self.indent.min(8), // Clamp to valid range
            quote_style: self.quote_style,
            escape_unicode: self.escape_unicode,
            sort_keys: self.sort_keys,
            line_ending: self.line_ending,
            pretty: self.pretty,
        }
    }
    
    /// Build the SerializeOptions with strict validation
    /// 
    /// Returns an error if any values are out of valid ranges.
    /// 
    /// # Errors
    /// 
    /// - `InvalidOptions` if indent > 8
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::SerializeOptionsBuilder;
    /// 
    /// let result = SerializeOptionsBuilder::new()
    ///     .indent(4)
    ///     .try_build();
    /// assert!(result.is_ok());
    /// 
    /// let result = SerializeOptionsBuilder::new()
    ///     .indent(10)  // Invalid
    ///     .try_build();
    /// assert!(result.is_err());
    /// ```
    pub fn try_build(self) -> Result<SerializeOptions> {
        if self.indent > 8 {
            return Err(Error::InvalidOptions("Indent must be 0-8 spaces".to_string()));
        }
        
        
        Ok(SerializeOptions {
            style: self.style,
            indent: self.indent,
            quote_style: self.quote_style,
            escape_unicode: self.escape_unicode,
            sort_keys: self.sort_keys,
            line_ending: self.line_ending,
            pretty: self.pretty,
        })
    }
}

impl Default for SerializeOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DeserializeOptions {
    /// Create a new builder for DeserializeOptions
    pub fn builder() -> DeserializeOptionsBuilder {
        DeserializeOptionsBuilder::new()
    }
    
    /// Create options from a preset name
    /// 
    /// Available presets:
    /// - "strict" - Strict JYAML spec compliance (default)
    /// - "permissive" - Lenient parsing allowing duplicates  
    /// - "fast" - Fast parsing with minimal features
    /// - "debug" - Debug mode with maximum information
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptions;
    /// 
    /// let opts = DeserializeOptions::from_preset("debug").unwrap();
    /// assert!(!opts.strict_mode);
    /// assert!(opts.include_comment_positions);
    /// ```
    pub fn from_preset(preset: &str) -> Result<Self> {
        DeserializeOptionsBuilder::from_preset(preset).map(|b| b.build())
    }
    
    /// Create strict parsing options
    pub fn strict() -> Self {
        Self::builder()
            .strict_mode(true)
            .preserve_comments(true)
            .max_depth(1000)
            .build()
    }
    
    /// Create permissive parsing options
    pub fn permissive() -> Self {
        Self::builder()
            .strict_mode(false)
            .preserve_comments(true)
            .allow_duplicate_keys(true)
            .max_depth(10000)
            .build()
    }
    
    /// Create fast parsing options (minimal features)
    pub fn fast() -> Self {
        Self::builder()
            .strict_mode(true)
            .preserve_comments(false)
            .max_depth(100)
            .build()
    }
    
    /// Create debug parsing options
    pub fn debug() -> Self {
        Self::builder()
            .strict_mode(false)
            .preserve_comments(true)
            .include_comment_positions(true)
            .allow_duplicate_keys(true)
            .build()
    }
    
    /// Set strict mode
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    /// Set maximum parsing depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    
    /// Allow or disallow duplicate keys
    pub fn with_allow_duplicate_keys(mut self, allow: bool) -> Self {
        self.allow_duplicate_keys = allow;
        self
    }
    
    /// Enable or disable comment preservation
    pub fn with_preserve_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }
    
    /// Enable or disable comment position tracking
    pub fn with_include_comment_positions(mut self, include: bool) -> Self {
        self.include_comment_positions = include;
        self
    }
    
    /// Set line ending normalization
    pub fn with_normalize_line_endings(mut self, ending: LineEnding) -> Self {
        self.normalize_line_endings = ending;
        self
    }
    
    /// Validate options for consistency
    /// 
    /// Checks for incompatible option combinations and returns an error
    /// if any are found.
    /// 
    /// # Errors
    /// 
    /// - `InvalidOptions` if strict_mode and allow_duplicate_keys are both true
    /// - `InvalidOptions` if include_comment_positions is true but preserve_comments is false
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptions;
    /// 
    /// let valid_opts = DeserializeOptions::default();
    /// assert!(valid_opts.validate().is_ok());
    /// 
    /// let invalid_opts = DeserializeOptions {
    ///     strict_mode: true,
    ///     allow_duplicate_keys: true,
    ///     ..Default::default()
    /// };
    /// assert!(invalid_opts.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.strict_mode && self.allow_duplicate_keys {
            return Err(Error::InvalidOptions(
                "strict_mode and allow_duplicate_keys are incompatible".to_string()
            ));
        }
        
        if self.include_comment_positions && !self.preserve_comments {
            return Err(Error::InvalidOptions(
                "include_comment_positions requires preserve_comments=true".to_string()
            ));
        }
        
        if self.max_depth == 0 {
            return Err(Error::InvalidOptions("Max depth must be at least 1".to_string()));
        }
        
        if self.max_depth > 100000 {
            return Err(Error::InvalidOptions("Max depth too large (max 100000)".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if this configuration is for strict parsing
    pub fn is_strict(&self) -> bool {
        self.strict_mode && !self.allow_duplicate_keys
    }
    
    /// Check if this configuration preserves all information
    pub fn is_preserving(&self) -> bool {
        self.preserve_comments && self.include_comment_positions
    }
}

impl DeserializeOptionsBuilder {
    /// Create a new builder with default values
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let builder = DeserializeOptionsBuilder::new();
    /// let options = builder.build();
    /// ```
    pub fn new() -> Self {
        Self {
            strict_mode: true,
            max_depth: 1000,
            allow_duplicate_keys: false,
            preserve_comments: true,
            include_comment_positions: false,
            normalize_line_endings: LineEnding::None,
        }
    }
    
    /// Create builder from a preset configuration
    /// 
    /// Available presets:
    /// - "strict" - Strict JYAML spec compliance (default)
    /// - "permissive" - Lenient parsing allowing duplicates  
    /// - "fast" - Fast parsing with minimal features
    /// - "debug" - Debug mode with maximum information
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let debug_builder = DeserializeOptionsBuilder::from_preset("debug");
    /// let options = debug_builder.build();
    /// ```
    pub fn from_preset(preset: &str) -> Result<Self> {
        match preset {
            "strict" => Ok(Self::new()
                .strict_mode(true)
                .preserve_comments(true)
                .max_depth(1000)),
            "permissive" => Ok(Self::new()
                .strict_mode(false)
                .preserve_comments(true)
                .allow_duplicate_keys(true)
                .max_depth(10000)),
            "fast" => Ok(Self::new()
                .strict_mode(true)
                .preserve_comments(false)
                .max_depth(100)),
            "debug" => Ok(Self::new()
                .strict_mode(false)
                .preserve_comments(true)
                .include_comment_positions(true)
                .allow_duplicate_keys(true)),
            _ => Err(Error::InvalidOptions(
                format!("Unknown preset: {}. Available: strict, permissive, fast, debug", preset)
            ))
        }
    }
    
    /// Set strict mode parsing
    /// 
    /// When enabled, parsing is more restrictive. When disabled, allows more lenient parsing.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let options = DeserializeOptionsBuilder::new()
    ///     .strict_mode(false)
    ///     .build();
    /// ```
    pub fn strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    /// Set maximum parsing depth
    /// 
    /// Prevents stack overflow on deeply nested structures. 
    /// Must be at least 1 and at most 100000.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let options = DeserializeOptionsBuilder::new()
    ///     .max_depth(500)
    ///     .build();
    /// ```
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    
    /// Allow or disallow duplicate keys
    /// 
    /// When enabled, duplicate keys are allowed and the last value wins.
    /// When disabled, duplicate keys cause a parsing error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let options = DeserializeOptionsBuilder::new()
    ///     .allow_duplicate_keys(true)
    ///     .build();
    /// ```
    pub fn allow_duplicate_keys(mut self, allow: bool) -> Self {
        self.allow_duplicate_keys = allow;
        self
    }
    
    /// Enable or disable comment preservation
    pub fn preserve_comments(mut self, preserve: bool) -> Self {
        self.preserve_comments = preserve;
        self
    }
    
    /// Enable or disable comment position tracking
    pub fn include_comment_positions(mut self, include: bool) -> Self {
        self.include_comment_positions = include;
        self
    }
    
    /// Set line ending normalization
    pub fn normalize_line_endings(mut self, ending: LineEnding) -> Self {
        self.normalize_line_endings = ending;
        self
    }
    
    /// Build the DeserializeOptions, performing validation
    /// 
    /// This method clamps invalid values to valid ranges (e.g., max_depth 0 becomes 1).
    /// For strict validation with errors, use `try_build()` instead.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let options = DeserializeOptionsBuilder::new()
    ///     .strict_mode(false)
    ///     .max_depth(500)
    ///     .allow_duplicate_keys(true)
    ///     .build();
    /// ```
    pub fn build(self) -> DeserializeOptions {
        DeserializeOptions {
            strict_mode: self.strict_mode,
            max_depth: self.max_depth.max(1), // Ensure at least depth 1
            allow_duplicate_keys: self.allow_duplicate_keys,
            preserve_comments: self.preserve_comments,
            include_comment_positions: self.include_comment_positions,
            normalize_line_endings: self.normalize_line_endings,
        }
    }
    
    /// Build the DeserializeOptions with strict validation
    /// 
    /// Returns an error if any values are out of valid ranges or if there are
    /// incompatible option combinations.
    /// 
    /// # Errors
    /// 
    /// - `InvalidOptions` if max_depth is 0 or > 100000
    /// - `InvalidOptions` if strict_mode and allow_duplicate_keys are both true
    /// - `InvalidOptions` if include_comment_positions is true but preserve_comments is false
    /// 
    /// # Examples
    /// 
    /// ```
    /// use jyaml::DeserializeOptionsBuilder;
    /// 
    /// let result = DeserializeOptionsBuilder::new()
    ///     .max_depth(500)
    ///     .try_build();
    /// assert!(result.is_ok());
    /// 
    /// let result = DeserializeOptionsBuilder::new()
    ///     .max_depth(0)  // Invalid
    ///     .try_build();
    /// assert!(result.is_err());
    /// 
    /// let result = DeserializeOptionsBuilder::new()
    ///     .strict_mode(true)
    ///     .allow_duplicate_keys(true)  // Incompatible
    ///     .try_build();
    /// assert!(result.is_err());
    /// ```
    pub fn try_build(self) -> Result<DeserializeOptions> {
        // Validate depth limits
        if self.max_depth == 0 {
            return Err(Error::InvalidOptions("Max depth must be at least 1".to_string()));
        }
        
        if self.max_depth > 100000 {
            return Err(Error::InvalidOptions("Max depth too large (max 100000)".to_string()));
        }
        
        // Validate option compatibility
        if self.strict_mode && self.allow_duplicate_keys {
            return Err(Error::InvalidOptions(
                "strict_mode and allow_duplicate_keys are incompatible".to_string()
            ));
        }
        
        if self.include_comment_positions && !self.preserve_comments {
            return Err(Error::InvalidOptions(
                "include_comment_positions requires preserve_comments=true".to_string()
            ));
        }
        
        Ok(DeserializeOptions {
            strict_mode: self.strict_mode,
            max_depth: self.max_depth,
            allow_duplicate_keys: self.allow_duplicate_keys,
            preserve_comments: self.preserve_comments,
            include_comment_positions: self.include_comment_positions,
            normalize_line_endings: self.normalize_line_endings,
        })
    }
}

impl Default for DeserializeOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_options_presets() {
        let compact = SerializeOptions::compact();
        assert_eq!(compact.style, OutputStyle::Flow);
        assert!(!compact.pretty);
        assert_eq!(compact.indent, 0);
        
        let pretty = SerializeOptions::pretty();
        assert_eq!(pretty.style, OutputStyle::Auto);
        assert!(pretty.pretty);
        assert_eq!(pretty.indent, 2);
        
        let block = SerializeOptions::block();
        assert_eq!(block.style, OutputStyle::Block);
        assert!(block.pretty);
        assert_eq!(block.indent, 2);
        
        let json = SerializeOptions::json_compatible();
        assert_eq!(json.style, OutputStyle::Flow);
        assert_eq!(json.quote_style, QuoteStyle::Double);
        assert!(json.escape_unicode);
        assert!(!json.pretty);
    }
    
    #[test]
    fn test_serialize_options_builder() {
        let opts = SerializeOptions::default()
            .with_style(OutputStyle::Block)
            .with_indent(4).unwrap()
            .with_quote_style(QuoteStyle::Single)
            .with_escape_unicode(true)
            .with_sort_keys(true)
            .with_pretty(true);
            
        assert_eq!(opts.style, OutputStyle::Block);
        assert_eq!(opts.indent, 4);
        assert_eq!(opts.quote_style, QuoteStyle::Single);
        assert!(opts.escape_unicode);
        assert!(opts.sort_keys);
        assert!(opts.pretty);
    }
    
    #[test]
    fn test_serialize_options_validation() {
        let result = SerializeOptions::default().with_indent(10);
        assert!(result.is_err());
        
        // Test builder validation
        let result = SerializeOptionsBuilder::new()
            .indent(10)
            .try_build();
        assert!(result.is_err());
        
        // Test direct validation
        let invalid = SerializeOptions {
            indent: 10,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }
    
    #[test]
    fn test_parse_options_presets() {
        let strict = DeserializeOptions::strict();
        assert!(strict.strict_mode);
        assert!(strict.preserve_comments);
        assert_eq!(strict.max_depth, 1000);
        
        let permissive = DeserializeOptions::permissive();
        assert!(!permissive.strict_mode);
        assert!(permissive.allow_duplicate_keys);
        assert_eq!(permissive.max_depth, 10000);
        
        let fast = DeserializeOptions::fast();
        assert!(fast.strict_mode);
        assert!(!fast.preserve_comments);
        assert_eq!(fast.max_depth, 100);
        
        let debug = DeserializeOptions::debug();
        assert!(!debug.strict_mode);
        assert!(debug.include_comment_positions);
        assert!(debug.allow_duplicate_keys);
    }
    
    #[test]
    fn test_parse_options_builder() {
        let opts = DeserializeOptions::default()
            .with_strict_mode(false)
            .with_max_depth(5000)
            .with_allow_duplicate_keys(true)
            .with_preserve_comments(false);
            
        assert!(!opts.strict_mode);
        assert_eq!(opts.max_depth, 5000);
        assert!(opts.allow_duplicate_keys);
        assert!(!opts.preserve_comments);
    }
    
    #[test]
    fn test_preset_creation() {
        let debug = SerializeOptions::from_preset("debug").unwrap();
        assert_eq!(debug.style, OutputStyle::Block);
        assert!(debug.pretty);
        assert!(debug.sort_keys);
        assert_eq!(debug.indent, 4);
        
        let compact = SerializeOptions::from_preset("compact").unwrap();
        assert_eq!(compact.style, OutputStyle::Flow);
        assert!(!compact.pretty);
        assert_eq!(compact.indent, 0);
        
        let invalid = SerializeOptions::from_preset("invalid");
        assert!(invalid.is_err());
    }
    
    #[test]
    fn test_validation() {
        // Valid options should pass
        let valid = DeserializeOptions::default();
        assert!(valid.validate().is_ok());
        
        // Invalid combinations should fail
        let invalid = DeserializeOptions {
            strict_mode: true,
            allow_duplicate_keys: true,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
        
        let invalid2 = DeserializeOptions {
            preserve_comments: false,
            include_comment_positions: true,
            ..Default::default()
        };
        assert!(invalid2.validate().is_err());
    }
    
    #[test]
    fn test_helper_methods() {
        let strict = DeserializeOptions::strict();
        assert!(strict.is_strict());
        
        let debug = DeserializeOptions::debug();
        assert!(debug.is_preserving());
        
        let compact = SerializeOptions::compact();
        assert!(compact.is_compact());
        
        let pretty = SerializeOptions::pretty();
        assert!(pretty.is_readable());
    }
}