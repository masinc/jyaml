# JYAML Rust Implementation

A Rust library for parsing and serializing JYAML (JSON-YAML Adaptive Markup Language).

> **Note**: Currently has known issues with indentation validation and nested object parsing that are being addressed.

## Features

- Full JYAML 0.2 specification support
- Serde integration for easy serialization/deserialization
- Streaming parser for large files
- Comprehensive error messages with line/column information
- Zero-copy parsing where possible

## Usage

```rust
use jyaml::{parse, serialize, Value};

// Parse JYAML text
let text = r#"
"name": "John Doe"
"age": 30
"languages":
  - "Rust"
  - "Python"
"#;

let value = parse(text)?;

// Serialize to JYAML
let output = serialize(&value)?;

// Use with Serde
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    languages: Vec<String>,
}

let person: Person = jyaml::from_str(text)?;
let jyaml_string = jyaml::to_string(&person)?;
```

## Installation

```toml
[dependencies]
jyaml = "0.1"
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.