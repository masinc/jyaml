use jyaml::{parse, Value};
use pretty_assertions::assert_eq;

#[test]
fn test_invalid_unicode_surrogate_pairs() {
    // Invalid surrogate pair combinations that should error
    let test_cases = vec![
        r#""\uD800\uDBFF""#,  // Two high surrogates
        r#""\uDC00\uDFFF""#,  // Two low surrogates  
        r#""\uD800""#,        // Lone high surrogate
        r#""\uDC00""#,        // Lone low surrogate
        r#""\uD800\u0041""#,  // High surrogate followed by regular char
        r#""\u0041\uDC00""#,  // Regular char followed by low surrogate
        r#""\uDBFF\uE000""#,  // High surrogate + non-surrogate
    ];
    
    for jyaml in test_cases {
        let result = parse(jyaml);
        assert!(result.is_err(), "Should error for invalid surrogate: {}", jyaml);
        
        if let Err(err) = result {
            let error_msg = format!("{}", err);
            assert!(
                error_msg.contains("surrogate") || 
                error_msg.contains("unicode") ||
                error_msg.contains("UTF") ||
                error_msg.contains("invalid"),
                "Error should mention unicode/surrogate issue: {}", error_msg
            );
        }
    }
}

#[test]
fn test_valid_unicode_surrogate_pairs() {
    // Valid surrogate pairs that should parse correctly
    let test_cases = vec![
        (r#""\uD83D\uDE00""#, "😀"),  // Grinning face emoji
        (r#""\uD83C\uDF89""#, "🎉"),  // Party popper emoji
        (r#""\uD834\uDD1E""#, "𝄞"),   // Musical symbol treble clef
        (r#""\uD800\uDC00""#, "𐀀"),   // Linear B syllable B008 A
    ];
    
    for (jyaml, expected) in test_cases {
        let value = parse(jyaml).unwrap();
        assert_eq!(value, Value::String(expected.to_string()));
    }
}

#[test]
fn test_unicode_in_object_keys() {
    // Unicode characters in object keys
    let jyaml = r#"
{
  "🔑": "emoji key",
  "ключ": "cyrillic key", 
  "鍵": "japanese key",
  "clé": "french key",
  "\u0041\u0300": "A with grave accent"
}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("🔑"), Some(&Value::String("emoji key".to_string())));
        assert_eq!(obj.get("ключ"), Some(&Value::String("cyrillic key".to_string())));
        assert_eq!(obj.get("鍵"), Some(&Value::String("japanese key".to_string())));
        assert_eq!(obj.get("clé"), Some(&Value::String("french key".to_string())));
        assert_eq!(obj.get("À"), Some(&Value::String("A with grave accent".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_unicode_normalization_edge_cases() {
    // Different Unicode normalization forms (should be treated as different keys)
    let jyaml = r#"
{
  "é": "precomposed",
  "e\u0301": "decomposed"
}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        // These should be treated as different keys (no normalization)
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("é"), Some(&Value::String("precomposed".to_string())));
        assert_eq!(obj.get("é"), Some(&Value::String("decomposed".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_control_characters_in_strings() {
    // Control characters that should error when unescaped
    let invalid_cases = vec![
        "\"\u{0000}\"",  // NULL character
        "\"\u{0001}\"",  // Start of heading  
        "\"\u{0007}\"",  // Bell
        "\"\u{000B}\"",  // Vertical tab
        "\"\u{000C}\"",  // Form feed
        "\"\u{000E}\"",  // Shift out
        "\"\u{001F}\"",  // Unit separator
        "\"\u{007F}\"",  // Delete
    ];
    
    for jyaml in invalid_cases {
        let result = parse(jyaml);
        assert!(result.is_err(), "Should error for unescaped control char: {:?}", jyaml);
    }
}

#[test]
fn test_escaped_control_characters() {
    // Properly escaped control characters should work
    let jyaml = r#"
{
  "null": "\u0000",
  "bell": "\u0007", 
  "tab": "\t",
  "newline": "\n",
  "carriage_return": "\r",
  "backslash": "\\",
  "quote": "\""
}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("null"), Some(&Value::String("\u{0000}".to_string())));
        assert_eq!(obj.get("bell"), Some(&Value::String("\u{0007}".to_string())));
        assert_eq!(obj.get("tab"), Some(&Value::String("\t".to_string())));
        assert_eq!(obj.get("newline"), Some(&Value::String("\n".to_string())));
        assert_eq!(obj.get("carriage_return"), Some(&Value::String("\r".to_string())));
        assert_eq!(obj.get("backslash"), Some(&Value::String("\\".to_string())));
        assert_eq!(obj.get("quote"), Some(&Value::String("\"".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_mixed_unicode_and_ascii() {
    // Mix of Unicode and ASCII characters
    let jyaml = r#"
{
  "mixed": "Hello 世界! 🌍 Welcome to ünïcöde testing",
  "numbers": "123 ४५६ ৭৮৯ 일이삼",
  "symbols": "© ® ™ € £ ¥ § ¶ † ‡ • …",
  "math": "∞ ∑ ∏ ∆ ∇ ∂ ∫ √ ∝ ∴",
  "arrows": "← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙"
}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("mixed"), Some(&Value::String("Hello 世界! 🌍 Welcome to ünïcöde testing".to_string())));
        assert_eq!(obj.get("numbers"), Some(&Value::String("123 ४५६ ৭৮৯ 일이삼".to_string())));
        assert_eq!(obj.get("symbols"), Some(&Value::String("© ® ™ € £ ¥ § ¶ † ‡ • …".to_string())));
        assert_eq!(obj.get("math"), Some(&Value::String("∞ ∑ ∏ ∆ ∇ ∂ ∫ √ ∝ ∴".to_string())));
        assert_eq!(obj.get("arrows"), Some(&Value::String("← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_unicode_byte_order_mark() {
    // BOM (Byte Order Mark) should be rejected
    let jyaml_with_bom = "\u{FEFF}{\"key\": \"value\"}";
    let result = parse(jyaml_with_bom);
    assert!(result.is_err(), "BOM should be rejected");
    
    if let Err(err) = result {
        let error_msg = format!("{}", err);
        assert!(
            error_msg.contains("BOM") || 
            error_msg.contains("byte order mark") ||
            error_msg.contains("FEFF"),
            "Error should mention BOM: {}", error_msg
        );
    }
}

#[test]
fn test_invalid_unicode_escape_sequences() {
    // Invalid Unicode escape sequences
    let test_cases = vec![
        r#""\u""#,           // Incomplete escape
        r#""\u123""#,        // Too few hex digits
        r#""\u12GH""#,       // Invalid hex characters
        r#""\uG123""#,       // Invalid hex characters
        r#""\U00000041""#,   // Capital U (not supported in JSON/JYAML)
    ];
    
    for jyaml in test_cases {
        let result = parse(jyaml);
        assert!(result.is_err(), "Should error for invalid unicode escape: {}", jyaml);
    }
}

#[test]
fn test_unicode_in_multiline_strings() {
    // Unicode in literal and folded multiline strings
    let jyaml = r#"
"literal_unicode": |
  こんにちは世界
  🌍 Hello World
  Привет мир
"folded_unicode": >
  This is a folded string
  with unicode: ñoño 中文
  and emojis: 🎉 🚀 💖
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        let expected_literal = "こんにちは世界\n🌍 Hello World\nПривет мир\n";
        assert_eq!(obj.get("literal_unicode"), Some(&Value::String(expected_literal.to_string())));
        
        let expected_folded = "This is a folded string with unicode: ñoño 中文 and emojis: 🎉 🚀 💖\n";
        assert_eq!(obj.get("folded_unicode"), Some(&Value::String(expected_folded.to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_unicode_comparison_and_sorting() {
    // Test that Unicode strings compare correctly
    let jyaml = r#"
{
  "α": "alpha",
  "β": "beta", 
  "ζ": "zeta",
  "Α": "Alpha",
  "1": "one",
  "²": "squared"
}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        // Just verify all keys are present and distinct
        assert_eq!(obj.len(), 6);
        assert!(obj.contains_key("α"));
        assert!(obj.contains_key("β"));
        assert!(obj.contains_key("ζ"));
        assert!(obj.contains_key("Α"));  // Different from lowercase α
        assert!(obj.contains_key("1"));
        assert!(obj.contains_key("²"));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_unicode_edge_cases_in_arrays() {
    // Unicode edge cases in array context
    let jyaml = r#"
[
  "🎯",
  "\uD83C\uDFAF",  # Same emoji as above, but escaped
  "café",
  "cafe\u0301",    # Different normalization
  "",              # Empty string
  " ",             # Space
  "\u0020",        # Escaped space
  "🔥🔥🔥"        # Multiple emojis
]
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Array(arr) = value {
        assert_eq!(arr.len(), 8);
        assert_eq!(arr[0], Value::String("🎯".to_string()));
        assert_eq!(arr[1], Value::String("🎯".to_string()));  // Same as [0]
        assert_eq!(arr[2], Value::String("café".to_string()));
        assert_eq!(arr[3], Value::String("café".to_string()));  // Different normalization but same result
        assert_eq!(arr[4], Value::String("".to_string()));
        assert_eq!(arr[5], Value::String(" ".to_string()));
        assert_eq!(arr[6], Value::String(" ".to_string()));    // Same as [5]
        assert_eq!(arr[7], Value::String("🔥🔥🔥".to_string()));
    } else {
        panic!("Expected array");
    }
}