use crate::from_body::ToCodeMessage;
use crate::CodeMessage;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// A dynamic value type for open objects (objects without predefined properties).
/// Similar to `serde_json::Value` but tailored for API serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v as f64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::Array(v.into_iter().map(Into::into).collect())
    }
}

impl<T: Into<Value>> From<HashMap<String, T>> for Value {
    fn from(v: HashMap<String, T>) -> Self {
        Value::Object(v.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}

/// Type alias for open objects - objects without predefined properties.
/// This allows any string key with dynamic values.
pub type OpenObject = HashMap<String, Value>;

/// Generic response type for APIs without strongly-typed response definitions.
/// This is used when an API produces JSON but doesn't define a 200 response schema.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct OpenObjectResponse {
    #[serde(flatten)]
    pub code_message: CodeMessage,
    /// Additional response data as an open object (dynamic JSON)
    pub open_object: OpenObject,
}

impl ToCodeMessage for OpenObjectResponse {
    fn to_code_message(&self) -> &CodeMessage {
        &self.code_message
    }
}
