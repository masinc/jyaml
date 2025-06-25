use jyaml::{parse, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[test]
fn test_suite_valid_files() {
    let test_suite_path = "../../test-suite";
    let valid_path = format!("{}/valid", test_suite_path);
    let expected_path = format!("{}/expected", test_suite_path);
    
    // Test basic valid files
    test_category(&valid_path, &expected_path, "basic");
    
    // Test complex valid files  
    test_category(&valid_path, &expected_path, "complex");
    
    // Test edge-case valid files
    test_category(&valid_path, &expected_path, "edge-cases");
}

#[test]
fn test_suite_invalid_files() {
    let test_suite_path = "../../test-suite";
    let invalid_path = format!("{}/invalid", test_suite_path);
    
    // Test syntax errors
    test_invalid_category(&invalid_path, "syntax");
    
    // Test structure errors
    test_invalid_category(&invalid_path, "structure");
    
    // Test type errors
    test_invalid_category(&invalid_path, "types");
}

fn test_category(valid_path: &str, expected_path: &str, category: &str) {
    let valid_dir = format!("{}/{}", valid_path, category);
    let expected_dir = format!("{}/{}", expected_path, category);
    
    if !Path::new(&valid_dir).exists() {
        eprintln!("Warning: {} directory not found", valid_dir);
        return;
    }
    
    let entries = fs::read_dir(&valid_dir).unwrap();
    
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("jyml") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let jyaml_content = fs::read_to_string(&path).unwrap();
            
            // Parse JYAML
            let parsed_value = parse(&jyaml_content).unwrap_or_else(|e| {
                panic!("Failed to parse {}: {}", path.display(), e);
            });
            
            // Load expected JSON if it exists
            let json_path = format!("{}/{}.json", expected_dir, file_stem);
            if Path::new(&json_path).exists() {
                let json_content = fs::read_to_string(&json_path).unwrap();
                let expected_json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
                let expected_jyaml = json_to_jyaml_value(expected_json);
                
                assert_eq!(parsed_value, expected_jyaml, 
                    "Mismatch for {}: parsed != expected", path.display());
                
                println!("✓ {}/{}.jyml", category, file_stem);
            } else {
                // Just verify it parses without error
                println!("✓ {}/{}.jyml (no expected output)", category, file_stem);
            }
        }
    }
}

fn test_invalid_category(invalid_path: &str, category: &str) {
    let invalid_dir = format!("{}/{}", invalid_path, category);
    
    if !Path::new(&invalid_dir).exists() {
        eprintln!("Warning: {} directory not found", invalid_dir);
        return;
    }
    
    let entries = fs::read_dir(&invalid_dir).unwrap();
    
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            if extension == "jyml" || extension == "jytml" {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let jyaml_content = fs::read_to_string(&path).unwrap();
                
                // Should fail to parse
                match parse(&jyaml_content) {
                    Ok(value) => {
                        panic!("Expected {} to fail parsing, but got: {:?}", path.display(), value);
                    }
                    Err(_) => {
                        println!("✓ {}/{} correctly failed to parse", category, file_stem);
                    }
                }
            }
        }
    }
}

// Convert serde_json::Value to jyaml::Value for comparison
fn json_to_jyaml_value(json_value: serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Number(jyaml::value::Number::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Value::Number(jyaml::value::Number::Float(f))
            } else {
                panic!("Invalid number: {}", n);
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(json_to_jyaml_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k, json_to_jyaml_value(v));
            }
            Value::Object(map)
        }
    }
}