#![doc = include_str!("../README.md")]

use anyhow::anyhow;
use http::HeaderMap;
use http::{HeaderValue, Method};
use serde::Deserialize;
use std::borrow::Cow;

mod auth;
mod common;
mod from_body;
mod into_body;
mod open_object;
mod query_value;
mod serializes;
mod v3;

use from_body::{
    BinaryResponseWithMetaWrap, BinaryResponseWrap, BinaryWithMeta, CODE_MESSAGE, FromBody,
    IntoResponse, JsonResponseWrap, SelfResponseWrap, ToCodeMessage, XmlResponseWrap,
};
use into_body::{Form, IntoBody, JsonBody, OctetStream, XmlBody};
pub use open_object::{OpenObject, OpenObjectResponse, Value};
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

/// Macro to implement ToCodeMessage for Response types that have a flattened code_message field.
/// Use this in generated code or for Response types.
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
pub(crate) use impl_to_code_message;

/// Macro to implement ToCodeMessage to return &crate::CODE_MESSAGE
/// Use this in generated code or for Response types.
macro_rules! impl_default_to_code_message {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $crate::ToCodeMessage for $ty {
                fn to_code_message(&self) -> &$crate::CodeMessage {
                    &$crate::CODE_MESSAGE
                }
            }
        )*
    };
}
pub(crate) use impl_default_to_code_message;
