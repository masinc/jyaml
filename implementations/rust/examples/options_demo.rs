//! Comprehensive demonstration of JYAML options system
//!
//! This example shows how to use the enhanced options system for both
//! parsing and serialization, including presets, builders, and validation.

use jyaml::{
    from_str_with_options, to_string_with_options,
    DeserializeOptions, SerializeOptions,
    OutputStyle, QuoteStyle, LineEnding,
    from_str, to_string
};
use std::collections::HashMap;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 JYAML Options System Demo");
    println!("============================\n");

    // Sample JYAML data
    let sample_jyaml = r#"
# Configuration file
{
  "name": "Alice",
  "age": 30,
  "city": "Tokyo",
  "hobbies": ["reading", "cycling", "photography"],
  "settings": {
    "theme": "dark",
    "notifications": true,
    "language": "en"
  }
}
"#;

    let duplicate_keys_jyaml = r#"
{
  "name": "Alice",
  "name": "Bob",  # Duplicate key
  "age": 25
}
"#;

    demo_deserialize_presets(sample_jyaml, duplicate_keys_jyaml)?;
    demo_serialize_presets()?;
    demo_custom_options(sample_jyaml)?;
    demo_validation()?;
    demo_real_world_scenarios()?;

    Ok(())
}

fn demo_deserialize_presets(sample: &str, duplicates: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 DeserializeOptions Presets Demo");
    println!("----------------------------------");

    // Strict mode (default)
    println!("1. Strict mode:");
    let strict = DeserializeOptions::strict();
    println!("   Config: strict_mode={}, preserve_comments={}, max_depth={}",
             strict.strict_mode, strict.preserve_comments, strict.max_depth);
    
    match from_str_with_options::<Value>(duplicates, &strict) {
        Ok(_) => println!("   ✅ Parsed duplicate keys (unexpected)"),
        Err(e) => println!("   ❌ Rejected duplicate keys: {}", e),
    }

    // Permissive mode
    println!("\n2. Permissive mode:");
    let permissive = DeserializeOptions::permissive();
    println!("   Config: strict_mode={}, allow_duplicate_keys={}, max_depth={}",
             permissive.strict_mode, permissive.allow_duplicate_keys, permissive.max_depth);
    
    match from_str_with_options::<Value>(duplicates, &permissive) {
        Ok(value) => println!("   ✅ Parsed with duplicates: {}", value),
        Err(e) => println!("   ❌ Failed: {}", e),
    }

    // Fast mode
    println!("\n3. Fast mode:");
    let fast = DeserializeOptions::fast();
    println!("   Config: preserve_comments={}, max_depth={}",
             fast.preserve_comments, fast.max_depth);
    
    let _result: Value = from_str_with_options(sample, &fast)?;
    println!("   ✅ Fast parsing completed");

    // Debug mode
    println!("\n4. Debug mode:");
    let debug = DeserializeOptions::debug();
    println!("   Config: include_comment_positions={}, allow_duplicate_keys={}",
             debug.include_comment_positions, debug.allow_duplicate_keys);
    
    let _result: Value = from_str_with_options(sample, &debug)?;
    println!("   ✅ Debug parsing with full information");

    // From preset by name
    println!("\n5. From preset name:");
    let from_name = DeserializeOptions::from_preset("debug")?;
    println!("   Created debug preset: {}", from_name.is_preserving());

    println!();
    Ok(())
}

fn demo_serialize_presets() -> Result<(), Box<dyn std::error::Error>> {
    println!("📤 SerializeOptions Presets Demo");
    println!("--------------------------------");

    let data: HashMap<&str, Value> = [
        ("name", Value::String("Alice".to_string())),
        ("age", Value::Number(30.into())),
        ("active", Value::Bool(true)),
        ("scores", Value::Array(vec![
            Value::Number(95.into()),
            Value::Number(87.into()),
            Value::Number(92.into())
        ])),
    ].into_iter().collect();

    // Compact output
    println!("1. Compact output:");
    let compact = SerializeOptions::compact();
    let result = to_string_with_options(&data, &compact)?;
    println!("   {}", result);
    println!("   Compact: {}", compact.is_compact());

    // Pretty output
    println!("\n2. Pretty output:");
    let pretty = SerializeOptions::pretty();
    let result = to_string_with_options(&data, &pretty)?;
    println!("{}", indent_text(&result, "   "));
    println!("   Readable: {}", pretty.is_readable());

    // Block style
    println!("3. Block style:");
    let block = SerializeOptions::block();
    let result = to_string_with_options(&data, &block)?;
    println!("{}", indent_text(&result, "   "));

    // JSON compatible
    println!("4. JSON compatible:");
    let json_compat = SerializeOptions::json_compatible();
    let result = to_string_with_options(&data, &json_compat)?;
    println!("   {}", result);

    // Debug output
    println!("\n5. Debug output:");
    let debug = SerializeOptions::debug();
    let result = to_string_with_options(&data, &debug)?;
    println!("{}", indent_text(&result, "   "));

    // From preset by name
    println!("6. From preset name:");
    let from_name = SerializeOptions::from_preset("debug")?;
    println!("   Created debug preset with indent: {}", from_name.indent);

    println!();
    Ok(())
}

fn demo_custom_options(sample: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔨 Custom Options with Builders");
    println!("-------------------------------");

    // Custom deserialize options
    println!("1. Custom DeserializeOptions:");
    let custom_parse = DeserializeOptions::builder()
        .strict_mode(false)
        .max_depth(50)
        .allow_duplicate_keys(true)
        .preserve_comments(true)
        .include_comment_positions(false)
        .normalize_line_endings(LineEnding::Lf)
        .build();
    
    println!("   Built custom options: max_depth={}, is_strict={}",
             custom_parse.max_depth, custom_parse.is_strict());

    let _result: Value = from_str_with_options(sample, &custom_parse)?;
    println!("   ✅ Parsed with custom options");

    // Custom serialize options
    println!("\n2. Custom SerializeOptions:");
    let custom_serialize = SerializeOptions::builder()
        .style(OutputStyle::Block)
        .pretty(true)
        .indent(4)
        .sort_keys(true)
        .quote_style(QuoteStyle::Double)
        .escape_unicode(false)
        .line_ending(LineEnding::Lf)
        .build();

    println!("   Built custom options: indent={}, readable={}",
             custom_serialize.indent, custom_serialize.is_readable());

    let simple_data: HashMap<&str, &str> = [
        ("name", "Bob"),
        ("city", "Osaka"),
        ("status", "active"),
    ].into_iter().collect();

    let result = to_string_with_options(&simple_data, &custom_serialize)?;
    println!("   Custom serialized output:");
    println!("{}", indent_text(&result, "   "));

    // With validation
    println!("3. Builder with validation:");
    match SerializeOptions::builder()
        .indent(4)
        .pretty(true)
        .sort_keys(true)
        .try_build()
    {
        Ok(opts) => println!("   ✅ Valid options built successfully"),
        Err(e) => println!("   ❌ Validation failed: {}", e),
    }

    match SerializeOptions::builder()
        .indent(15)  // Invalid
        .try_build()
    {
        Ok(_) => println!("   ❌ Invalid options accepted (unexpected)"),
        Err(e) => println!("   ✅ Invalid options rejected: {}", e),
    }

    println!();
    Ok(())
}

fn demo_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("✅ Options Validation Demo");
    println!("-------------------------");

    // Valid options
    println!("1. Valid option combinations:");
    let valid = DeserializeOptions::default();
    match valid.validate() {
        Ok(_) => println!("   ✅ Default options are valid"),
        Err(e) => println!("   ❌ Validation failed: {}", e),
    }

    // Invalid combinations
    println!("\n2. Invalid option combinations:");
    
    // Strict + duplicate keys
    let invalid1 = DeserializeOptions {
        strict_mode: true,
        allow_duplicate_keys: true,
        ..Default::default()
    };
    match invalid1.validate() {
        Ok(_) => println!("   ❌ Invalid combo accepted (unexpected)"),
        Err(e) => println!("   ✅ Rejected strict + duplicates: {}", e),
    }

    // Comment positions without comments
    let invalid2 = DeserializeOptions {
        preserve_comments: false,
        include_comment_positions: true,
        ..Default::default()
    };
    match invalid2.validate() {
        Ok(_) => println!("   ❌ Invalid combo accepted (unexpected)"),
        Err(e) => println!("   ✅ Rejected positions without comments: {}", e),
    }

    // Invalid depth
    let invalid3 = SerializeOptions {
        indent: 10,
        ..Default::default()
    };
    match invalid3.validate() {
        Ok(_) => println!("   ❌ Invalid indent accepted (unexpected)"),
        Err(e) => println!("   ✅ Rejected large indent: {}", e),
    }

    println!();
    Ok(())
}

fn demo_real_world_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Real-World Usage Scenarios");
    println!("-----------------------------");

    let config_data = r#"
# Application Configuration
{
  "app": {
    "name": "MyService",
    "version": "1.2.3",
    "debug": false
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "timeout": 30.0,
    "pool_size": 10
  }
}
"#;

    // Configuration file processing
    println!("1. Configuration file processing:");
    
    // Development mode (preserve comments, permissive)
    let dev_opts = DeserializeOptions::permissive();
    let dev_data: Value = from_str_with_options(config_data, &dev_opts)?;
    println!("   ✅ Development mode parsing (permissive)");

    // Production mode (strict, fast)
    let prod_opts = DeserializeOptions::fast();
    let _prod_data: Value = from_str_with_options(config_data, &prod_opts)?;
    println!("   ✅ Production mode parsing (fast & strict)");

    // API response formatting
    println!("\n2. API response formatting:");
    
    let api_data: HashMap<&str, Value> = [
        ("status", Value::String("success".to_string())),
        ("data", Value::Object([
            ("count".to_string(), Value::Number(42.into())),
            ("results".to_string(), Value::Array(vec![
                Value::String("item1".to_string()),
                Value::String("item2".to_string()),
            ])),
        ].into_iter().collect())),
    ].into_iter().collect();

    // Compact for network transmission
    let api_compact = SerializeOptions::compact();
    let compact_result = to_string_with_options(&api_data, &api_compact)?;
    println!("   Compact API response: {}", compact_result);

    // Pretty for debugging
    let api_debug = SerializeOptions::debug();
    let debug_result = to_string_with_options(&api_data, &api_debug)?;
    println!("   Debug API response:");
    println!("{}", indent_text(&debug_result, "   "));

    // Configuration file output
    println!("3. Configuration file output:");
    let config_opts = SerializeOptions::builder()
        .style(OutputStyle::Block)
        .pretty(true)
        .indent(2)
        .sort_keys(true)
        .line_ending(LineEnding::Lf)
        .build();

    let config_output = to_string_with_options(&dev_data, &config_opts)?;
    println!("   Config file format:");
    println!("{}", indent_text(&config_output, "   "));

    println!();
    Ok(())
}

fn indent_text(text: &str, prefix: &str) -> String {
    text.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}