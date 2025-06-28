use jyaml::{parse, Value};
use pretty_assertions::assert_eq;

#[test]
fn test_literal_string_empty_blocks() {
    // Empty literal block
    let jyaml = r#"
"empty": |

"next": "value"
"#;
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("empty"), Some(&Value::String("".to_string())));
        assert_eq!(obj.get("next"), Some(&Value::String("value".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_literal_string_with_chomping_indicators() {
    // Literal strip (|-) - remove final newlines
    let jyaml = r#"
"strip": |-
  Line 1
  Line 2
"keep": |+
  Line 1
  Line 2

"#;
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("strip"), Some(&Value::String("Line 1\nLine 2".to_string())));
        assert_eq!(obj.get("keep"), Some(&Value::String("Line 1\nLine 2\n\n".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_folded_string_paragraph_handling() {
    // Folded string with multiple paragraphs
    let jyaml = r#"
"description": >
  This is the first paragraph
  which continues on this line.
  
  This is the second paragraph
  starting after blank line.
  
  Final paragraph here.
"#;
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        let expected = "This is the first paragraph which continues on this line.\nThis is the second paragraph starting after blank line.\nFinal paragraph here.\n";
        assert_eq!(obj.get("description"), Some(&Value::String(expected.to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_folded_string_with_chomping() {
    // Folded with different chomping indicators
    let jyaml = r#"
"clip": >
  Folded text
  continues here
"strip": >-
  Folded text
  no final newline
"keep": >+
  Folded text
  keep newlines

"#;
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("clip"), Some(&Value::String("Folded text continues here\n".to_string())));
        assert_eq!(obj.get("strip"), Some(&Value::String("Folded text no final newline".to_string())));
        assert_eq!(obj.get("keep"), Some(&Value::String("Folded text keep newlines\n\n".to_string())));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_mixed_multiline_styles_in_document() {
    // Mix literal and folded strings in same document
    let jyaml = r#"
"literal": |
  Line 1
  Line 2
  Line 3
"folded": >
  This text
  will be folded
  into single line
"normal": "regular string"
"array":
  - |
    Array literal
    multiline
  - >
    Array folded
    multiline
"#;
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("literal"), Some(&Value::String("Line 1\nLine 2\nLine 3\n".to_string())));
        assert_eq!(obj.get("folded"), Some(&Value::String("This text will be folded into single line\n".to_string())));
        assert_eq!(obj.get("normal"), Some(&Value::String("regular string".to_string())));
        
        if let Some(Value::Array(arr)) = obj.get("array") {
            assert_eq!(arr[0], Value::String("Array literal\nmultiline\n".to_string()));
            assert_eq!(arr[1], Value::String("Array folded multiline\n".to_string()));
        } else {
            panic!("Expected array");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_multiline_string_indentation_errors() {
    // Inconsistent indentation in literal block should error
    let jyaml = r#"
"bad_literal": |
    Line 1
  Line 2
"#;
    let result = parse(jyaml);
    assert!(result.is_err());
    
    // Check that it's specifically an indentation error
    if let Err(err) = result {
        let error_msg = format!("{}", err);
        assert!(error_msg.contains("indentation") || error_msg.contains("indent"));
    }
}

#[test]
fn test_multiline_string_with_tabs_error() {
    // Tabs in multiline strings should error
    let jyaml = "\"literal\": |\n\tLine with tab";
    let result = parse(jyaml);
    assert!(result.is_err());
    
    if let Err(err) = result {
        let error_msg = format!("{}", err);
        assert!(error_msg.contains("tab") || error_msg.contains("Tab"));
    }
}

#[test]
fn test_malformed_multiline_indicators() {
    // Invalid multiline indicators
    let test_cases = vec![
        "\"key\": |invalid",  // Invalid chomping
        "\"key\": >2",        // Invalid explicit indent
        "\"key\": |++",       // Double chomping
        "\"key\": >-+",       // Conflicting chomping
    ];
    
    for jyaml in test_cases {
        let result = parse(jyaml);
        assert!(result.is_err(), "Should error for: {}", jyaml);
    }
}

#[test]
fn test_multiline_string_in_nested_structures() {
    // Literal strings in complex nested structures
    let jyaml = r#"
"config":
  "database":
    "query": |
      SELECT *
      FROM users
      WHERE active = true
    "description": >
      This is a long description
      that spans multiple lines
      and will be folded.
  "servers":
    - "name": "web1"
      "config": |
        server {
          listen 80;
          root /var/www;
        }
    - "name": "web2"
      "startup": >
        This server requires
        special startup procedures.
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        if let Some(Value::Object(config)) = obj.get("config") {
            if let Some(Value::Object(database)) = config.get("database") {
                let expected_query = "SELECT *\nFROM users\nWHERE active = true\n";
                assert_eq!(database.get("query"), Some(&Value::String(expected_query.to_string())));
                
                let expected_desc = "This is a long description that spans multiple lines and will be folded.\n";
                assert_eq!(database.get("description"), Some(&Value::String(expected_desc.to_string())));
            }
            
            if let Some(Value::Array(servers)) = config.get("servers") {
                if let Value::Object(server1) = &servers[0] {
                    let expected_config = "server {\n  listen 80;\n  root /var/www;\n}\n";
                    assert_eq!(server1.get("config"), Some(&Value::String(expected_config.to_string())));
                }
            }
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_multiline_string_edge_cases() {
    // Edge cases for multiline strings
    let jyaml = r#"
"only_spaces": |
   
   
"only_newlines": |



"mixed_whitespace": |
  Line 1
  
  Line 3
"trailing_spaces": |
  Line with trailing spaces   
  Another line
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("only_spaces"), Some(&Value::String("   \n   \n".to_string())));
        assert_eq!(obj.get("only_newlines"), Some(&Value::String("\n\n\n\n".to_string())));
        assert_eq!(obj.get("mixed_whitespace"), Some(&Value::String("Line 1\n\nLine 3\n".to_string())));
        assert_eq!(obj.get("trailing_spaces"), Some(&Value::String("Line with trailing spaces   \nAnother line\n".to_string())));
    } else {
        panic!("Expected object");
    }
}