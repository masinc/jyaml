#[cfg(test)]
mod multiline_tests {
    use crate::parse;

    #[test]
    fn test_literal_string_simple() {
        let input = r#""literal": |
  Line 1"#;
        
        match parse(input) {
            Ok(value) => {
                println!("Parsed: {:?}", value);
                // Check that we got the expected content
                if let Some(crate::Value::Object(obj)) = Some(value) {
                    if let Some(crate::Value::String(s)) = obj.get("literal") {
                        println!("Literal content: '{}'", s);
                        assert!(s.contains("Line 1"), "Expected 'Line 1' in content, got: '{}'", s);
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                panic!("Failed to parse literal string: {}", e);
            }
        }
    }

    #[test]
    fn test_folded_string_simple() {
        let input = r#""folded": >
  This is text"#;
        
        match parse(input) {
            Ok(value) => println!("Parsed: {:?}", value),
            Err(e) => {
                println!("Error: {}", e);
                panic!("Failed to parse folded string: {}", e);
            }
        }
    }

    #[test]
    fn test_folded_keep() {
        let input = r#""folded_keep": >+
  Keep trailing
  newlines

"#;
        
        match parse(input) {
            Ok(value) => {
                println!("Parsed: {:?}", value);
                if let Some(crate::Value::Object(obj)) = Some(value) {
                    if let Some(crate::Value::String(s)) = obj.get("folded_keep") {
                        println!("Folded content: '{}'", s);
                        println!("Folded content (debug): {:?}", s);
                        // Expected: "Keep trailing\nnewlines\n\n"
                        assert_eq!(s, "Keep trailing\nnewlines\n\n", "Expected folded_keep to have proper formatting");
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                panic!("Failed to parse folded keep string: {}", e);
            }
        }
    }

}