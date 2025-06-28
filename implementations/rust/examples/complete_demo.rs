use jyaml::{
    from_str, from_str_with_options, parse, parse_with_options, to_string, to_string_pretty,
    to_string_with_options, DeserializeOptions, DeserializeOptionsBuilder, LineEnding, OutputStyle,
    QuoteStyle, SerializeOptions, SerializeOptionsBuilder, Value,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
    tags: Vec<String>,
    settings: HashMap<String, String>,
}

fn create_sample_data() -> User {
    let mut settings = HashMap::new();
    settings.insert("theme".to_string(), "dark".to_string());
    settings.insert("language".to_string(), "en".to_string());
    settings.insert("notifications".to_string(), "enabled".to_string());

    User {
        name: "Alice Johnson".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        active: true,
        tags: vec![
            "developer".to_string(),
            "rust".to_string(),
            "jyaml".to_string(),
        ],
        settings,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JYAML Complete Demo ===\n");

    let user = create_sample_data();

    // 1. Basic serialization
    println!("1. Basic Serialization:");
    println!("   Compact: {}", to_string(&user)?);
    println!("   Pretty:\n{}", to_string_pretty(&user, 2)?);

    // 2. Serialization with options
    println!("\n2. Serialization with Options:");

    // Compact JSON-like
    let compact = SerializeOptions::compact();
    println!("   Compact: {}", to_string_with_options(&user, &compact)?);

    // Pretty YAML-like block style
    let block = SerializeOptions::block();
    println!("   Block:\n{}", to_string_with_options(&user, &block)?);

    // Custom options with builder
    let custom = SerializeOptions::builder()
        .style(OutputStyle::Auto)
        .indent(4)
        .quote_style(QuoteStyle::Double)
        .sort_keys(true)
        .escape_unicode(false)
        .pretty(true)
        .build();
    println!(
        "   Custom (sorted keys, 4-space indent):\n{}",
        to_string_with_options(&user, &custom)?
    );

    // 3. Deserialization
    println!("\n3. Deserialization:");

    let jyaml_text = to_string_pretty(&user, 2)?;
    println!("   Original JYAML:\n{}", jyaml_text);

    // Basic deserialization
    let parsed_user: User = from_str(&jyaml_text)?;
    println!("   Parsed successfully: {}", parsed_user.name);
    assert_eq!(user, parsed_user);

    // 4. Value parsing (low-level)
    println!("\n4. Value Parsing:");
    let value = parse(&jyaml_text)?;
    if let Value::Object(obj) = &value {
        println!("   Parsed as Value with {} keys", obj.len());
        if let Some(Value::String(name)) = obj.get("name") {
            println!("   Name from Value: {}", name);
        }
    }

    // 5. Deserialization with options
    println!("\n5. Deserialization with Options:");

    // Test with duplicate keys (should fail with strict mode)
    let duplicate_keys = r#""name": "Alice"
"age": 30
"name": "Bob""#;

    // Strict mode (default) - should fail
    let strict = DeserializeOptions::strict();
    match parse_with_options(duplicate_keys, &strict) {
        Ok(_) => println!("   Strict mode: Unexpected success"),
        Err(e) => println!("   Strict mode: Correctly rejected duplicate keys - {}", e),
    }

    // Permissive mode - should succeed
    let permissive = DeserializeOptions::permissive();
    match parse_with_options(duplicate_keys, &permissive) {
        Ok(value) => {
            if let Value::Object(obj) = value {
                if let Some(Value::String(name)) = obj.get("name") {
                    println!("   Permissive mode: Last value wins - name = {}", name);
                }
            }
        }
        Err(e) => println!("   Permissive mode: Unexpected error - {}", e),
    }

    // 6. Builder pattern examples
    println!("\n6. Builder Pattern Examples:");

    // Serialize options builder
    let serialize_opts = SerializeOptionsBuilder::new()
        .style(OutputStyle::Flow)
        .quote_style(QuoteStyle::Double)
        .sort_keys(true)
        .pretty(false)
        .try_build()?;

    println!(
        "   Builder serialization: {}",
        to_string_with_options(&user, &serialize_opts)?
    );

    // Deserialize options builder
    let deserialize_opts = DeserializeOptionsBuilder::new()
        .strict_mode(false)
        .allow_duplicate_keys(true)
        .max_depth(100)
        .preserve_comments(true)
        .build();

    let test_input = r#""name": "Test User"
"age": 25
"email": "test@example.com"
"active": true
"tags": []
"settings": {}"#;
    let test_result: User = from_str_with_options(test_input, &deserialize_opts)?;
    println!(
        "   Builder deserialization: {} (age: {})",
        test_result.name, test_result.age
    );

    // 7. Unicode and special characters
    println!("\n7. Unicode Handling:");

    let unicode_data = HashMap::from([
        ("english".to_string(), "Hello World".to_string()),
        ("japanese".to_string(), "こんにちは世界".to_string()),
        ("emoji".to_string(), "🌍🦀🚀".to_string()),
        ("symbols".to_string(), "© ® ™ € £".to_string()),
    ]);

    // Without Unicode escaping
    let no_escape = SerializeOptions::builder()
        .escape_unicode(false)
        .pretty(true)
        .build();
    println!(
        "   Without escaping:\n{}",
        to_string_with_options(&unicode_data, &no_escape)?
    );

    // With Unicode escaping
    let with_escape = SerializeOptions::builder()
        .escape_unicode(true)
        .pretty(true)
        .build();
    println!(
        "   With escaping:\n{}",
        to_string_with_options(&unicode_data, &with_escape)?
    );

    // 8. Line ending options
    println!("\n8. Line Ending Options:");

    let line_data = HashMap::from([
        ("line1".to_string(), "First line".to_string()),
        ("line2".to_string(), "Second line".to_string()),
    ]);

    // Different line endings
    let lf_opts = SerializeOptions::builder()
        .line_ending(LineEnding::Lf)
        .pretty(true)
        .build();
    let crlf_opts = SerializeOptions::builder()
        .line_ending(LineEnding::Crlf)
        .pretty(true)
        .build();
    let none_opts = SerializeOptions::builder()
        .line_ending(LineEnding::None)
        .pretty(true)
        .build();

    println!(
        "   LF endings: {:?}",
        to_string_with_options(&line_data, &lf_opts)?
            .chars()
            .filter(|&c| c == '\n' || c == '\r')
            .collect::<Vec<_>>()
    );
    println!(
        "   CRLF endings: {:?}",
        to_string_with_options(&line_data, &crlf_opts)?
            .chars()
            .filter(|&c| c == '\n' || c == '\r')
            .collect::<Vec<_>>()
    );
    println!(
        "   No normalization: {:?}",
        to_string_with_options(&line_data, &none_opts)?
            .chars()
            .filter(|&c| c == '\n' || c == '\r')
            .collect::<Vec<_>>()
    );

    // 9. Error handling
    println!("\n9. Error Handling:");

    // Invalid JYAML
    match from_str::<User>(r#""name": "Alice" "age":"#) {
        Ok(_) => println!("   Unexpected success parsing invalid JYAML"),
        Err(e) => println!("   Correctly caught parse error: {}", e),
    }

    // Invalid options
    match SerializeOptionsBuilder::new().indent(10).try_build() {
        Ok(_) => println!("   Unexpected success with invalid indent"),
        Err(e) => println!("   Correctly caught options error: {}", e),
    }

    // Max depth exceeded
    let deep_opts = DeserializeOptions::builder().max_depth(1).build();
    match parse_with_options(r#""level1": {"level2": "value"}"#, &deep_opts) {
        Ok(_) => println!("   Unexpected success with deep nesting"),
        Err(e) => println!("   Correctly caught depth error: {}", e),
    }

    println!("\n=== Demo completed successfully! ===");
    Ok(())
}
