use anyhow::anyhow;
use http::{HeaderValue, Method};
use reqwest::Body;
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::BTreeMap;

mod common;
mod v3;

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

    fn to_query_params(&self) -> Result<BTreeMap<&'static str, QueryValue<'_>>> {
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

#[derive(Debug, Deserialize, thiserror::Error)]
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
