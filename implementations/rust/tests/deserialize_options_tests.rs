use jyaml::{from_str_with_options, parse_with_options, DeserializeOptions, Error, Value};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug)]
struct TestStruct {
    name: String,
    age: u32,
}

#[test]
fn test_from_str_with_options_basic() {
    let input = r#""name": "Alice"
"age": 30"#;

    let options = DeserializeOptions::default();
    let result: TestStruct = from_str_with_options(input, &options).unwrap();

    assert_eq!(result.name, "Alice");
    assert_eq!(result.age, 30);
}

#[test]
fn test_parse_with_options_basic() {
    let input = r#""name": "Alice"
"age": 30"#;

    let options = DeserializeOptions::default();
    let value = parse_with_options(input, &options).unwrap();

    if let Value::Object(obj) = value {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(
            obj.get("age"),
            Some(&Value::Number(jyaml::value::Number::Integer(30)))
        );
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_allow_duplicate_keys() {
    let input = r#""name": "Alice"
"name": "Bob""#;

    // Test with duplicate keys disallowed (default)
    let strict_options = DeserializeOptions::default();
    let result = parse_with_options(input, &strict_options);
    assert!(result.is_err());
    if let Err(Error::DuplicateKey { key, .. }) = result {
        assert_eq!(key, "name");
    } else {
        panic!("Expected DuplicateKey error");
    }

    // Test with duplicate keys allowed
    let permissive_options = DeserializeOptions::default().with_allow_duplicate_keys(true);
    let result = parse_with_options(input, &permissive_options).unwrap();

    if let Value::Object(obj) = result {
        // Last value should win
        assert_eq!(obj.get("name"), Some(&Value::String("Bob".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_max_depth_limit() {
    // Create deeply nested structure
    let input = r#""level1":
  "level2":
    "level3":
      "level4":
        "level5": "deep""#;

    // Test with low max depth
    let shallow_options = DeserializeOptions::default().with_max_depth(1);
    let result = parse_with_options(input, &shallow_options);
    assert!(result.is_err());

    // Test with sufficient max depth
    let deep_options = DeserializeOptions::default().with_max_depth(10);
    let result = parse_with_options(input, &deep_options);
    assert!(result.is_ok());
}

#[test]
fn test_parse_options_presets() {
    let input = r#""name": "Alice"
"name": "Bob"
"nested":
  "deep":
    "value": 42"#;

    // Test strict preset
    let strict = DeserializeOptions::strict();
    let result = parse_with_options(input, &strict);
    assert!(result.is_err()); // Should fail on duplicate keys

    // Test permissive preset
    let permissive = DeserializeOptions::permissive();
    let result = parse_with_options(input, &permissive);
    assert!(result.is_ok()); // Should allow duplicate keys

    // Test fast preset
    let fast = DeserializeOptions::fast();
    let result = parse_with_options(input, &fast);
    assert!(result.is_err()); // Should fail on duplicate keys (strict mode)

    // Test debug preset
    let debug = DeserializeOptions::debug();
    let result = parse_with_options(input, &debug);
    assert!(result.is_ok()); // Should allow duplicate keys
}

#[test]
fn test_parse_options_builder() {
    let input = r#""name": "Alice"
"name": "Bob""#;

    let options = DeserializeOptions::builder()
        .strict_mode(false)
        .allow_duplicate_keys(true)
        .max_depth(100)
        .preserve_comments(false)
        .build();

    let result = parse_with_options(input, &options);
    assert!(result.is_ok());

    if let Value::Object(obj) = result.unwrap() {
        assert_eq!(obj.get("name"), Some(&Value::String("Bob".to_string())));
    }
}

#[test]
fn test_parse_options_builder_validation() {
    // Test builder with valid options
    let valid_options = DeserializeOptions::builder().max_depth(1000).try_build();
    assert!(valid_options.is_ok());

    // Test builder with invalid options
    let invalid_options = DeserializeOptions::builder().max_depth(0).try_build();
    assert!(invalid_options.is_err());

    let too_deep_options = DeserializeOptions::builder().max_depth(200000).try_build();
    assert!(too_deep_options.is_err());
}

#[test]
fn test_serde_integration_with_options() {
    let input = r#""name": "Alice"
"age": 30
"active": true"#;

    let options = DeserializeOptions::permissive();
    let result: TestStruct = from_str_with_options(input, &options).unwrap();

    assert_eq!(result.name, "Alice");
    assert_eq!(result.age, 30);
}

#[test]
fn test_comment_preservation_option() {
    let input = r#"# This is a comment
"name": "Alice" # Inline comment
"age": 30"#;

    // Test with comment preservation enabled (default)
    let preserve_options = DeserializeOptions::default().with_preserve_comments(true);
    let result = parse_with_options(input, &preserve_options);
    assert!(result.is_ok());

    // Test with comment preservation disabled
    let no_preserve_options = DeserializeOptions::default().with_preserve_comments(false);
    let result = parse_with_options(input, &no_preserve_options);
    assert!(result.is_ok());

    // Both should produce the same result since comments are not stored in Value
    if let Value::Object(obj) = result.unwrap() {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(
            obj.get("age"),
            Some(&Value::Number(jyaml::value::Number::Integer(30)))
        );
    }
}
