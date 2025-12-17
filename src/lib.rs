use http::{HeaderValue, Method};
use reqwest::Body;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use thiserror::Error;

mod common;
mod v3;

pub type Error = anyhow::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Each api entry should implement this trait.
trait Request: Sized + Send {
    const METHOD: Method;
    const URL_PATH: &'static str = "/";
    const ACTION: &'static str;
    const VERSION: &'static str;
    const END_POINT: &'static str;
    /// Request body, will serialize to json. Use unit type if no request body.
    /// Not used if Method is GET.
    type Body: IntoBody + Send;
    /// Result data if no error.
    type Result;
    /// Response json type
    type Response: DeserializeOwned + Into<Result<Self::Result>>;

    fn to_query_params(&self) -> Result<BTreeMap<&'static str, String>> {
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
