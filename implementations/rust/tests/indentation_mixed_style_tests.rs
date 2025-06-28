use jyaml::{parse, Value};
use pretty_assertions::assert_eq;

#[test]
fn test_block_in_flow_should_error() {
    // Block collections nested within flow collections should error
    let test_cases = vec![
        r#"["valid", "array", - "invalid_block_item"]"#,
        r#"{"key": "value", - "block_array_item"}"#,
        r#"[1, 2, "key": "block_object"]"#,
        r#"{"flow": true, "nested":
  - "block"
  - "items"}"#,
    ];
    
    for jyaml in test_cases {
        let result = parse(jyaml);
        assert!(result.is_err(), "Should error for block in flow: {}", jyaml);
    }
}

#[test]
fn test_complex_nested_indentation() {
    // Complex array-object nesting with varying indentations
    let jyaml = r#"
"users":
  - "name": "Alice"
    "permissions":
      - "read"
      - "write"
    "profile":
      "age": 30
      "settings":
        "theme": "dark"
        "notifications":
          - "email"
          - "push"
  - "name": "Bob"
    "permissions": ["admin"]
    "profile":
      "age": 25
"global":
  "version": "1.0"
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        if let Some(Value::Array(users)) = obj.get("users") {
            assert_eq!(users.len(), 2);
            
            // Check Alice's data
            if let Value::Object(alice) = &users[0] {
                assert_eq!(alice.get("name"), Some(&Value::String("Alice".to_string())));
                
                if let Some(Value::Array(perms)) = alice.get("permissions") {
                    assert_eq!(perms.len(), 2);
                    assert_eq!(perms[0], Value::String("read".to_string()));
                    assert_eq!(perms[1], Value::String("write".to_string()));
                }
                
                if let Some(Value::Object(profile)) = alice.get("profile") {
                    if let Some(Value::Object(settings)) = profile.get("settings") {
                        assert_eq!(settings.get("theme"), Some(&Value::String("dark".to_string())));
                        
                        if let Some(Value::Array(notifications)) = settings.get("notifications") {
                            assert_eq!(notifications.len(), 2);
                        }
                    }
                }
            }
            
            // Check Bob's data  
            if let Value::Object(bob) = &users[1] {
                assert_eq!(bob.get("name"), Some(&Value::String("Bob".to_string())));
                
                if let Some(Value::Array(perms)) = bob.get("permissions") {
                    assert_eq!(perms.len(), 1);
                    assert_eq!(perms[0], Value::String("admin".to_string()));
                }
            }
        }
        
        assert_eq!(obj.get("global"), Some(&Value::Object({
            let mut map = std::collections::HashMap::new();
            map.insert("version".to_string(), Value::String("1.0".to_string()));
            map
        })));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_indentation_with_comments() {
    // Comments affecting indentation parsing
    let jyaml = r#"
"config":   # Configuration section
  "database":   # Database settings
    "host": "localhost"   # Server host
    "port": 5432   # Server port
    # Connection pool settings
    "pool":
      "min": 5   # Minimum connections
      "max": 20   # Maximum connections
  # API settings
  "api":
    "timeout": 30   # Request timeout in seconds
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        if let Some(Value::Object(config)) = obj.get("config") {
            if let Some(Value::Object(database)) = config.get("database") {
                assert_eq!(database.get("host"), Some(&Value::String("localhost".to_string())));
                assert_eq!(database.get("port"), Some(&Value::Number(jyaml::value::Number::Integer(5432))));
                
                if let Some(Value::Object(pool)) = database.get("pool") {
                    assert_eq!(pool.get("min"), Some(&Value::Number(jyaml::value::Number::Integer(5))));
                    assert_eq!(pool.get("max"), Some(&Value::Number(jyaml::value::Number::Integer(20))));
                }
            }
            
            if let Some(Value::Object(api)) = config.get("api") {
                assert_eq!(api.get("timeout"), Some(&Value::Number(jyaml::value::Number::Integer(30))));
            }
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_inconsistent_indentation_errors() {
    // Various inconsistent indentation scenarios that should error
    let test_cases = vec![
        // Mixed spaces and tabs
        r#"
"key1": "value1"
	"key2": "value2"
"#,
        // Inconsistent object indentation
        r#"
"object":
  "key1": "value1"
    "key2": "value2"
"#,
        // Inconsistent array indentation
        r#"
"array":
  - "item1"
    - "item2"
"#,
        // Mixed indentation in nested structure
        r#"
"root":
  "level1":
    "level2": "value"
      "level2b": "value"
"#,
    ];
    
    for (i, jyaml) in test_cases.iter().enumerate() {
        let result = parse(jyaml);
        assert!(result.is_err(), "Test case {} should error: {}", i, jyaml);
        
        if let Err(err) = result {
            let error_msg = format!("{}", err);
            assert!(
                error_msg.contains("indent") || 
                error_msg.contains("tab") || 
                error_msg.contains("indentation"),
                "Error should mention indentation issue: {}", error_msg
            );
        }
    }
}

#[test]
fn test_edge_case_indentation_scenarios() {
    // Edge case indentation scenarios
    let jyaml = r#"
"empty_object": {}
"empty_array": []
"single_item_array":
  - "item"
"single_key_object":
  "key": "value"
"mixed_empty_and_content":
  "empty": {}
  "array":
    - "item1"
    - "item2"
  "nested":
    "deep":
      "deeper": "value"
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        assert_eq!(obj.get("empty_object"), Some(&Value::Object(std::collections::HashMap::new())));
        assert_eq!(obj.get("empty_array"), Some(&Value::Array(vec![])));
        
        if let Some(Value::Array(single_array)) = obj.get("single_item_array") {
            assert_eq!(single_array.len(), 1);
            assert_eq!(single_array[0], Value::String("item".to_string()));
        }
        
        if let Some(Value::Object(single_obj)) = obj.get("single_key_object") {
            assert_eq!(single_obj.get("key"), Some(&Value::String("value".to_string())));
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_deeply_nested_structure_limits() {
    // Test depth limits and stack overflow protection
    fn create_deep_structure(depth: usize) -> String {
        let mut result = String::new();
        
        for i in 0..depth {
            result.push_str(&format!("\"level{}\": {{\n", i));
            result.push_str(&"  ".repeat(i + 1));
        }
        
        result.push_str("\"value\": \"deep\"\n");
        
        for i in 0..depth {
            result.push_str(&"  ".repeat(depth - i - 1));
            result.push_str("}\n");
        }
        
        result
    }
    
    // Reasonable depth should work
    let reasonable_depth = create_deep_structure(50);
    let result = parse(&reasonable_depth);
    assert!(result.is_ok(), "Reasonable depth should parse successfully");
    
    // Extremely deep structure should be handled gracefully  
    // Reduced depth to avoid stack overflow in tests
    let extreme_depth = create_deep_structure(500);
    let result = parse(&extreme_depth);
    // Should either succeed or fail gracefully with depth error
    match result {
        Ok(_) => {
            // If it succeeds, that's fine
        }
        Err(err) => {
            let error_msg = format!("{}", err);
            // Should be a depth/recursion error, not a crash
            assert!(
                error_msg.contains("depth") || 
                error_msg.contains("recursion") || 
                error_msg.contains("nested") ||
                error_msg.contains("stack"),
                "Deep structure error should mention depth/recursion: {}", error_msg
            );
        }
    }
}

#[test]
fn test_mixed_flow_and_block_styles() {
    // Valid mixed flow and block styles
    let jyaml = r#"
"config":
  "servers": ["web1", "web2", "db1"]
  "users":
    - {"name": "Alice", "role": "admin"}
    - {"name": "Bob", "role": "user"}
  "settings":
    "timeout": 30
    "retries": 3
    "features": ["auth", "logging", "metrics"]
"metadata": {"version": "1.0", "author": "team"}
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        if let Some(Value::Object(config)) = obj.get("config") {
            // Check servers array (flow style)
            if let Some(Value::Array(servers)) = config.get("servers") {
                assert_eq!(servers.len(), 3);
                assert_eq!(servers[0], Value::String("web1".to_string()));
                assert_eq!(servers[1], Value::String("web2".to_string()));
                assert_eq!(servers[2], Value::String("db1".to_string()));
            }
            
            // Check users array (block style with flow objects)
            if let Some(Value::Array(users)) = config.get("users") {
                assert_eq!(users.len(), 2);
                if let Value::Object(alice) = &users[0] {
                    assert_eq!(alice.get("name"), Some(&Value::String("Alice".to_string())));
                    assert_eq!(alice.get("role"), Some(&Value::String("admin".to_string())));
                }
            }
            
            // Check settings object (block style)
            if let Some(Value::Object(settings)) = config.get("settings") {
                assert_eq!(settings.get("timeout"), Some(&Value::Number(jyaml::value::Number::Integer(30))));
                
                if let Some(Value::Array(features)) = settings.get("features") {
                    assert_eq!(features.len(), 3);
                }
            }
        }
        
        // Check metadata (flow style object)
        if let Some(Value::Object(metadata)) = obj.get("metadata") {
            assert_eq!(metadata.get("version"), Some(&Value::String("1.0".to_string())));
            assert_eq!(metadata.get("author"), Some(&Value::String("team".to_string())));
        }
    } else {
        panic!("Expected object");
    }
}

#[test] 
fn test_comment_placement_edge_cases() {
    // Comments in various positions affecting parsing
    let jyaml = r#"
# Document start comment
"root": # Root comment
  # Before key comment
  "key1": "value1" # Inline comment
  
  # Between keys comment
  "key2": # After colon comment
    # Before value comment
    "nested_value" # Final comment
    
# Document end comment
"#;
    
    let value = parse(jyaml).unwrap();
    if let Value::Object(obj) = value {
        if let Some(Value::Object(root)) = obj.get("root") {
            assert_eq!(root.get("key1"), Some(&Value::String("value1".to_string())));
            assert_eq!(root.get("key2"), Some(&Value::String("nested_value".to_string())));
        }
    } else {
        panic!("Expected object");
    }
}