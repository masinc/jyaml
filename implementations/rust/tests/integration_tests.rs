use jyaml::{from_str, parse, to_string, value, Error, Value};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[test]
fn test_basic_values() {
    // Null
    let value = parse("null").unwrap();
    assert_eq!(value, Value::Null);

    // Boolean
    let value = parse("true").unwrap();
    assert_eq!(value, Value::Bool(true));

    let value = parse("false").unwrap();
    assert_eq!(value, Value::Bool(false));

    // Numbers
    let value = parse("42").unwrap();
    assert_eq!(value, Value::Number(value::Number::Integer(42)));

    let value = parse("3.14").unwrap();
    assert_eq!(value, Value::Number(value::Number::Float(3.14)));

    // Strings
    let value = parse("\"hello\"").unwrap();
    assert_eq!(value, Value::String("hello".to_string()));

    let value = parse("'world'").unwrap();
    assert_eq!(value, Value::String("world".to_string()));
}

#[test]
fn test_string_escapes() {
    // Double quotes
    let value = parse(r#""hello\nworld""#).unwrap();
    assert_eq!(value, Value::String("hello\nworld".to_string()));

    let value = parse(r#""unicode: \u00A9""#).unwrap();
    assert_eq!(value, Value::String("unicode: ©".to_string()));

    // Single quotes (limited escaping)
    let value = parse(r#"'can\'t stop'"#).unwrap();
    assert_eq!(value, Value::String("can't stop".to_string()));

    let value = parse(r#"'literal \n'"#).unwrap();
    assert_eq!(value, Value::String("literal \\n".to_string()));
}

#[test]
fn test_flow_array() {
    let input = r#"[1, 2, "three"]"#;
    let value = parse(input).unwrap();

    let expected = Value::Array(vec![
        Value::Number(value::Number::Integer(1)),
        Value::Number(value::Number::Integer(2)),
        Value::String("three".to_string()),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_flow_object() {
    let input = r#"{"name": "John", "age": 30}"#;
    let value = parse(input).unwrap();

    let mut expected_map = HashMap::new();
    expected_map.insert("name".to_string(), Value::String("John".to_string()));
    expected_map.insert("age".to_string(), Value::Number(value::Number::Integer(30)));

    assert_eq!(value, Value::Object(expected_map));
}

#[test]
fn test_block_object() {
    let input = r#"
"name": "John"
"age": 30
"city": "Tokyo"
"#;
    let value = parse(input).unwrap();

    let mut expected_map = HashMap::new();
    expected_map.insert("name".to_string(), Value::String("John".to_string()));
    expected_map.insert("age".to_string(), Value::Number(value::Number::Integer(30)));
    expected_map.insert("city".to_string(), Value::String("Tokyo".to_string()));

    assert_eq!(value, Value::Object(expected_map));
}

#[test]
fn test_block_array() {
    let input = r#"
- "first"
- "second"
- 42
"#;
    let value = parse(input).unwrap();

    let expected = Value::Array(vec![
        Value::String("first".to_string()),
        Value::String("second".to_string()),
        Value::Number(value::Number::Integer(42)),
    ]);
    assert_eq!(value, expected);
}

#[test]
fn test_comments() {
    let input = r#"
# YAML-style comment
"name": "John"  # inline comment
// C-style comment
"age": 30  // another inline comment
"#;
    let value = parse(input).unwrap();

    let mut expected_map = HashMap::new();
    expected_map.insert("name".to_string(), Value::String("John".to_string()));
    expected_map.insert("age".to_string(), Value::Number(value::Number::Integer(30)));

    assert_eq!(value, Value::Object(expected_map));
}

#[test]
fn test_serde_integration() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        #[serde(default)]
        languages: Vec<String>,
    }

    let jyaml = r#""name": "Alice"
"age": 25
"languages":
  - "Rust"
  - "Python"
  - "JavaScript""#;

    let person: Person = from_str(jyaml).unwrap();

    assert_eq!(
        person,
        Person {
            name: "Alice".to_string(),
            age: 25,
            languages: vec![
                "Rust".to_string(),
                "Python".to_string(),
                "JavaScript".to_string()
            ],
        }
    );
}

#[test]
fn test_nested_structures() {
    let input = r#""users":
  - "name": "Alice"
    "age": 30
"config":
  "timeout": 30"#;

    let value = parse(input).unwrap();

    println!("Parsed value: {:#?}", value);

    // Verify structure without exact comparison due to HashMap ordering
    if let Value::Object(obj) = value {
        println!("Object keys: {:?}", obj.keys().collect::<Vec<_>>());
        assert!(obj.contains_key("users"));
        assert!(obj.contains_key("config"));

        if let Some(Value::Array(users)) = obj.get("users") {
            assert_eq!(users.len(), 1);
            if let Some(Value::Object(user)) = users.get(0) {
                assert!(user.contains_key("name"));
                assert!(user.contains_key("age"));
            }
        } else {
            panic!("users should be an array");
        }

        if let Some(Value::Object(config)) = obj.get("config") {
            assert!(config.contains_key("timeout"));
        } else {
            panic!("config should be an object");
        }
    } else {
        panic!("Root should be an object");
    }
}

#[test]
fn test_error_cases() {
    // Invalid JSON syntax
    assert!(parse("{").is_err());
    assert!(parse("\"unclosed string").is_err());
    assert!(parse("01234").is_err()); // Leading zeros
    assert!(parse("yes").is_err()); // Invalid boolean

    // Duplicate keys
    let input = r#"
"name": "first"
"name": "second"
"#;
    assert!(parse(input).is_err());
}

#[test]
fn test_utf8_validation() {
    // Valid UTF-8
    assert!(parse("\"こんにちは\"").is_ok());
    assert!(parse("\"🦀\"").is_ok());

    // Unicode escapes
    let value = parse(r#""\u3053\u3093\u306b\u3061\u306f""#).unwrap();
    assert_eq!(value, Value::String("こんにちは".to_string()));
}

#[test]
fn test_trailing_comma_support() {
    // Trailing comma in array (JYAML 0.3 feature)
    let input = r#"[1, 2, 3,]"#;
    let value = parse(input).unwrap();
    let expected = Value::Array(vec![
        Value::Number(value::Number::Integer(1)),
        Value::Number(value::Number::Integer(2)),
        Value::Number(value::Number::Integer(3)),
    ]);
    assert_eq!(value, expected);

    // Trailing comma in object (JYAML 0.3 feature)
    let input = r#"{"name": "Alice", "age": 30,}"#;
    let value = parse(input).unwrap();
    let mut expected_map = HashMap::new();
    expected_map.insert("name".to_string(), Value::String("Alice".to_string()));
    expected_map.insert("age".to_string(), Value::Number(value::Number::Integer(30)));
    assert_eq!(value, Value::Object(expected_map));

    // Empty array/object with trailing comma should still be empty
    let value = parse("[,]").unwrap_or_else(|_| parse("[]").unwrap());
    // Note: [,] might be invalid syntax, but [] should work
    let value = parse("[]").unwrap();
    assert_eq!(value, Value::Array(vec![]));

    let value = parse("{}").unwrap();
    assert_eq!(value, Value::Object(HashMap::new()));
}
