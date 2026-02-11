#![doc = include_str!("../README.md")]

use anyhow::anyhow;
use http::HeaderMap;
use http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

mod auth;
mod common;
mod from_body;
mod into_body;
mod query_value;
mod serializes;
mod v3;

use from_body::{
    BinaryResponseWithMetaWrap, BinaryResponseWrap, BinaryWithMeta, CODE_MESSAGE, FromBody,
    IntoResponse, JsonResponseWrap, SelfResponseWrap, ToCodeMessage, XmlResponseWrap,
};
use into_body::{Form, IntoBody, JsonBody, OctetStream, XmlBody};
use query_value::QueryValue;
use serializes::{FlatSerialize, SimpleSerialize};
pub use v3::AccessKeySecret;

#[cfg(feature = "ocr")]
pub mod ocr;

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(feature = "ecs")]
pub mod ecs;

#[cfg(feature = "oss")]
pub mod oss;

#[cfg(feature = "fc")]
pub mod fc;

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

/// Trait for types that can be converted to form data parameters.
/// This is used instead of serde_urlencoded to support custom parameter styles
/// like Flat and RepeatList.
pub(crate) trait ToFormData {
    fn to_form_data(&self) -> Vec<(Cow<'static, str>, QueryValue<'_>)>;
}

/// Each api entry should implement this trait.
trait Request: Sized + Send {
    const METHOD: Method;
    const URL_PATH: &'static str = "/";
    const ACTION: &'static str;
    /// Request body, will serialize to json. Use unit type if no request body.
    /// Not used if Method is GET.
    type Body: IntoBody + Send;
    /// Response wrapper type that handles deserialization from response bytes.
    type ResponseWrap: FromBody + ToCodeMessage + IntoResponse;

    fn to_query_params(&self) -> Vec<(Cow<'static, str>, QueryValue<'_>)> {
        Vec::new()
    }

    /// Used in oss api to change host name by bucket
    fn process_endpoint(&self, endpoint: &'static str) -> Cow<'static, str> {
        endpoint.into()
    }

    /// Returns custom headers to be included in the request.
    fn to_headers(&self) -> Vec<(Cow<'static, str>, String)> {
        Vec::new()
    }

    /// Returns the resource path for ROA-style APIs (e.g., "/bucket/object" for OSS).
    /// Defaults to "/" for non-ROA APIs.
    fn resource_path(&self) -> Cow<'static, str> {
        "/".into()
    }

    fn to_body(self) -> Self::Body;

    fn get_path_args(&self) -> Box<[(&'static str, String)]> {
        Box::new([])
    }

    fn from_headers(_resp: &mut Self::ResponseWrap, _headers: &HeaderMap<HeaderValue>) {}
}

#[derive(Debug, Deserialize, Default, thiserror::Error)]
#[serde(rename_all = "PascalCase")]
#[error("{code}: {message}")]
pub struct CodeMessage {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl CodeMessage {
    pub fn check(&self) -> Result<()> {
        if &self.code == "OK" || (self.code.is_empty() && self.message.is_empty()) {
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

/// Generic response type for APIs without strongly-typed response definitions.
/// This is used when an API produces JSON but doesn't define a 200 response schema.
#[derive(Debug, Default, serde::Deserialize)]
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

/// Macro to implement ToCodeMessage for Response types that have a flattened code_message field.
/// Use this in generated code or for Response types.
#[macro_export]
macro_rules! impl_to_code_message {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::ToCodeMessage for $ty {
                fn to_code_message(&self) -> &$crate::CodeMessage {
                    &self.code_message
                }
            }
        )*
    };
}
