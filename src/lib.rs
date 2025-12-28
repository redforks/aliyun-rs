#![doc = include_str!("../README.md")]

use anyhow::anyhow;
use http::{HeaderValue, Method};
use reqwest::Body;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;

mod common;
mod v3;

pub use v3::AccessKeySecret;

pub mod sms;

#[cfg(test)]
mod sample;

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Ali(#[from] CodeMessage),

    #[error(transparent)]
    JsonSerialize(#[from] serde_json::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) enum QueryValue<'a> {
    Str(&'a str),
    OwnedStr(String),
    I64(i64),
    I32(i32),
    Bool(bool),
    Json(serde_json::Value),
}

impl From<String> for QueryValue<'_> {
    fn from(value: String) -> Self {
        Self::OwnedStr(value)
    }
}

impl<'a> From<&'a String> for QueryValue<'a> {
    fn from(value: &'a String) -> Self {
        Self::Str(value)
    }
}

impl<'a> From<&'a str> for QueryValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<i64> for QueryValue<'_> {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl<'a> From<&'a i64> for QueryValue<'_> {
    fn from(value: &i64) -> Self {
        Self::I64(*value)
    }
}

impl From<i32> for QueryValue<'_> {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<bool> for QueryValue<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<'a> From<&'a i32> for QueryValue<'_> {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl<'a> From<&'a bool> for QueryValue<'_> {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl<'a> From<serde_json::Value> for QueryValue<'a> {
    fn from(value: serde_json::Value) -> Self {
        Self::Json(value)
    }
}

impl<'a> QueryValue<'a> {
    fn to_query_value(&self) -> String {
        match self {
            QueryValue::Str(v) => v.to_string(),
            QueryValue::I64(v) => v.to_string(),
            QueryValue::I32(v) => v.to_string(),
            QueryValue::Bool(v) => {
                if *v {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            QueryValue::Json(v) => v.to_string(),
            QueryValue::OwnedStr(v) => v.clone(),
        }
    }

    fn url_encode(&self) -> String {
        urlencoding::encode(&self.to_query_value()).into_owned()
    }
}

/// Trait for types that can be flattened into query parameters.
/// Used for ParameterStyle::Flat and ParameterStyle::RepeatList.
///
/// This trait handles the recursive flattening of nested structs and arrays
/// into dot-notation key-value pairs, e.g., `Name.1.Field` for array elements
/// or `Name.Field` for nested objects.
pub(crate) trait FlatSerialize {
    /// Append this value's query parameters to the map.
    ///
    /// For scalar types, this inserts a single entry with the given name.
    /// For struct types, this inserts entries for each field with `name.field_name` format.
    /// For array types (with RepeatList style), this inserts entries with `name.{index}` format (1-indexed).
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    );
}

impl FlatSerialize for String {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

impl FlatSerialize for str {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), self.into());
    }
}

impl FlatSerialize for i32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), (*self).into());
    }
}

impl FlatSerialize for i64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), (*self).into());
    }
}

impl FlatSerialize for bool {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), (*self).into());
    }
}

impl FlatSerialize for f32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), self.to_string().into());
    }
}

impl FlatSerialize for f64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        params.insert(name.to_string().into(), self.to_string().into());
    }
}

impl<T: FlatSerialize> FlatSerialize for Vec<T> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        for (i, item) in self.iter().enumerate() {
            item.flat_serialize(&format!("{}.{}", name, i + 1), params);
        }
    }
}

impl<T: FlatSerialize> FlatSerialize for Option<T> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        if let Some(v) = self {
            v.flat_serialize(name, params);
        }
    }
}

/// A dynamic value type for open objects (objects without predefined properties).
/// Similar to `serde_json::Value` but tailored for API serialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
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

impl FlatSerialize for Value {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        match self {
            Value::Null => {}
            Value::Bool(v) => {
                params.insert(name.to_string().into(), (*v).into());
            }
            Value::Integer(v) => {
                params.insert(name.to_string().into(), (*v).into());
            }
            Value::Float(v) => {
                params.insert(name.to_string().into(), v.to_string().into());
            }
            Value::String(v) => {
                params.insert(name.to_string().into(), v.as_str().into());
            }
            Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    item.flat_serialize(&format!("{}.{}", name, i + 1), params);
                }
            }
            Value::Object(obj) => {
                for (key, value) in obj {
                    value.flat_serialize(&format!("{}.{}", name, key), params);
                }
            }
        }
    }
}

impl<V: FlatSerialize> FlatSerialize for HashMap<String, V> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut BTreeMap<Cow<'static, str>, QueryValue<'a>>,
    ) {
        for (key, value) in self {
            value.flat_serialize(&format!("{}.{}", name, key), params);
        }
    }
}

/// Type alias for open objects - objects without predefined properties.
/// This allows any string key with dynamic values.
pub type OpenObject = HashMap<String, Value>;

/// Trait for types that can be converted to form data parameters.
/// This is used instead of serde_urlencoded to support custom parameter styles
/// like Flat and RepeatList.
trait ToFormData {
    fn to_form_data(&self) -> BTreeMap<Cow<'static, str>, QueryValue<'_>>;
}

/// Each api entry should implement this trait.
trait Request: Sized + Send {
    const METHOD: Method;
    const URL_PATH: &'static str = "/";
    const ACTION: &'static str;
    /// Request body, will serialize to json. Use unit type if no request body.
    /// Not used if Method is GET.
    type Body: IntoBody + Send;
    /// Response type returned by the call() method.
    type Response: DeserializeOwned;

    fn to_query_params(&self) -> Result<BTreeMap<Cow<'static, str>, QueryValue<'_>>> {
        Ok(BTreeMap::new())
    }
    fn to_body(self) -> Result<Self::Body>;
}

trait IntoBody {
    fn content_type(&self) -> HeaderValue;
    fn into_body(self) -> Result<Body>;
}

impl IntoBody for () {
    fn content_type(&self) -> HeaderValue {
        unreachable!("Unit type should used with GET method")
    }

    fn into_body(self) -> Result<Body> {
        Ok(b"".as_slice().into())
    }
}

pub(crate) struct Form<T: ToFormData>(pub T);

impl<T: ToFormData> IntoBody for Form<T> {
    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_static("application/x-www-form-urlencoded")
    }

    fn into_body(self) -> Result<Body> {
        let params = self.0.to_form_data();
        let encoded = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), v.url_encode()))
            .collect::<Vec<_>>()
            .join("&");
        Ok(encoded.into())
    }
}

#[derive(Debug, Deserialize, Default, thiserror::Error)]
#[serde(rename_all = "PascalCase")]
#[error("{code}: {message}")]
pub struct CodeMessage {
    pub code: String,
    pub message: String,
}

impl CodeMessage {
    pub fn check(&self) -> Result<()> {
        if &self.code == "OK" {
            Ok(())
        } else {
            Err(anyhow!("{}: {}", self.code, self.message).into())
        }
    }
}

impl From<CodeMessage> for Result<()> {
    fn from(value: CodeMessage) -> Self {
        value.check()
    }
}
