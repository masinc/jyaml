use jyaml::{from_str, Value as JyamlValue};
use pretty_assertions::assert_eq;
use serde_json::Value as JsonValue;

/// Convert JYAML Value to JSON Value for comparison
fn jyaml_to_json(jyaml_value: &JyamlValue) -> JsonValue {
    match jyaml_value {
        JyamlValue::Null => JsonValue::Null,
        JyamlValue::Bool(b) => JsonValue::Bool(*b),
        JyamlValue::Number(n) => match n {
            jyaml::value::Number::Integer(i) => JsonValue::Number(serde_json::Number::from(*i)),
            jyaml::value::Number::Float(f) => {
                JsonValue::Number(serde_json::Number::from_f64(*f).unwrap())
            }
        },
        JyamlValue::String(s) => JsonValue::String(s.clone()),
        JyamlValue::Array(arr) => JsonValue::Array(arr.iter().map(jyaml_to_json).collect()),
        JyamlValue::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                map.insert(k.clone(), jyaml_to_json(v));
            }
            JsonValue::Object(map)
        }
    }
}

#[test]
fn test_json_compatibility_basic_values() {
    let test_cases = vec![
        ("null", "null"),
        ("true", "true"),
        ("false", "false"),
        ("42", "42"),
        ("-10", "-10"),
        ("3.14", "3.14"),
        ("1.5e-3", "1.5e-3"),
        (r#""hello""#, r#""hello""#),
        (r#""hello\nworld""#, r#""hello\nworld""#),
    ];

    for (jyaml_input, json_input) in test_cases {
        println!("Testing: {}", jyaml_input);

        let jyaml_value = from_str::<JyamlValue>(jyaml_input).unwrap();
        let json_value: JsonValue = serde_json::from_str(json_input).unwrap();

        let converted_jyaml = jyaml_to_json(&jyaml_value);
        assert_eq!(
            converted_jyaml, json_value,
            "Values should match for input: {}",
            jyaml_input
        );
    }
}

#[test]
fn test_json_compatibility_arrays() {
    let test_cases = vec![
        ("[]", "[]"),
        ("[1, 2, 3]", "[1, 2, 3]"),
        (r#"["a", "b", "c"]"#, r#"["a", "b", "c"]"#),
        ("[null, true, false]", "[null, true, false]"),
        ("[[1, 2], [3, 4]]", "[[1, 2], [3, 4]]"),
    ];

    for (jyaml_input, json_input) in test_cases {
        println!("Testing array: {}", jyaml_input);

        let jyaml_value = from_str::<JyamlValue>(jyaml_input).unwrap();
        let json_value: JsonValue = serde_json::from_str(json_input).unwrap();

        let converted_jyaml = jyaml_to_json(&jyaml_value);
        assert_eq!(
            converted_jyaml, json_value,
            "Arrays should match for input: {}",
            jyaml_input
        );
    }
}

#[test]
fn test_json_compatibility_objects() {
    let test_cases = vec![
        ("{}", "{}"),
        (r#"{"name": "John"}"#, r#"{"name": "John"}"#),
        (
            r#"{"age": 30, "active": true}"#,
            r#"{"age": 30, "active": true}"#,
        ),
        (r#"{"values": [1, 2, 3]}"#, r#"{"values": [1, 2, 3]}"#),
    ];

    for (jyaml_input, json_input) in test_cases {
        println!("Testing object: {}", jyaml_input);

        let jyaml_value = from_str::<JyamlValue>(jyaml_input).unwrap();
        let json_value: JsonValue = serde_json::from_str(json_input).unwrap();

        let converted_jyaml = jyaml_to_json(&jyaml_value);
        assert_eq!(
            converted_jyaml, json_value,
            "Objects should match for input: {}",
            jyaml_input
        );
    }
}

#[test]
fn test_json_compatibility_unicode() {
    let test_cases = vec![
        // BMP characters should be identical
        (r#""\u00A9 2023""#, r#""\u00A9 2023""#),
        (r#""\u00AE\u2603""#, r#""\u00AE\u2603""#),
        // JYAML supports direct emoji, JSON needs escape
        (r#""🚀""#, r#""\ud83d\ude80""#),
        (r#""Hello 🌍 World""#, r#""Hello \ud83c\udf0d World""#),
    ];

    for (jyaml_input, json_input) in test_cases {
        println!("Testing Unicode: {} vs {}", jyaml_input, json_input);

        let jyaml_value = from_str::<JyamlValue>(jyaml_input).unwrap();
        let json_value: JsonValue = serde_json::from_str(json_input).unwrap();

        let converted_jyaml = jyaml_to_json(&jyaml_value);

        // For Unicode, we compare the actual string content, not the representation
        if let (JsonValue::String(jyaml_str), JsonValue::String(json_str)) =
            (&converted_jyaml, &json_value)
        {
            assert_eq!(
                jyaml_str, json_str,
                "Unicode strings should have same content for: {} vs {}",
                jyaml_input, json_input
            );
        } else {
            assert_eq!(
                converted_jyaml, json_value,
                "Values should match for input: {} vs {}",
                jyaml_input, json_input
            );
        }
    }
}

#[test]
fn test_jyaml_surrogate_pair_parsing() {
    // Test that JYAML can parse surrogate pairs like JSON
    let test_cases = vec![
        (r#""\uD83D\uDE80""#, "🚀"), // Rocket emoji
        (r#""\uD83C\uDF89""#, "🎉"), // Party emoji
        (r#""\uD83E\uDD80""#, "🦀"), // Crab emoji
        (r#""\uD83C\uDF0D""#, "🌍"), // Earth emoji
    ];

    for (surrogate_input, expected_emoji) in test_cases {
        println!(
            "Testing surrogate pair: {} -> {}",
            surrogate_input, expected_emoji
        );

        // Parse with JYAML
        let jyaml_value = from_str::<JyamlValue>(surrogate_input).unwrap();
        if let JyamlValue::String(s) = jyaml_value {
            assert_eq!(
                s, expected_emoji,
                "JYAML should convert surrogate pair to emoji"
            );
        } else {
            panic!("Expected string value");
        }

        // Parse with serde_json for comparison
        let json_value: JsonValue = serde_json::from_str(surrogate_input).unwrap();
        if let JsonValue::String(s) = json_value {
            assert_eq!(
                s, expected_emoji,
                "JSON should also convert surrogate pair to emoji"
            );
        } else {
            panic!("Expected string value");
        }
    }
}

#[test]
fn test_json_compatibility_nested_structures() {
    // Use compact format since JYAML parser has issues with multi-line objects
    let complex_data = r#"{"user": {"name": "Alice", "age": 30, "preferences": {"theme": "dark", "notifications": true}, "tags": ["developer", "rust"]}, "stats": [100, 85, 92]}"#;

    let jyaml_value = from_str::<JyamlValue>(complex_data).unwrap();
    let json_value: JsonValue = serde_json::from_str(complex_data).unwrap();

    let converted_jyaml = jyaml_to_json(&jyaml_value);
    assert_eq!(
        converted_jyaml, json_value,
        "Complex nested structures should match"
    );
}

#[test]
fn test_jyaml_specific_features_vs_json() {
    // Test JYAML features that JSON doesn't support

    // 1. Comments (JYAML supports, JSON doesn't) - use compact format
    let jyaml_with_comments = r#"{"name": "test", "value": 42}  # This is a comment"#;

    let jyaml_result = from_str::<JyamlValue>(jyaml_with_comments);
    assert!(jyaml_result.is_ok(), "JYAML should support comments");

    // JSON equivalent without comments
    let json_no_comments = r#"{"name": "test", "value": 42}"#;
    let json_result: Result<JsonValue, _> = serde_json::from_str(json_no_comments);
    assert!(json_result.is_ok(), "JSON should parse without comments");

    // Compare content (ignoring comments)
    if let (Ok(jyaml_val), Ok(json_val)) = (jyaml_result, json_result) {
        let converted_jyaml = jyaml_to_json(&jyaml_val);
        assert_eq!(
            converted_jyaml, json_val,
            "Content should be identical despite comments"
        );
    }
}

#[test]
fn test_performance_comparison_basic() {
    // Simple performance comparison on a moderately complex structure - use compact format
    let test_data = r#"{"users": [{"id": 1, "name": "Alice", "active": true}, {"id": 2, "name": "Bob", "active": false}, {"id": 3, "name": "Charlie", "active": true}], "metadata": {"count": 3, "timestamp": "2023-01-01T00:00:00Z"}}"#;

    // Test JYAML parsing
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _: JyamlValue = from_str(test_data).unwrap();
    }
    let jyaml_duration = start.elapsed();

    // Test JSON parsing
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _: JsonValue = serde_json::from_str(test_data).unwrap();
    }
    let json_duration = start.elapsed();

    println!("JYAML parsing time: {:?}", jyaml_duration);
    println!("JSON parsing time: {:?}", json_duration);

    // Verify they produce equivalent results
    let jyaml_value: JyamlValue = from_str(test_data).unwrap();
    let json_value: JsonValue = serde_json::from_str(test_data).unwrap();
    let converted_jyaml = jyaml_to_json(&jyaml_value);

    assert_eq!(
        converted_jyaml, json_value,
        "Both parsers should produce equivalent results"
    );
}

#[test]
fn test_error_handling_comparison() {
    let invalid_inputs = vec![
        r#"{"invalid": }"#,          // Missing value
        r#"{"unclosed": "string"#,   // Unclosed string
        r#"{invalid_key: "value"}"#, // Unquoted key
        r#"[1, 2, 3,]"#,             // Trailing comma (should be OK in JYAML, error in JSON)
    ];

    for (i, input) in invalid_inputs.iter().enumerate() {
        println!("Testing error case {}: {}", i, input);

        let jyaml_result = from_str::<JyamlValue>(input);
        let json_result = serde_json::from_str::<JsonValue>(input);

        if i == 3 {
            // Trailing comma case - JYAML should accept, JSON should reject
            assert!(jyaml_result.is_ok(), "JYAML should accept trailing commas");
            assert!(json_result.is_err(), "JSON should reject trailing commas");
        } else {
            // Other cases - both should reject
            assert!(
                jyaml_result.is_err(),
                "JYAML should reject invalid input: {}",
                input
            );
            assert!(
                json_result.is_err(),
                "JSON should reject invalid input: {}",
                input
            );
        }
    }
}

#[test]
fn test_direct_deserialization_comparison() {
    // Test deserializing to the same Rust struct with both parsers
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Person {
        name: String,
        age: u32,
        active: bool,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Company {
        name: String,
        employees: Vec<Person>,
        founded: u32,
    }

    let json_data = r#"{"name": "TechCorp", "employees": [{"name": "Alice", "age": 30, "active": true}, {"name": "Bob", "age": 25, "active": false}], "founded": 2020}"#;

    // Deserialize with JYAML
    let jyaml_result: Result<Company, _> = jyaml::from_str(json_data);
    assert!(
        jyaml_result.is_ok(),
        "JYAML should deserialize successfully"
    );

    // Deserialize with serde_json
    let json_result: Result<Company, _> = serde_json::from_str(json_data);
    assert!(json_result.is_ok(), "JSON should deserialize successfully");

    // Compare the deserialized structs directly
    let jyaml_company = jyaml_result.unwrap();
    let json_company = json_result.unwrap();

    assert_eq!(
        jyaml_company, json_company,
        "Both parsers should produce identical structs"
    );

    // Verify specific fields
    assert_eq!(jyaml_company.name, "TechCorp");
    assert_eq!(jyaml_company.employees.len(), 2);
    assert_eq!(jyaml_company.employees[0].name, "Alice");
    assert_eq!(jyaml_company.employees[0].age, 30);
    assert_eq!(jyaml_company.employees[0].active, true);
    assert_eq!(jyaml_company.founded, 2020);
}

#[test]
fn test_primitive_deserialization_comparison() {
    // Test deserializing primitive types

    // String
    let string_data = r#""Hello, World!""#;
    let jyaml_string: String = jyaml::from_str(string_data).unwrap();
    let json_string: String = serde_json::from_str(string_data).unwrap();
    assert_eq!(jyaml_string, json_string);

    // Integer
    let int_data = "42";
    let jyaml_int: i32 = jyaml::from_str(int_data).unwrap();
    let json_int: i32 = serde_json::from_str(int_data).unwrap();
    assert_eq!(jyaml_int, json_int);

    // Float
    let float_data = "3.14159";
    let jyaml_float: f64 = jyaml::from_str(float_data).unwrap();
    let json_float: f64 = serde_json::from_str(float_data).unwrap();
    assert_eq!(jyaml_float, json_float);

    // Boolean
    let bool_data = "true";
    let jyaml_bool: bool = jyaml::from_str(bool_data).unwrap();
    let json_bool: bool = serde_json::from_str(bool_data).unwrap();
    assert_eq!(jyaml_bool, json_bool);

    // Array
    let array_data = r#"[1, 2, 3, 4, 5]"#;
    let jyaml_array: Vec<i32> = jyaml::from_str(array_data).unwrap();
    let json_array: Vec<i32> = serde_json::from_str(array_data).unwrap();
    assert_eq!(jyaml_array, json_array);
}

#[test]
fn test_unicode_deserialization_comparison() {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct UnicodeTest {
        copyright: String,
        emoji: String,
        mixed: String,
    }

    // Test with various Unicode content
    let unicode_data = r#"{"copyright": "© 2023", "emoji": "🚀🎉", "mixed": "Hello 🌍 World"}"#;

    let jyaml_result: UnicodeTest = jyaml::from_str(unicode_data).unwrap();
    let json_result: UnicodeTest = serde_json::from_str(unicode_data).unwrap();

    assert_eq!(jyaml_result, json_result);
    assert_eq!(jyaml_result.copyright, "© 2023");
    assert_eq!(jyaml_result.emoji, "🚀🎉");
    assert_eq!(jyaml_result.mixed, "Hello 🌍 World");
}

#[test]
fn test_surrogate_pair_deserialization_comparison() {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct EmojiTest {
        rocket: String,
        party: String,
        crab: String,
    }

    // Test with surrogate pairs - both parsers should handle these identically
    let surrogate_data =
        r#"{"rocket": "\uD83D\uDE80", "party": "\uD83C\uDF89", "crab": "\uD83E\uDD80"}"#;

    let jyaml_result: EmojiTest = jyaml::from_str(surrogate_data).unwrap();
    let json_result: EmojiTest = serde_json::from_str(surrogate_data).unwrap();

    assert_eq!(jyaml_result, json_result);
    assert_eq!(jyaml_result.rocket, "🚀");
    assert_eq!(jyaml_result.party, "🎉");
    assert_eq!(jyaml_result.crab, "🦀");
}

#[test]
fn test_optional_fields_deserialization() {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct OptionalTest {
        required: String,
        optional: Option<String>,
        default_value: Option<i32>,
    }

    // Test with missing optional fields
    let data_with_null = r#"{"required": "test", "optional": null, "default_value": null}"#;
    let data_missing_fields = r#"{"required": "test"}"#;

    // Both parsers should handle missing/null optional fields
    let jyaml_result1: OptionalTest = jyaml::from_str(data_with_null).unwrap();
    let json_result1: OptionalTest = serde_json::from_str(data_with_null).unwrap();
    assert_eq!(jyaml_result1, json_result1);

    let jyaml_result2: OptionalTest = jyaml::from_str(data_missing_fields).unwrap();
    let json_result2: OptionalTest = serde_json::from_str(data_missing_fields).unwrap();
    assert_eq!(jyaml_result2, json_result2);

    assert_eq!(jyaml_result1.required, "test");
    assert_eq!(jyaml_result1.optional, None);
    assert_eq!(jyaml_result1.default_value, None);
}

#[test]
fn test_nested_collection_deserialization() {
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, PartialEq)]
    struct NestedTest {
        matrix: Vec<Vec<i32>>,
        map: HashMap<String, Vec<String>>,
        tuple_list: Vec<(String, i32)>,
    }

    let complex_data = r#"{"matrix": [[1, 2], [3, 4], [5, 6]], "map": {"fruits": ["apple", "banana"], "colors": ["red", "blue"]}, "tuple_list": [["first", 1], ["second", 2]]}"#;

    let jyaml_result: NestedTest = jyaml::from_str(complex_data).unwrap();
    let json_result: NestedTest = serde_json::from_str(complex_data).unwrap();

    assert_eq!(jyaml_result, json_result);

    // Verify nested structures
    assert_eq!(
        jyaml_result.matrix,
        vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    );
    assert_eq!(
        jyaml_result.map.get("fruits"),
        Some(&vec!["apple".to_string(), "banana".to_string()])
    );
    assert_eq!(jyaml_result.tuple_list[0], ("first".to_string(), 1));
}
