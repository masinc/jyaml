use jyaml::{parse, to_string, from_str, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    languages: Vec<String>,
    active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JYAML Basic Usage Examples ===\n");

    // Example 1: Parse simple values
    println!("1. Simple value parsing:");
    let number = parse("42")?;
    let string = parse("\"hello world\"")?;
    let boolean = parse("true")?;
    println!("  Number: {:?}", number);
    println!("  String: {:?}", string);
    println!("  Boolean: {:?}\n", boolean);

    // Example 2: Parse flow-style array
    println!("2. Flow-style array:");
    let flow_array = parse(r#"["Rust", "Python", "JavaScript"]"#)?;
    println!("  Parsed: {:?}\n", flow_array);

    // Example 3: Parse block-style array
    println!("3. Block-style array:");
    let block_array_jyaml = r#"
- "Rust"
- "Python"
- "JavaScript"
"#;
    let block_array = parse(block_array_jyaml)?;
    println!("  JYAML:\n{}", block_array_jyaml);
    println!("  Parsed: {:?}\n", block_array);

    // Example 4: Parse flow-style object
    println!("4. Flow-style object:");
    let flow_object = parse(r#"{"name": "Alice", "age": 30}"#)?;
    println!("  Parsed: {:?}\n", flow_object);

    // Example 5: Parse block-style object
    println!("5. Block-style object:");
    let block_object_jyaml = r#"
"name": "Bob"
"age": 25
"active": true
"#;
    let block_object = parse(block_object_jyaml)?;
    println!("  JYAML:\n{}", block_object_jyaml);
    println!("  Parsed: {:?}\n", block_object);

    // Example 6: Parse with comments
    println!("6. JYAML with comments:");
    let with_comments = r#"
# This is a YAML-style comment
"name": "Charlie"  # inline comment
// This is a C-style comment
"age": 35  // another inline comment
"#;
    let parsed_comments = parse(with_comments)?;
    println!("  JYAML:\n{}", with_comments);
    println!("  Parsed: {:?}\n", parsed_comments);

    // Example 7: Serialize to JYAML
    println!("7. Serialize to JYAML:");
    let mut data = HashMap::new();
    data.insert("name".to_string(), Value::String("Diana".to_string()));
    data.insert("age".to_string(), Value::Number(jyaml::value::Number::Integer(28)));
    data.insert("active".to_string(), Value::Bool(true));
    
    let serialized = to_string(&Value::Object(data))?;
    println!("  Serialized: {}\n", serialized);

    // Example 8: Serde integration
    println!("8. Serde integration:");
    let person_jyaml = r#"
"name": "Eve"
"age": 32
"languages": ["Rust", "Go", "TypeScript"]
"active": true
"#;
    
    let person: Person = from_str(person_jyaml)?;
    println!("  JYAML:\n{}", person_jyaml);
    println!("  Deserialized struct: {:?}\n", person);

    // Example 9: Complex nested structure
    println!("9. Complex nested structure:");
    let complex_jyaml = r#"
"project": "JYAML"
"version": "0.2"
"maintainers":
  - "name": "Alice"
    "role": "Lead"
  - "name": "Bob"
    "role": "Contributor"
"config":
  "debug": true
  "max_connections": 100
"#;
    
    let complex = parse(complex_jyaml)?;
    println!("  JYAML:\n{}", complex_jyaml);
    println!("  Parsed: {:#?}\n", complex);

    println!("=== All examples completed successfully! ===");
    Ok(())
}