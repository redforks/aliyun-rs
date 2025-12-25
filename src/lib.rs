use anyhow::anyhow;
use http::{HeaderValue, Method};
use reqwest::Body;
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::BTreeMap;

mod common;
mod v3;

#[cfg(test)]
mod sample;

pub mod sms;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Ali(#[from] CodeMessage),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

enum QueryValue<'a> {
    Str(&'a str),
    OptStr(Option<&'a str>),
    I64(&'a i64),
    Bool(&'a bool),
    OptI64(Option<&'a i64>),
    OptBool(Option<&'a bool>),
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

impl<'a> From<Option<&'a str>> for QueryValue<'a> {
    fn from(value: Option<&'a str>) -> Self {
        Self::OptStr(value)
    }
}

impl<'a> From<&'a Option<String>> for QueryValue<'a> {
    fn from(value: &'a Option<String>) -> Self {
        Self::OptStr(value.as_deref())
    }
}

impl<'a> From<&'a i64> for QueryValue<'a> {
    fn from(value: &'a i64) -> Self {
        Self::I64(value)
    }
}

impl<'a> From<&'a bool> for QueryValue<'a> {
    fn from(value: &'a bool) -> Self {
        Self::Bool(value)
    }
}

impl<'a> From<Option<&'a i64>> for QueryValue<'a> {
    fn from(value: Option<&'a i64>) -> Self {
        Self::OptI64(value)
    }
}

impl<'a> From<&'a Option<i64>> for QueryValue<'a> {
    fn from(value: &'a Option<i64>) -> Self {
        Self::OptI64(value.as_ref())
    }
}

impl<'a> From<Option<&'a bool>> for QueryValue<'a> {
    fn from(value: Option<&'a bool>) -> Self {
        Self::OptBool(value)
    }
}

impl<'a> From<&'a Option<bool>> for QueryValue<'a> {
    fn from(value: &'a Option<bool>) -> Self {
        Self::OptBool(value.as_ref())
    }
}

impl<'a> QueryValue<'a> {
    fn to_query_value(&self) -> Option<String> {
        match self {
            QueryValue::Str(v) => Some(v.to_string()),
            QueryValue::OptStr(v) => v.map(|s| s.to_string()),
            QueryValue::I64(v) => Some(v.to_string()),
            QueryValue::Bool(v) => Some(if **v {
                "true".to_string()
            } else {
                "false".to_string()
            }),
            QueryValue::OptI64(v) => v.map(|i| i.to_string()),
            QueryValue::OptBool(v) => v.map(|b| {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }),
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

    fn to_query_params(&self) -> Result<BTreeMap<&'static str, QueryValue>> {
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
