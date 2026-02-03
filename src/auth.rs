//! Authorization algorithms for different Aliyun cloud products.
//!
//! Different Aliyun cloud products may use different authorization algorithms:
//! - Most products use `ACS3-HMAC-SHA256`
//! - OSS uses `OSS4-HMAC-SHA256`

use crate::QueryValue;
use crate::Result;
use anyhow::Context;
use http::HeaderMap;
use std::borrow::Cow;

mod acs3;
mod oss4;

pub use acs3::Acs3HmacSha256;
pub use oss4::Oss4HmacSha256;

/// Type of ali access key and secret key.
#[derive(Clone, Debug)]
pub struct AccessKeySecret(Cow<'static, str>, Cow<'static, str>);

impl AccessKeySecret {
    #[inline]
    pub fn new(key: impl Into<Cow<'static, str>>, secret: impl Into<Cow<'static, str>>) -> Self {
        Self(key.into(), secret.into())
    }

    /// Returns the access key ID.
    #[inline]
    pub fn access_key_id(&self) -> &str {
        &self.0
    }

    /// Returns the access key secret.
    #[inline]
    pub fn access_key_secret(&self) -> &str {
        &self.1
    }
}

impl From<(&'static str, &'static str)> for AccessKeySecret {
    fn from(value: (&'static str, &'static str)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

impl From<(Cow<'static, str>, Cow<'static, str>)> for AccessKeySecret {
    fn from(value: (Cow<'static, str>, Cow<'static, str>)) -> Self {
        Self(value.0, value.1)
    }
}

/// Trait for different Aliyun authorization algorithms.
///
/// Different Aliyun cloud products may use different authorization algorithms:
/// - Most products use `ACS3-HMAC-SHA256` (implemented by [`Acs3HmacSha256`])
/// - OSS uses `OSS4-HMAC-SHA256` (implemented by [`Oss4HmacSha256`])
pub trait AliyunAuth: Send + Sync {
    /// Generate the Authorization header value.
    ///
    /// # Arguments
    /// * `headers` - The request headers (should include x-acs-* headers, host, content-type, etc.)
    ///               This is mutable to allow the implementation to add required headers.
    /// * `path` - The canonical URI path (e.g., "/")
    /// * `query_string` - The canonical query string (already URL-encoded and sorted)
    /// * `method` - The HTTP method (GET, POST, PUT, etc.)
    /// * `hashed_payload` - The hex-encoded SHA256 hash of the request body
    ///
    /// # Returns
    /// The complete Authorization header value.
    fn sign(
        &self,
        headers: &mut HeaderMap,
        path: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String>;

    /// Build canonical query string from query parameters.
    ///
    /// Different authentication algorithms have different requirements:
    /// - **ACS3**: Sort by parameter name, always include "=" even for empty values
    /// - **OSS4**: Sort by URL-encoded parameter name, omit "=" for empty values
    ///
    /// # Arguments
    /// * `values` - Vector of (key, value) pairs for query parameters
    ///
    /// # Returns
    /// The canonical query string.
    fn canonical_query_string(&self, values: Vec<(Cow<'static, str>, QueryValue)>) -> String;

    /// Create headers required for authentication.
    ///
    /// Different authentication algorithms require different headers:
    /// - **ACS3**: x-acs-action, x-acs-version, x-acs-signature-nonce, x-acs-date, x-acs-content-sha256
    /// - **OSS4**: Only x-oss-content-sha256 (other headers like x-oss-date are added during signing)
    ///
    /// # Arguments
    /// * `action` - The API action to perform (e.g., "DescribeInstances")
    /// * `version` - The API version (e.g., "2014-05-26")
    /// * `hashed_content` - The hex-encoded SHA256 hash of the request body
    ///
    /// # Returns
    /// A HeaderMap containing the authentication headers.
    fn create_headers(
        &self,
        action: &str,
        version: &str,
        hashed_content: &str,
    ) -> Result<HeaderMap>;
}

/// Compute HMAC-SHA256 with raw key bytes, returning raw bytes.
fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key).context("Create hmac from key")?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Compute signature String using HMAC-SHA256, and encode use hex.
fn hexed_hmac_sha256(key: &[u8], to_sign: &[u8]) -> Result<String> {
    let result = hmac_sha256(key, to_sign)?;
    Ok(hex::encode(result))
}

/// SHA256 hash the input and hex encode the result.
fn hexed_sha256(s: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(s);
    hex::encode(hasher.finalize())
}

/// Percent-encodes a string per RFC 3986.
///
/// The `urlencoding` crate's default behavior matches Aliyun's requirements:
/// - Spaces are encoded as `%20` (not `+`)
/// - The `*` character is encoded as `%2A`
/// - The `~` character is not encoded
fn percent_encode(s: &str) -> Cow<'_, str> {
    urlencoding::encode(s)
}
