use jyaml::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JYAML Mixed Style Examples ===\n");

    // Example 1: Block object with flow array values
    println!("1. Block object with flow array values:");
    let mixed1_jyaml = r#"
"languages": ["Rust", "Python", "Go"]
"frameworks": ["Actix", "Django", "Gin"]
"tools": ["Cargo", "Poetry", "Go Modules"]
"#;
    let mixed1 = parse(mixed1_jyaml)?;
    println!("  JYAML:\n{}", mixed1_jyaml);
    println!("  Parsed: {:#?}\n", mixed1);

    // Example 2: Block object with flow object values
    println!("2. Block object with flow object values:");
    let mixed2_jyaml = r#"
"person": {"name": "Alice", "age": 30}
"address": {"city": "Tokyo", "country": "Japan"}
"contact": {"email": "alice@example.com", "phone": "+81-90-1234-5678"}
"#;
    let mixed2 = parse(mixed2_jyaml)?;
    println!("  JYAML:\n{}", mixed2_jyaml);
    println!("  Parsed: {:#?}\n", mixed2);

    // Example 3: Block array with flow object elements
    println!("3. Block array with flow object elements:");
    let mixed3_jyaml = r#"
- {"name": "Tokyo", "population": 14000000}
- {"name": "Osaka", "population": 19000000}
- {"name": "Kyoto", "population": 1500000}
"#;
    let mixed3 = parse(mixed3_jyaml)?;
    println!("  JYAML:\n{}", mixed3_jyaml);
    println!("  Parsed: {:#?}\n", mixed3);

    // Example 4: Flow object with block-style multi-line strings
    println!("4. Complex mixed structure:");
    let mixed4_jyaml = r#"
"metadata": {"version": "1.0", "created": "2023-12-01"}
"description": |
  This is a multi-line description
  that spans several lines
  and preserves line breaks.
"config":
  "database": {"host": "localhost", "port": 5432}
  "cache": {"type": "redis", "ttl": 3600}
"features": ["authentication", "logging", "monitoring"]
"#;
    let mixed4 = parse(mixed4_jyaml)?;
    println!("  JYAML:\n{}", mixed4_jyaml);
    println!("  Parsed: {:#?}\n", mixed4);

    // Example 5: Deeply nested mixed styles
    println!("5. Deeply nested mixed styles:");
    let mixed5_jyaml = r#"
"services":
  "web":
    "instances": [
      {"name": "web-1", "port": 8001},
      {"name": "web-2", "port": 8002}
    ]
    "config": {"workers": 4, "timeout": 30}
  "api":
    "instances": [{"name": "api-1", "port": 9001}]
    "docs": >
      This API service provides
      RESTful endpoints for
      client applications.
"#;
    let mixed5 = parse(mixed5_jyaml)?;
    println!("  JYAML:\n{}", mixed5_jyaml);
    println!("  Parsed: {:#?}\n", mixed5);

    // Example 6: Configuration file example
    println!("6. Real-world configuration example:");
    let config_jyaml = r#"
# Application configuration
"app":
  "name": "JYAML Demo"
  "version": "0.2.0"
  "debug": true

# Database settings  
"database":
  "primary": {"host": "db1.example.com", "port": 5432}
  "replica": {"host": "db2.example.com", "port": 5432}
  "pool_size": 10

# Logging configuration
"logging":
  "level": "info"
  "outputs": ["console", "file"]
  "format": >
    This is the log format that will be used
    across all logging outputs in the system.

// Server configuration
"server":
  "host": "0.0.0.0"
  "port": 8080
  "middleware": ["cors", "compression", "auth"]
"#;
    let config = parse(config_jyaml)?;
    println!("  JYAML:\n{}", config_jyaml);
    println!("  Parsed: {:#?}\n", config);

    println!("=== Mixed style examples completed! ===");
    Ok(())
}