//! JYAML deserializer implementation

use crate::{error::Result, parser, value::Value, Error, options::DeserializeOptions};
use serde::de::{self, Deserialize};

/// Deserialize a JYAML string into a type that implements `serde::Deserialize`
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;
    T::deserialize(&mut deserializer)
}

/// Deserialize a JYAML string with custom options into a type that implements `serde::Deserialize`
pub fn from_str_with_options<'a, T>(s: &'a str, options: &DeserializeOptions) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str_with_options(s, options)?;
    T::deserialize(&mut deserializer)
}

/// A JYAML deserializer
pub struct Deserializer {
    value: Value,
}

impl Deserializer {
    /// Create a new deserializer from a JYAML string
    pub fn from_str(s: &str) -> Result<Self> {
        Self::from_str_with_options(s, &DeserializeOptions::default())
    }
    
    /// Create a new deserializer from a JYAML string with custom options
    pub fn from_str_with_options(s: &str, options: &DeserializeOptions) -> Result<Self> {
        let value = parser::parse_with_options(s, options)?;
        Ok(Deserializer { value })
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match &self.value {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(*b),
            Value::Number(n) => match n {
                crate::value::Number::Integer(i) => visitor.visit_i64(*i),
                crate::value::Number::Float(f) => visitor.visit_f64(*f),
            },
            Value::String(s) => visitor.visit_string(s.clone()),
            Value::Array(arr) => {
                let seq = SeqAccess::new(arr.clone());
                visitor.visit_seq(seq)
            }
            Value::Object(map) => {
                let map_access = MapAccess::new(map.clone());
                visitor.visit_map(map_access)
            }
        }
    }

    // Forward to deserialize_any for most types
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct SeqAccess {
    iter: std::vec::IntoIter<Value>,
}

impl SeqAccess {
    fn new(vec: Vec<Value>) -> Self {
        SeqAccess {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => {
                let mut de = Deserializer { value };
                seed.deserialize(&mut de).map(Some)
            }
            None => Ok(None),
        }
    }
}

struct MapAccess {
    iter: std::collections::hash_map::IntoIter<String, Value>,
    value: Option<Value>,
}

impl MapAccess {
    fn new(map: std::collections::HashMap<String, Value>) -> Self {
        MapAccess {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let mut de = Deserializer {
                    value: Value::String(key),
                };
                seed.deserialize(&mut de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .ok_or_else(|| Error::Deserialization("missing value in map".to_string()))?;
        let mut de = Deserializer { value };
        seed.deserialize(&mut de)
    }
}