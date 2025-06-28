use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a JYAML value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

/// Represents a JYAML number
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Number(Number::Integer(i)) => Some(*i),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(Number::Float(f)) => Some(*f),
            Value::Number(Number::Integer(i)) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Number::Integer(i) => serializer.serialize_i64(*i),
            Number::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> serde::de::Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(Number::Integer(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
                Ok(Number::Float(value))
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}
