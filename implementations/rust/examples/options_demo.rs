use jyaml::{to_string_with_options, SerializeOptions, OutputStyle, QuoteStyle, Value};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    email: String,
    tags: Vec<String>,
    settings: HashMap<String, Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample data
    let mut settings = HashMap::new();
    settings.insert("theme".to_string(), Value::String("dark".to_string()));
    settings.insert("notifications".to_string(), Value::Bool(true));
    settings.insert("language".to_string(), Value::String("en".to_string()));
    
    let person = Person {
        name: "Alice Johnson".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
        tags: vec!["developer".to_string(), "rust".to_string(), "jyaml".to_string()],
        settings,
    };

    println!("=== JYAML Options Demo ===\n");

    // 1. Compact format
    println!("1. Compact format:");
    let compact = SerializeOptions::compact();
    let result = to_string_with_options(&person, &compact)?;
    println!("{}\n", result);

    // 2. Pretty format
    println!("2. Pretty format:");
    let pretty = SerializeOptions::pretty();
    let result = to_string_with_options(&person, &pretty)?;
    println!("{}\n", result);

    // 3. Block style (YAML-like)
    println!("3. Block style (YAML-like):");
    let block = SerializeOptions::block();
    let result = to_string_with_options(&person, &block)?;
    println!("{}\n", result);

    // 4. JSON-compatible format
    println!("4. JSON-compatible format:");
    let json_compat = SerializeOptions::json_compatible();
    let result = to_string_with_options(&person, &json_compat)?;
    println!("{}\n", result);

    // 5. Custom options
    println!("5. Custom options (large indent, sorted keys):");
    let custom = SerializeOptions::default()
        .with_style(OutputStyle::Block)
        .with_indent(4)?
        .with_sort_keys(true)
        .with_pretty(true);
    let result = to_string_with_options(&person, &custom)?;
    println!("{}\n", result);

    // 6. Unicode handling
    println!("6. Unicode handling:");
    let unicode_data = Value::String("Hello 🌍 World © 2023 🦀".to_string());
    
    println!("  Without Unicode escaping:");
    let no_escape = SerializeOptions::default().with_escape_unicode(false);
    let result = to_string_with_options(&unicode_data, &no_escape)?;
    println!("  {}\n", result);
    
    println!("  With Unicode escaping:");
    let with_escape = SerializeOptions::default().with_escape_unicode(true);
    let result = to_string_with_options(&unicode_data, &with_escape)?;
    println!("  {}\n", result);

    // 7. Builder pattern examples
    println!("7. Builder pattern examples:");
    
    let builder_example = SerializeOptions::default()
        .with_style(OutputStyle::Auto)
        .with_quote_style(QuoteStyle::Double)
        .with_pretty(true)
        .with_indent(2)?
        .with_sort_keys(false);
        
    let result = to_string_with_options(&person, &builder_example)?;
    println!("{}\n", result);

    println!("=== All options work correctly! ===");
    Ok(())
}