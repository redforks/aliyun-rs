//! Response body deserialization traits and implementations.

use crate::{CodeMessage, Result};
use anyhow::Context as _;
use serde::de::DeserializeOwned;

/// Default CodeMessage for binary responses (which don't have structured response data)
pub static CODE_MESSAGE: CodeMessage = CodeMessage {
    code: String::new(),
    message: String::new(),
};

/// Trait for types that can provide a reference to CodeMessage.
/// This is used instead of AsRef<CodeMessage> to allow implementations
/// for types like Vec<u8> that need to be deserialized into CodeMessage.
pub trait ToCodeMessage {
    fn to_code_message(&self) -> &CodeMessage;
}

/// Trait for types that can be deserialized from raw response bytes.
pub trait FromBody: Sized {
    fn from_body(bytes: Vec<u8>) -> Result<Self>;
}

/// Trait for converting wrapper types into their inner response type.
pub trait IntoResponse {
    type Response;

    fn into_response(self) -> Self::Response;
}

/// Trait for types that can store binary data with metadata.
pub trait BinaryWithMeta {
    fn set_binary(&mut self, body: Vec<u8>);
}

/// JSON response wrapper that deserializes the inner type from JSON bytes.
#[derive(Debug)]
pub struct JsonResponseWrap<T> {
    pub inner: T,
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

impl<T> IntoResponse for JsonResponseWrap<T> {
    type Response = T;

    fn into_response(self) -> Self::Response {
        self.inner
    }
}

/// Response wrapper that returns a default value (for APIs with no response body).
#[derive(Debug)]
pub struct SelfResponseWrap<T> {
    pub inner: T,
}

impl<T> IntoResponse for SelfResponseWrap<T> {
    type Response = T;

    fn into_response(self) -> Self::Response {
        self.inner
    }
}

impl<T: Default> FromBody for SelfResponseWrap<T> {
    fn from_body(_bytes: Vec<u8>) -> Result<Self> {
        Ok(SelfResponseWrap {
            inner: T::default(),
        })
    }
}

impl<T> ToCodeMessage for SelfResponseWrap<T> {
    fn to_code_message(&self) -> &CodeMessage {
        &CODE_MESSAGE
    }
}

impl FromBody for () {
    fn from_body(_bytes: Vec<u8>) -> Result<Self> {
        Ok(())
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
pub struct XmlResponseWrap<T> {
    pub inner: T,
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
pub struct BinaryResponseWrap {
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

/// Binary response wrapper with metadata from response headers.
/// Wraps a struct containing `inner: T` (metadata from headers) and `body: Vec<u8>` (raw bytes).
#[derive(Debug)]
pub struct BinaryResponseWithMetaWrap<T: BinaryWithMeta + Default> {
    /// Metadata extracted from response headers
    pub inner: T,
}

impl<T: BinaryWithMeta + Default> FromBody for BinaryResponseWithMetaWrap<T> {
    fn from_body(bytes: Vec<u8>) -> Result<Self> {
        let mut inner: T = Default::default();
        inner.set_binary(bytes);
        Ok(Self { inner })
    }
}

impl<T: BinaryWithMeta + Default> ToCodeMessage for BinaryResponseWithMetaWrap<T> {
    fn to_code_message(&self) -> &CodeMessage {
        &CODE_MESSAGE
    }
}

impl<T: BinaryWithMeta + Default> IntoResponse for BinaryResponseWithMetaWrap<T> {
    type Response = T;

    fn into_response(self) -> Self::Response {
        self.inner
    }
}
