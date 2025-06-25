# Changelog

All notable changes to the JYAML Rust implementation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-06-25

### Added
- Full JYAML 0.3 specification compliance
- Trailing comma support in flow-style arrays and objects
- Comprehensive tab character detection and error reporting
- Test file runner with timeout protection (`examples/test_file.rs`)
- Debug utilities for tab character analysis (`examples/debug_tab.rs`)
- Extensive test suite covering edge cases and error conditions
- Proper error propagation from lexer to parser
- Line and column position tracking for all errors

### Fixed
- Infinite loop issues when encountering tab characters
- Parser error handling now properly propagates lexer errors
- Tab character detection now works correctly in all contexts
- Trailing comma parsing no longer causes infinite loops

### Changed
- Updated to JYAML specification version 0.3
- Improved error messages with precise line/column information
- Enhanced lexer robustness with better error handling
- Parser methods now return `Result<()>` for proper error propagation

### Technical Details
- Parser supports both flow-style (`[1, 2, 3,]`) and object (`{"a": 1, "b": 2,}`) trailing commas
- Tab characters are properly detected and cause `TabInIndentation` errors
- Timeout mechanism prevents infinite loops during parsing
- Comprehensive test coverage for both valid and invalid JYAML inputs

## [0.2.0] - 2025-06-25

### Added
- Initial Rust implementation of JYAML parser
- Support for all JYAML data types (null, boolean, number, string, array, object)
- Flow and block style parsing
- Comment support (`#` and `//` styles)
- Multiline string support (`|` and `>` indicators)
- UTF-8 validation and BOM rejection
- Basic error handling with position tracking

### Features
- Lexer with comprehensive token support
- Parser with recursive descent parsing
- Value enum representing all JYAML data types
- Number type supporting integers and floats
- String escaping and Unicode support
- Indentation-based block structure parsing

## [0.1.0] - 2025-06-25

### Added
- Project structure and initial setup
- Basic lexer framework
- Token definitions for JYAML syntax
- Error type definitions
- Initial parser skeleton