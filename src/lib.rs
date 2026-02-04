#![doc = include_str!("../README.md")]

use anyhow::Context as _;
use anyhow::anyhow;
use http::HeaderMap;
use http::{HeaderValue, Method};
use reqwest::Body;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::debug;

mod auth;
mod common;
mod v3;

#[cfg(feature = "ocr")]
pub mod ocr;

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(feature = "ecs")]
pub mod ecs;

#[cfg(feature = "fc")]
pub mod fc;

#[cfg(feature = "oss")]
pub mod oss;

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

enum QueryValue<'a> {
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

impl From<&i64> for QueryValue<'_> {
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

impl From<&i32> for QueryValue<'_> {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl From<&bool> for QueryValue<'_> {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl<'a> From<serde_json::Value> for QueryValue<'a> {
    fn from(value: serde_json::Value) -> Self {
        Self::Json(value)
    }
}

impl From<f32> for QueryValue<'_> {
    fn from(value: f32) -> Self {
        Self::OwnedStr(value.to_string())
    }
}

impl From<&f32> for QueryValue<'_> {
    fn from(value: &f32) -> Self {
        Self::OwnedStr(value.to_string())
    }
}

impl<'a> QueryValue<'a> {
    fn to_query_value(&self) -> Cow<'_, str> {
        match self {
            QueryValue::Str(v) => (*v).into(),
            QueryValue::I64(v) => v.to_string().into(),
            QueryValue::I32(v) => v.to_string().into(),
            QueryValue::Bool(v) => {
                if *v {
                    Cow::Borrowed("true")
                } else {
                    Cow::Borrowed("false")
                }
            }
            QueryValue::Json(v) => v.to_string().into(),
            QueryValue::OwnedStr(v) => v.as_str().into(),
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
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    );
}

impl FlatSerialize for String {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.into()));
    }
}

impl FlatSerialize for str {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.into()));
    }
}

impl FlatSerialize for i32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for i64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for bool {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for f32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.to_string().into()));
    }
}

impl FlatSerialize for f64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.to_string().into()));
    }
}

impl<T: FlatSerialize> FlatSerialize for Vec<T> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
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
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if let Some(v) = self {
            v.flat_serialize(name, params);
        }
    }
}

/// Trait for types that can be serialized as comma-separated values.
/// Used for ParameterStyle::Simple with array types.
///
/// This trait handles the serialization of arrays into a single comma-separated string,
/// e.g., `["a", "b", "c"]` becomes `"a,b,c"`.
pub(crate) trait SimpleSerialize {
    /// Serialize this value as a comma-separated string and insert into the map.
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    );
}

impl SimpleSerialize for Vec<String> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self.join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<i32> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<i64> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<bool> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| if *v { "true" } else { "false" })
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<f32> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<f64> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl<T: SimpleSerialize> SimpleSerialize for Option<T> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if let Some(v) = self {
            v.simple_serialize(name, params);
        }
    }
}

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

impl FlatSerialize for Value {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        match self {
            Value::Null => {}
            Value::Bool(v) => {
                params.push((name.to_owned().into(), (*v).into()));
            }
            Value::Integer(v) => {
                params.push((name.to_owned().into(), (*v).into()));
            }
            Value::Float(v) => {
                params.push((name.to_owned().into(), QueryValue::OwnedStr(v.to_string())));
            }
            Value::String(v) => {
                params.push((name.to_owned().into(), v.as_str().into()));
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
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
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
    fn to_form_data(&self) -> Vec<(Cow<'static, str>, QueryValue<'_>)>;
}

/// Trait for types that can provide a reference to CodeMessage.
/// This is used instead of AsRef<CodeMessage> to allow implementations
/// for types like Vec<u8> that need to be deserialized into CodeMessage.
trait ToCodeMessage {
    fn to_code_message(&self) -> &CodeMessage;
}

/// Trait for types that can be deserialized from raw response bytes.
trait FromBody: Sized {
    fn from_body(bytes: Vec<u8>) -> Result<Self>;
}

/// JSON response wrapper that deserializes the inner type from JSON bytes.
#[derive(Debug)]
struct JsonResponseWrap<T> {
    inner: T,
}

impl<T: DeserializeOwned> FromBody for JsonResponseWrap<T> {
    fn from_body(bytes: Vec<u8>) -> Result<Self> {
        let text = String::from_utf8(bytes).context("Response body is not valid UTF-8")?;
        let inner = serde_json::from_str(&text)
            .with_context(|| format!("Decode response as JSON: {}", &text))?;
        Ok(Self { inner })
    }
}

impl<T: ToCodeMessage> ToCodeMessage for JsonResponseWrap<T> {
    fn to_code_message(&self) -> &CodeMessage {
        self.inner.to_code_message()
    }
}

trait IntoResponse {
    type Response;

    fn into_response(self) -> Self::Response;
}

// Compatibility layer: Allow Response type to be convertible from JsonResponseWrap<Response>
// This supports the new pattern where Response: From<ResponseWrap>
impl<T> IntoResponse for JsonResponseWrap<T> {
    type Response = T;

    fn into_response(self) -> Self::Response {
        self.inner
    }
}

// Compatibility layer: For existing code that uses Response directly
impl<T: DeserializeOwned + ToCodeMessage + Default> FromBody for T {
    fn from_body(bytes: Vec<u8>) -> Result<Self> {
        // For empty response body, return default value (useful for unit type)
        if bytes.is_empty() {
            return Ok(T::default());
        }
        let inner = serde_json::from_slice(&bytes).with_context(|| {
            format!(
                "Decode response as JSON: {}",
                &String::from_utf8_lossy(&bytes)
            )
        })?;
        Ok(inner)
    }
}

// Unit type implementation for empty responses
impl ToCodeMessage for () {
    fn to_code_message(&self) -> &CodeMessage {
        &CODE_MESSAGE
    }
}

impl IntoResponse for () {
    type Response = ();

    fn into_response(self) -> Self::Response {
        self
    }
}

/// XML response wrapper that deserializes the inner type from XML bytes.
#[derive(Debug)]
struct XmlResponseWrap<T> {
    inner: T,
}

impl<T: DeserializeOwned> FromBody for XmlResponseWrap<T> {
    fn from_body(bytes: Vec<u8>) -> Result<Self> {
        let inner = quick_xml::de::from_reader(&bytes[..]).context("Decode response as XML")?;
        Ok(Self { inner })
    }
}

impl<T: ToCodeMessage> ToCodeMessage for XmlResponseWrap<T> {
    fn to_code_message(&self) -> &CodeMessage {
        self.inner.to_code_message()
    }
}

impl<T> IntoResponse for XmlResponseWrap<T> {
    type Response = T;

    fn into_response(self) -> Self::Response {
        self.inner
    }
}

/// Binary response wrapper that passes through raw bytes without deserialization.
#[derive(Debug)]
struct BinaryResponseWrap {
    /// The raw response bytes
    pub inner: Vec<u8>,
}

impl FromBody for BinaryResponseWrap {
    fn from_body(bytes: Vec<u8>) -> Result<Self> {
        Ok(Self { inner: bytes })
    }
}

impl ToCodeMessage for BinaryResponseWrap {
    fn to_code_message(&self) -> &CodeMessage {
        &CODE_MESSAGE
    }
}

impl IntoResponse for BinaryResponseWrap {
    type Response = Vec<u8>;

    fn into_response(self) -> Self::Response {
        self.inner
    }
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

trait IntoBody {
    fn content_type(&self) -> Option<HeaderValue>;
    fn into_body(self) -> Result<Body>;
}

impl IntoBody for () {
    fn content_type(&self) -> Option<HeaderValue> {
        None
    }

    fn into_body(self) -> Result<Body> {
        Ok(b"".as_slice().into())
    }
}

pub(crate) struct Form<T: ToFormData>(pub T);

impl<T: ToFormData> IntoBody for Form<T> {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static(
            "application/x-www-form-urlencoded",
        ))
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

/// Body wrapper for binary data using application/octet-stream content type.
pub(crate) struct OctetStream(pub Vec<u8>);

impl IntoBody for OctetStream {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static("application/octet-stream"))
    }

    fn into_body(self) -> Result<Body> {
        Ok(self.0.into())
    }
}

/// Body wrapper for XML-serialized data using application/xml content type.
pub(crate) struct XmlBody<T: serde::Serialize>(pub T);

impl<T: serde::Serialize> IntoBody for XmlBody<T> {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static("application/xml"))
    }

    fn into_body(self) -> Result<Body> {
        let xml = quick_xml::se::to_string(&self.0).context("Failed to serialize body to XML")?;
        debug!("XML Request body: {}", xml);
        Ok(xml.into())
    }
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

/// Default CodeMessage for binary responses (which don't have structured response data)
static CODE_MESSAGE: CodeMessage = CodeMessage {
    code: String::new(),
    message: String::new(),
};

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
