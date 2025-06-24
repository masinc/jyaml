use jyaml::parse;

fn main() {
    println!("=== JYAML Error Handling Examples ===\n");

    // Example 1: Syntax error
    println!("1. Syntax error (unclosed string):");
    let bad1 = r#""unclosed string"#;
    match parse(bad1) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 2: Invalid number format
    println!("2. Invalid number format (leading zero):");
    let bad2 = "01234";
    match parse(bad2) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 3: Tab in indentation
    println!("3. Tab character in indentation:");
    let bad3 = "\tindented with tab";
    match parse(bad3) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 4: Trailing comma in array
    println!("4. Trailing comma in array:");
    let bad4 = "[1, 2, 3,]";
    match parse(bad4) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 5: Trailing comma in object
    println!("5. Trailing comma in object:");
    let bad5 = r#"{"name": "test", "age": 30,}"#;
    match parse(bad5) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 6: Duplicate keys
    println!("6. Duplicate keys in object:");
    let bad6 = r#"
"name": "first"
"age": 25
"name": "second"
"#;
    match parse(bad6) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 7: Invalid escape sequence
    println!("7. Invalid escape sequence:");
    let bad7 = r#""invalid \x escape""#;
    match parse(bad7) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 8: Invalid Unicode escape
    println!("8. Invalid Unicode escape:");
    let bad8 = r#""invalid \uGGGG""#;
    match parse(bad8) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 9: BOM at beginning
    println!("9. BOM (Byte Order Mark) at beginning:");
    let bad9 = "\u{FEFF}test";
    match parse(bad9) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 10: Invalid identifier
    println!("10. Invalid identifier:");
    let bad10 = "yes";  // YAML allows 'yes' as true, but JYAML doesn't
    match parse(bad10) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 11: Missing colon in object
    println!("11. Missing colon in object:");
    let bad11 = r#"{"name" "value"}"#;
    match parse(bad11) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 12: Unescaped control character
    println!("12. Unescaped control character:");
    let bad12 = "\"line1\nline2\"";  // Unescaped newline
    match parse(bad12) {
        Ok(value) => println!("  Unexpected success: {:?}", value),
        Err(e) => println!("  Error: {}\n", e),
    }

    println!("=== Error handling examples completed! ===");
}