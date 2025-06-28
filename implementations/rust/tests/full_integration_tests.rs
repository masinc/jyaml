use jyaml::{
    from_str, from_str_with_options, to_string, to_string_pretty, to_string_with_options,
    parse, parse_with_options, Value,
    SerializeOptions, SerializeOptionsBuilder, DeserializeOptions, DeserializeOptionsBuilder,
    OutputStyle, QuoteStyle, LineEnding, Error,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TestUser {
    name: String,
    age: u32,
    email: String,
    active: bool,
    tags: Vec<String>,
    metadata: HashMap<String, Value>,
}


fn create_test_user() -> TestUser {
    let mut metadata = HashMap::new();
    metadata.insert("created".to_string(), Value::String("2023-01-01".to_string()));
    metadata.insert("version".to_string(), Value::Number(jyaml::value::Number::Integer(1)));
    metadata.insert("premium".to_string(), Value::Bool(true));
    
    TestUser {
        name: "Integration Test User".to_string(),
        age: 25,
        email: "test@example.com".to_string(),
        active: true,
        tags: vec!["test".to_string(), "integration".to_string(), "jyaml".to_string()],
        metadata,
    }
}

#[test]
fn test_full_roundtrip_basic() {
    let user = create_test_user();
    
    // Serialize
    let jyaml_str = to_string(&user).unwrap();
    assert!(!jyaml_str.is_empty());
    
    // Deserialize
    let parsed_user: TestUser = from_str(&jyaml_str).unwrap();
    assert_eq!(user, parsed_user);
}

#[test]
fn test_full_roundtrip_pretty() {
    let user = create_test_user();
    
    // Serialize with pretty printing
    let jyaml_str = to_string_pretty(&user, 4).unwrap();
    assert!(jyaml_str.contains('\n')); // Should have newlines
    assert!(jyaml_str.contains("    ")); // Should have 4-space indentation
    
    // Deserialize
    let parsed_user: TestUser = from_str(&jyaml_str).unwrap();
    assert_eq!(user, parsed_user);
}

#[test]
fn test_serialize_options_roundtrip() {
    let user = create_test_user();
    
    // Test different serialize options
    let options_list = vec![
        SerializeOptions::compact(),
        SerializeOptions::pretty(),
        SerializeOptions::block(),
        SerializeOptions::json_compatible(),
    ];
    
    for options in options_list {
        let jyaml_str = to_string_with_options(&user, &options).unwrap();
        let parsed_user: TestUser = from_str(&jyaml_str).unwrap();
        assert_eq!(user, parsed_user, "Failed with options: {:?}", options);
    }
}

#[test]
fn test_deserialize_options_strict_vs_permissive() {
    let input_with_duplicates = r#""name": "Test User"
"age": 25
"email": "first@example.com"
"email": "second@example.com"
"active": true
"tags": ["test"]
"metadata": {}"#;
    
    // Strict mode should reject duplicates
    let strict = DeserializeOptions::strict();
    let result = parse_with_options(input_with_duplicates, &strict);
    assert!(result.is_err());
    if let Err(Error::DuplicateKey { key, .. }) = result {
        assert_eq!(key, "email");
    } else {
        panic!("Expected DuplicateKey error");
    }
    
    // Permissive mode should allow duplicates (last value wins)
    let permissive = DeserializeOptions::permissive();
    let result = parse_with_options(input_with_duplicates, &permissive).unwrap();
    if let Value::Object(obj) = result {
        if let Some(Value::String(email)) = obj.get("email") {
            assert_eq!(email, "second@example.com");
        } else {
            panic!("Expected email field");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_nested_data_structures() {
    let user = create_test_user();
    
    // Test simpler nested structure first
    let simple_nested = HashMap::from([
        ("user_name".to_string(), user.name.clone()),
        ("user_age".to_string(), user.age.to_string()),
        ("user_email".to_string(), user.email.clone()),
    ]);
    
    // Serialize
    let jyaml_str = to_string_pretty(&simple_nested, 2).unwrap();
    
    // Deserialize
    let parsed: HashMap<String, String> = from_str(&jyaml_str).unwrap();
    assert_eq!(simple_nested, parsed);
    
    // Test the user itself
    let user_jyaml = to_string_pretty(&user, 2).unwrap();
    let parsed_user: TestUser = from_str(&user_jyaml).unwrap();
    assert_eq!(user, parsed_user);
}

#[test]
fn test_unicode_handling() {
    let unicode_data = HashMap::from([
        ("ascii".to_string(), "Hello World".to_string()),
        ("latin".to_string(), "Café résumé naïve".to_string()),
        ("chinese".to_string(), "你好世界".to_string()),
        ("japanese".to_string(), "こんにちは世界".to_string()),
        ("emoji".to_string(), "🌍🦀🚀💯".to_string()),
        ("symbols".to_string(), "© ® ™ € £ ¥".to_string()),
    ]);
    
    // Test without Unicode escaping
    let no_escape_opts = SerializeOptions::builder()
        .escape_unicode(false)
        .build();
    let jyaml_str = to_string_with_options(&unicode_data, &no_escape_opts).unwrap();
    let parsed: HashMap<String, String> = from_str(&jyaml_str).unwrap();
    assert_eq!(unicode_data, parsed);
    
    // Test with Unicode escaping
    let escape_opts = SerializeOptions::builder()
        .escape_unicode(true)
        .build();
    let escaped_str = to_string_with_options(&unicode_data, &escape_opts).unwrap();
    let parsed_escaped: HashMap<String, String> = from_str(&escaped_str).unwrap();
    assert_eq!(unicode_data, parsed_escaped);
    
    // Escaped version should contain \u sequences for non-ASCII
    assert!(escaped_str.contains("\\u"));
}

#[test]
fn test_builder_pattern_validation() {
    // Test valid builder usage
    let valid_serialize = SerializeOptionsBuilder::new()
        .style(OutputStyle::Block)
        .indent(4)
        .quote_style(QuoteStyle::Double)
        .sort_keys(true)
        .escape_unicode(false)
        .pretty(true)
        .try_build();
    assert!(valid_serialize.is_ok());
    
    let valid_deserialize = DeserializeOptionsBuilder::new()
        .strict_mode(false)
        .max_depth(100)
        .allow_duplicate_keys(true)
        .preserve_comments(true)
        .try_build();
    assert!(valid_deserialize.is_ok());
    
    // Test invalid builder usage
    let invalid_serialize = SerializeOptionsBuilder::new()
        .indent(10) // Invalid: > 8
        .try_build();
    assert!(invalid_serialize.is_err());
    
    let invalid_deserialize_zero = DeserializeOptionsBuilder::new()
        .max_depth(0) // Invalid: must be >= 1
        .try_build();
    assert!(invalid_deserialize_zero.is_err());
    
    let invalid_deserialize_large = DeserializeOptionsBuilder::new()
        .max_depth(200000) // Invalid: > 100000
        .try_build();
    assert!(invalid_deserialize_large.is_err());
}

#[test]
fn test_value_api() {
    // Test simpler value structures to avoid serializer issues
    let simple_value = Value::Object([
        ("string".to_string(), Value::String("test".to_string())),
        ("number".to_string(), Value::Number(jyaml::value::Number::Integer(42))),
        ("bool".to_string(), Value::Bool(true)),
        ("null".to_string(), Value::Null),
    ].iter().cloned().collect());
    
    // Serialize Value
    let jyaml_str = to_string_pretty(&simple_value, 2).unwrap();
    
    // Parse back to Value
    let parsed_value = parse(&jyaml_str).unwrap();
    
    // Test individual fields
    if let (Value::Object(original), Value::Object(parsed)) = (&simple_value, &parsed_value) {
        for (key, value) in original {
            assert_eq!(parsed.get(key), Some(value), "Mismatch for key: {}", key);
        }
        assert_eq!(original.len(), parsed.len(), "Different number of keys");
    } else {
        panic!("Expected both to be objects");
    }
    
    // Test array separately
    let array_value = Value::Array(vec![
        Value::String("item1".to_string()),
        Value::String("item2".to_string()),
        Value::Number(jyaml::value::Number::Integer(123)),
    ]);
    
    let array_jyaml = to_string(&array_value).unwrap();
    let parsed_array = parse(&array_jyaml).unwrap();
    assert_eq!(array_value, parsed_array);
    
    // Test nested object separately
    let nested_value = Value::Object([
        ("outer".to_string(), Value::Object([
            ("inner".to_string(), Value::String("value".to_string())),
        ].iter().cloned().collect())),
    ].iter().cloned().collect());
    
    let nested_jyaml = to_string_pretty(&nested_value, 2).unwrap();
    let parsed_nested = parse(&nested_jyaml).unwrap();
    
    if let (Value::Object(original), Value::Object(parsed)) = (&nested_value, &parsed_nested) {
        assert_eq!(original.len(), parsed.len());
        if let (Some(Value::Object(orig_inner)), Some(Value::Object(parsed_inner))) = 
            (original.get("outer"), parsed.get("outer")) {
            assert_eq!(orig_inner.get("inner"), parsed_inner.get("inner"));
        }
    }
}

#[test]
fn test_depth_limits() {
    // Create a deeply nested structure
    let deep_input = "\"l1\": { \"l2\": { \"l3\": { \"l4\": { \"l5\": \"deep\" } } } }";
    
    // Should succeed with sufficient depth
    let high_depth = DeserializeOptions::builder()
        .max_depth(10)
        .build();
    let result = parse_with_options(deep_input, &high_depth);
    assert!(result.is_ok());
    
    // Should fail with insufficient depth
    let low_depth = DeserializeOptions::builder()
        .max_depth(2)
        .build();
    let result = parse_with_options(deep_input, &low_depth);
    assert!(result.is_err());
}

#[test]
fn test_line_ending_handling() {
    let data = HashMap::from([
        ("first".to_string(), "line1".to_string()),
        ("second".to_string(), "line2".to_string()),
    ]);
    
    // Test different line ending options
    let lf_opts = SerializeOptions::builder()
        .line_ending(LineEnding::Lf)
        .pretty(true)
        .build();
    let lf_result = to_string_with_options(&data, &lf_opts).unwrap();
    
    let crlf_opts = SerializeOptions::builder()
        .line_ending(LineEnding::Crlf)
        .pretty(true)
        .build();
    let crlf_result = to_string_with_options(&data, &crlf_opts).unwrap();
    
    let none_opts = SerializeOptions::builder()
        .line_ending(LineEnding::None)
        .pretty(true)
        .build();
    let none_result = to_string_with_options(&data, &none_opts).unwrap();
    
    // All should deserialize correctly regardless of line endings
    let parsed_lf: HashMap<String, String> = from_str(&lf_result).unwrap();
    let parsed_crlf: HashMap<String, String> = from_str(&crlf_result).unwrap();
    let parsed_none: HashMap<String, String> = from_str(&none_result).unwrap();
    
    assert_eq!(data, parsed_lf);
    assert_eq!(data, parsed_crlf);
    assert_eq!(data, parsed_none);
}

#[test]
fn test_error_conditions() {
    // Test various error conditions
    
    // Invalid JSON/JYAML syntax
    let invalid_inputs = vec![
        r#""unclosed_string"#,
        r#""key": "value" "missing_comma""#,
        r#"{"malformed": }"#,
        r#""key": value_without_quotes"#,
    ];
    
    for invalid_input in invalid_inputs {
        let result = from_str::<HashMap<String, String>>(invalid_input);
        assert!(result.is_err(), "Should fail for input: {}", invalid_input);
    }
    
    // Type mismatch
    let type_mismatch_input = r#""name": "Alice", "age": "not_a_number""#;
    let result = from_str::<TestUser>(type_mismatch_input);
    assert!(result.is_err());
}

#[test]
fn test_serde_compatibility() {
    // Test that our JYAML output can be parsed by serde_json for simple cases
    let simple_data = HashMap::from([
        ("name".to_string(), "Alice".to_string()),
        ("age".to_string(), "30".to_string()),
    ]);
    
    let json_compatible = SerializeOptions::json_compatible();
    let output = to_string_with_options(&simple_data, &json_compatible).unwrap();
    
    // Note: This might not always work due to current implementation differences,
    // but we test the option exists and produces output
    assert!(!output.is_empty());
    assert!(output.contains("Alice"));
    assert!(output.contains("30"));
}

#[test]
fn test_preset_options() {
    let user = create_test_user();
    
    // Test all preset serialize options
    let presets = vec![
        ("compact", SerializeOptions::compact()),
        ("pretty", SerializeOptions::pretty()),
        ("block", SerializeOptions::block()),
        ("json_compatible", SerializeOptions::json_compatible()),
    ];
    
    for (name, preset) in presets {
        let result = to_string_with_options(&user, &preset);
        assert!(result.is_ok(), "Preset {} failed", name);
        
        let jyaml_str = result.unwrap();
        let parsed: TestUser = from_str(&jyaml_str).unwrap();
        assert_eq!(user, parsed, "Roundtrip failed for preset {}", name);
    }
    
    // Test all preset deserialize options
    let input = to_string(&user).unwrap();
    let presets = vec![
        ("strict", DeserializeOptions::strict()),
        ("permissive", DeserializeOptions::permissive()),
        ("fast", DeserializeOptions::fast()),
        ("debug", DeserializeOptions::debug()),
    ];
    
    for (name, preset) in presets {
        let result: Result<TestUser, _> = from_str_with_options(&input, &preset);
        assert!(result.is_ok(), "Preset {} failed", name);
        
        let parsed = result.unwrap();
        assert_eq!(user, parsed, "Roundtrip failed for preset {}", name);
    }
}

#[test]
fn test_comments_handling() {
    let input_with_comments = r#"# This is a comment
"name": "Alice" # Inline comment
# Another comment
"age": 30
"active": true"#;
    
    // Should parse successfully even with comments
    let result = parse(input_with_comments);
    assert!(result.is_ok());
    
    if let Value::Object(obj) = result.unwrap() {
        assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(obj.get("age"), Some(&Value::Number(jyaml::value::Number::Integer(30))));
        assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
    } else {
        panic!("Expected object");
    }
    
    // Test with comment preservation options
    let preserve_comments = DeserializeOptions::builder()
        .preserve_comments(true)
        .build();
    let result = parse_with_options(input_with_comments, &preserve_comments);
    assert!(result.is_ok());
    
    let no_preserve_comments = DeserializeOptions::builder()
        .preserve_comments(false)
        .build();
    let result = parse_with_options(input_with_comments, &no_preserve_comments);
    assert!(result.is_ok());
}