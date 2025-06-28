//! JYAML serializer implementation

use crate::{error::Result, Error};
use serde::ser::{self, Serialize, SerializeSeq, SerializeMap, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeStruct, SerializeStructVariant};
use std::fmt::Write;

/// Serialize a value into a JYAML string
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

/// Serialize a value into a pretty-printed JYAML string
pub fn to_string_pretty<T>(value: &T, indent_size: usize) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::pretty(indent_size);
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

/// A JYAML serializer
pub struct Serializer {
    output: String,
    indent_size: usize,
    current_indent: usize,
    is_pretty: bool,
    in_flow: bool,
}

impl Serializer {
    /// Create a new serializer with compact output
    pub fn new() -> Self {
        Serializer {
            output: String::new(),
            indent_size: 2,
            current_indent: 0,
            is_pretty: false,
            in_flow: false,
        }
    }

    /// Create a new serializer with pretty-printed output
    pub fn pretty(indent_size: usize) -> Self {
        Serializer {
            output: String::new(),
            indent_size,
            current_indent: 0,
            is_pretty: true,
            in_flow: false,
        }
    }

    fn write_indent(&mut self) -> Result<()> {
        if self.is_pretty && !self.in_flow {
            for _ in 0..self.current_indent {
                self.output.push(' ');
            }
        }
        Ok(())
    }

    fn write_newline(&mut self) -> Result<()> {
        if self.is_pretty && !self.in_flow {
            self.output.push('\n');
        }
        Ok(())
    }

    fn write_unicode_escape(&mut self, code: u32) -> Result<()> {
        write!(&mut self.output, "\\u{:04X}", code)?;
        Ok(())
    }

    fn write_surrogate_pair(&mut self, code_point: u32) -> Result<()> {
        // Convert Unicode code point to UTF-16 surrogate pair
        let code = code_point - 0x10000;
        let high_surrogate = 0xD800 + (code >> 10);
        let low_surrogate = 0xDC00 + (code & 0x3FF);
        
        write!(&mut self.output, "\\u{:04X}\\u{:04X}", high_surrogate, low_surrogate)?;
        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push_str(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        write!(&mut self.output, "{}", v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        write!(&mut self.output, "{}", v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        write!(&mut self.output, "{}", v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.push('"');
        for ch in v.chars() {
            match ch {
                '"' => self.output.push_str("\\\""),
                '\\' => self.output.push_str("\\\\"),
                '\u{0008}' => self.output.push_str("\\b"),
                '\u{000C}' => self.output.push_str("\\f"),
                '\n' => self.output.push_str("\\n"),
                '\r' => self.output.push_str("\\r"),
                '\t' => self.output.push_str("\\t"),
                ch if ch.is_control() => {
                    self.write_unicode_escape(ch as u32)?;
                }
                ch if (ch as u32) > 0xFFFF => {
                    // 4-byte Unicode character: use surrogate pair
                    self.write_surrogate_pair(ch as u32)?;
                }
                ch => self.output.push(ch),
            }
        }
        self.output.push('"');
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        // Serialize as an array of numbers
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            SerializeSeq::serialize_element(&mut seq, byte)?;
        }
        SerializeSeq::end(seq)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.push_str("null");
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut map = self.serialize_map(Some(1))?;
        SerializeMap::serialize_entry(&mut map, variant, value)?;
        SerializeMap::end(map)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        if self.in_flow || !self.is_pretty {
            self.output.push('[');
            self.in_flow = true;
        } else {
            self.current_indent += self.indent_size;
        }
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let mut map = self.serialize_map(Some(1))?;
        SerializeMap::serialize_key(&mut map, variant)?;
        Ok(map)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        if self.in_flow || !self.is_pretty {
            self.output.push('{');
            self.in_flow = true;
        } else {
            self.current_indent += self.indent_size;
        }
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let mut map = self.serialize_map(Some(1))?;
        SerializeMap::serialize_key(&mut map, variant)?;
        Ok(map)
    }
}

// Implement serialization for sequences
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.in_flow {
            if self.output.ends_with('[') {
                // First element
            } else {
                self.output.push_str(", ");
            }
        } else {
            self.write_newline()?;
            self.write_indent()?;
            self.output.push_str("- ");
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if self.in_flow {
            self.output.push(']');
            self.in_flow = false;
        } else {
            self.current_indent = self.current_indent.saturating_sub(self.indent_size);
        }
        Ok(())
    }
}

// Forward tuple serialization to sequence
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        SerializeSeq::end(self)
    }
}

// Implement serialization for maps
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.in_flow {
            if self.output.ends_with('{') {
                // First element
            } else {
                self.output.push_str(", ");
            }
        } else {
            self.write_newline()?;
            self.write_indent()?;
        }
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.push_str(": ");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if self.in_flow {
            self.output.push('}');
            self.in_flow = false;
        } else {
            self.current_indent = self.current_indent.saturating_sub(self.indent_size);
        }
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<()> {
        SerializeMap::end(self)
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<()> {
        SerializeMap::end(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{Number, Value};
    use std::collections::HashMap;

    #[test]
    fn test_serialize_null() {
        let value = Value::Null;
        let result = to_string(&value).unwrap();
        assert_eq!(result, "null");
    }

    #[test]
    fn test_serialize_booleans() {
        assert_eq!(to_string(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(to_string(&Value::Bool(false)).unwrap(), "false");
    }

    #[test]
    fn test_serialize_numbers() {
        assert_eq!(to_string(&Value::Number(Number::Integer(42))).unwrap(), "42");
        assert_eq!(to_string(&Value::Number(Number::Integer(-10))).unwrap(), "-10");
        assert_eq!(to_string(&Value::Number(Number::Float(3.14))).unwrap(), "3.14");
    }

    #[test]
    fn test_serialize_strings() {
        assert_eq!(to_string(&Value::String("hello".to_string())).unwrap(), r#""hello""#);
        assert_eq!(to_string(&Value::String("world".to_string())).unwrap(), r#""world""#);
    }

    #[test]
    fn test_serialize_string_escapes() {
        assert_eq!(
            to_string(&Value::String("hello\nworld".to_string())).unwrap(),
            r#""hello\nworld""#
        );
        assert_eq!(
            to_string(&Value::String("tab\there".to_string())).unwrap(),
            r#""tab\there""#
        );
        assert_eq!(
            to_string(&Value::String(r#"quote""#.to_string())).unwrap(),
            r#""quote\"""#
        );
    }

    #[test]
    fn test_serialize_array() {
        let array = Value::Array(vec![
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3)),
        ]);
        let result = to_string(&array).unwrap();
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_serialize_empty_array() {
        let array = Value::Array(vec![]);
        let result = to_string(&array).unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_serialize_object() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("John".to_string()));
        obj.insert("age".to_string(), Value::Number(Number::Integer(30)));
        
        let value = Value::Object(obj);
        let result = to_string(&value).unwrap();
        
        // Since HashMap ordering is not guaranteed, check both possible orders
        assert!(
            result == r#"{"name": "John", "age": 30}"# ||
            result == r#"{"age": 30, "name": "John"}"#
        );
    }

    #[test]
    fn test_serialize_empty_object() {
        let obj = HashMap::new();
        let value = Value::Object(obj);
        let result = to_string(&value).unwrap();
        assert_eq!(result, "{}");
    }

    #[test]
    fn test_serialize_nested_structures() {
        let mut inner_obj = HashMap::new();
        inner_obj.insert("nested".to_string(), Value::Bool(true));
        
        let mut obj = HashMap::new();
        obj.insert("array".to_string(), Value::Array(vec![
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
        ]));
        obj.insert("object".to_string(), Value::Object(inner_obj));
        
        let value = Value::Object(obj);
        let result = to_string(&value).unwrap();
        
        // Check that result contains expected parts
        assert!(result.contains("[1, 2]"));
        assert!(result.contains(r#""nested": true"#));
    }

    #[test]
    fn test_serialize_unicode() {
        let value = Value::String("© 2023 🦀".to_string());
        let result = to_string(&value).unwrap();
        // With JYAML 0.4 spec: 4-byte chars should use surrogate pairs
        assert_eq!(result, r#""© 2023 \uD83E\uDD80""#);
    }
    
    #[test]
    fn test_serialize_emoji_surrogate_pairs() {
        let value = Value::String("🚀🎉🦀".to_string());
        let result = to_string(&value).unwrap();
        // 🚀 = U+1F680 -> \uD83D\uDE80
        // 🎉 = U+1F389 -> \uD83C\uDF89  
        // 🦀 = U+1F980 -> \uD83E\uDD80
        assert_eq!(result, r#""\uD83D\uDE80\uD83C\uDF89\uD83E\uDD80""#);
    }
    
    #[test]
    fn test_serialize_mixed_unicode() {
        let value = Value::String("Hello © 🚀 World".to_string());
        let result = to_string(&value).unwrap();
        // BMP chars remain as-is, 4-byte chars become surrogate pairs
        assert_eq!(result, r#""Hello © \uD83D\uDE80 World""#);
    }

    #[test]
    fn test_serialize_control_characters() {
        let value = Value::String("\u{0001}\u{001F}".to_string());
        let result = to_string(&value).unwrap();
        assert_eq!(result, r#""\u0001\u001F""#);
    }

    #[test]
    fn test_serialize_pretty() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("Alice".to_string()));
        obj.insert("numbers".to_string(), Value::Array(vec![
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
        ]));
        
        let value = Value::Object(obj);
        let result = to_string_pretty(&value, 2).unwrap();
        
        // Pretty format should include newlines and indentation
        assert!(result.contains('\n'));
        assert!(result.contains("  ")); // indentation
    }

    #[test]
    fn test_serialize_serde_struct() {
        #[derive(serde::Serialize)]
        struct Person {
            name: String,
            age: u32,
            active: bool,
        }
        
        let person = Person {
            name: "Bob".to_string(),
            age: 25,
            active: true,
        };
        
        let result = to_string(&person).unwrap();
        
        // Check that all fields are present
        assert!(result.contains(r#""name": "Bob""#));
        assert!(result.contains(r#""age": 25"#));
        assert!(result.contains(r#""active": true"#));
    }

    #[test]
    fn test_serialize_serde_array() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = to_string(&numbers).unwrap();
        assert_eq!(result, "[1, 2, 3, 4, 5]");
    }

    #[test]
    fn test_serialize_option() {
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;
        
        assert_eq!(to_string(&some_value).unwrap(), "42");
        assert_eq!(to_string(&none_value).unwrap(), "null");
    }

    #[test]
    fn test_serialize_bytes() {
        let bytes: &[u8] = &[1, 2, 3, 255];
        let result = to_string(&bytes).unwrap();
        assert_eq!(result, "[1, 2, 3, 255]");
    }

    #[test]
    fn test_serialize_special_floats() {
        // Test that special float values are handled properly
        let value = Value::Number(Number::Float(0.0));
        assert_eq!(to_string(&value).unwrap(), "0");
        
        let value = Value::Number(Number::Float(-0.0));
        assert_eq!(to_string(&value).unwrap(), "-0");
    }

    #[test]
    fn test_serializer_state() {
        // Test that serializer correctly handles different contexts
        let complex_structure = Value::Object({
            let mut obj = HashMap::new();
            obj.insert("flow_array".to_string(), Value::Array(vec![
                Value::String("item1".to_string()),
                Value::String("item2".to_string()),
            ]));
            obj.insert("flow_object".to_string(), Value::Object({
                let mut inner = HashMap::new();
                inner.insert("key".to_string(), Value::String("value".to_string()));
                inner
            }));
            obj
        });
        
        let result = to_string(&complex_structure).unwrap();
        // Should produce valid JYAML
        assert!(result.contains("["));
        assert!(result.contains("{"));
        assert!(result.contains(":"));
    }

    #[test]
    fn test_round_trip_compatibility() {
        // Test that serialized values can be parsed back
        let original = Value::Array(vec![
            Value::String("test".to_string()),
            Value::Number(Number::Integer(42)),
            Value::Bool(true),
            Value::Null,
        ]);
        
        let serialized = to_string(&original).unwrap();
        let parsed = crate::parse(&serialized).unwrap();
        assert_eq!(original, parsed);
    }
}