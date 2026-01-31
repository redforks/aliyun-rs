//! Authorization algorithms for different Aliyun cloud products.
//!
//! Different Aliyun cloud products may use different authorization algorithms:
//! - Most products use `ACS3-HMAC-SHA256`
//! - OSS uses `OSS4-HMAC-SHA256`

use std::borrow::Cow;

use anyhow::Context;
use http::HeaderMap;

use crate::Result;

/// Type of ali access key and secret key.
#[derive(Clone, Debug)]
pub struct AccessKeySecret(pub Cow<'static, str>, pub Cow<'static, str>);

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

/// Default implementation of AliyunAuth for AccessKeySecret using ACS3-HMAC-SHA256.
/// This provides backward compatibility for existing code.
impl AliyunAuth for AccessKeySecret {
    fn sign(
        &self,
        headers: &HeaderMap,
        url: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String> {
        Acs3HmacSha256(self.clone()).sign(headers, url, query_string, method, hashed_payload)
    }

    fn access_key_id(&self) -> &str {
        &self.0
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
    /// * `url` - The canonical URI path (e.g., "/")
    /// * `query_string` - The canonical query string (already URL-encoded and sorted)
    /// * `method` - The HTTP method (GET, POST, PUT, etc.)
    /// * `hashed_payload` - The hex-encoded SHA256 hash of the request body
    ///
    /// # Returns
    /// The complete Authorization header value.
    fn sign(
        &self,
        headers: &HeaderMap,
        url: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String>;

    /// Returns the access key ID.
    fn access_key_id(&self) -> &str;
}

/// ACS3-HMAC-SHA256 authorization algorithm.
///
/// This is the standard authorization algorithm used by most Aliyun cloud products
/// like ECS, SMS, FC, OCR, etc.
#[derive(Clone, Debug)]
pub struct Acs3HmacSha256(pub AccessKeySecret);

impl Acs3HmacSha256 {
    pub fn new(key: impl Into<Cow<'static, str>>, secret: impl Into<Cow<'static, str>>) -> Self {
        Self(AccessKeySecret::new(key, secret))
    }
}

impl From<AccessKeySecret> for Acs3HmacSha256 {
    fn from(key_secret: AccessKeySecret) -> Self {
        Self(key_secret)
    }
}

const ACS3_SIGNATURE_ALGORITHM: &str = "ACS3-HMAC-SHA256";

impl AliyunAuth for Acs3HmacSha256 {
    fn sign(
        &self,
        headers: &HeaderMap,
        url: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String> {
        use std::collections::BTreeMap;

        // Build canonical headers and signed headers
        let headers_map = headers
            .iter()
            .filter_map(|(k, v)| {
                let k = k.as_str().to_lowercase();
                (k.starts_with("x-acs-") || k == "host" || k == "content-type").then(|| {
                    let v = v.to_str().context("convert header value to string")?;
                    Ok((k, v))
                })
            })
            .collect::<Result<BTreeMap<_, _>>>()?;

        let mut canonical_headers = String::new();
        let mut signed_headers = String::new();
        for (k, v) in &headers_map {
            canonical_headers.push_str(k);
            canonical_headers.push(':');
            canonical_headers.push_str(v);
            canonical_headers.push('\n');
            signed_headers.push_str(k);
            signed_headers.push(';');
        }
        // Remove last ';' from signed_headers
        signed_headers.pop();

        // Build canonical request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method, url, query_string, canonical_headers, signed_headers, hashed_payload
        );

        // Hash the canonical request
        let hashed_canonical_request = hexed_sha256(&canonical_request);

        // Build string to sign
        let string_to_sign = format!("{}\n{}", ACS3_SIGNATURE_ALGORITHM, hashed_canonical_request);

        // Calculate signature
        let signature = hexed_hmac_sha256(&self.0.access_key_secret(), string_to_sign.as_bytes())?;

        // Build Authorization header
        Ok(format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            ACS3_SIGNATURE_ALGORITHM,
            self.0.access_key_id(),
            signed_headers,
            signature
        ))
    }

    fn access_key_id(&self) -> &str {
        self.0.access_key_id()
    }
}

/// OSS4-HMAC-SHA256 authorization algorithm.
///
/// This authorization algorithm is used by Aliyun OSS (Object Storage Service).
/// It follows a similar pattern to ACS3-HMAC-SHA256 but with OSS-specific headers.
#[derive(Clone, Debug)]
pub struct Oss4HmacSha256(pub AccessKeySecret);

impl Oss4HmacSha256 {
    pub fn new(key: impl Into<Cow<'static, str>>, secret: impl Into<Cow<'static, str>>) -> Self {
        Self(AccessKeySecret::new(key, secret))
    }
}

impl From<AccessKeySecret> for Oss4HmacSha256 {
    fn from(key_secret: AccessKeySecret) -> Self {
        Self(key_secret)
    }
}

const OSS4_SIGNATURE_ALGORITHM: &str = "OSS4-HMAC-SHA256";

impl AliyunAuth for Oss4HmacSha256 {
    fn sign(
        &self,
        headers: &HeaderMap,
        url: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String> {
        use std::collections::BTreeMap;

        // Build canonical headers and signed headers
        // OSS uses x-oss-* headers instead of x-acs-* headers
        let headers_map = headers
            .iter()
            .filter_map(|(k, v)| {
                let k = k.as_str().to_lowercase();
                (k.starts_with("x-oss-") || k == "host" || k == "content-type").then(|| {
                    let v = v.to_str().context("convert header value to string")?;
                    Ok((k, v))
                })
            })
            .collect::<Result<BTreeMap<_, _>>>()?;

        let mut canonical_headers = String::new();
        let mut signed_headers = String::new();
        for (k, v) in &headers_map {
            canonical_headers.push_str(k);
            canonical_headers.push(':');
            canonical_headers.push_str(v);
            canonical_headers.push('\n');
            signed_headers.push_str(k);
            signed_headers.push(';');
        }
        // Remove last ';' from signed_headers
        signed_headers.pop();

        // Build canonical request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method, url, query_string, canonical_headers, signed_headers, hashed_payload
        );

        // Hash the canonical request
        let hashed_canonical_request = hexed_sha256(&canonical_request);

        // Build string to sign
        let string_to_sign = format!("{}\n{}", OSS4_SIGNATURE_ALGORITHM, hashed_canonical_request);

        // Calculate signature
        let signature = hexed_hmac_sha256(&self.0.access_key_secret(), string_to_sign.as_bytes())?;

        // Build Authorization header
        Ok(format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            OSS4_SIGNATURE_ALGORITHM,
            self.0.access_key_id(),
            signed_headers,
            signature
        ))
    }

    fn access_key_id(&self) -> &str {
        self.0.access_key_id()
    }
}

/// Compute signature String using HMAC-SHA256, and encode use hex.
fn hexed_hmac_sha256(key: &str, to_sign: &[u8]) -> Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())
        .context("Create hmac from ali-key-secret")?;
    mac.update(to_sign);
    let result = mac.finalize().into_bytes();
    Ok(hex::encode(result))
}

/// SHA256 hash the input and hex encode the result.
fn hexed_sha256(s: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(s);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::HeaderValue;

    fn create_test_headers_acs3() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("ecs.aliyuncs.com"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert(
            "x-acs-action",
            HeaderValue::from_static("DescribeInstances"),
        );
        headers.insert("x-acs-version", HeaderValue::from_static("2014-05-26"));
        headers.insert(
            "x-acs-signature-nonce",
            HeaderValue::from_static("test-nonce-12345"),
        );
        headers.insert(
            "x-acs-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        );
        headers.insert(
            "x-acs-content-sha256",
            HeaderValue::from_static(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
        );
        headers
    }

    fn create_test_headers_oss4() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "host",
            HeaderValue::from_static("bucket.oss-cn-hangzhou.aliyuncs.com"),
        );
        headers.insert("content-type", HeaderValue::from_static("application/xml"));
        headers.insert(
            "x-oss-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        );
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
        );
        headers
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_empty_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_with_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_acs3();
        let query_string = "PageSize=10&RegionId=cn-hangzhou";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_acs3_hmac_sha256_deterministic() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result1 = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_acs3_hmac_sha256_access_key_id() {
        let auth = Acs3HmacSha256::new("my-key-id", "my-secret");
        assert_eq!(auth.access_key_id(), "my-key-id");
    }

    #[test]
    fn test_oss4_hmac_sha256_sign_empty_query() {
        let auth = Oss4HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_oss4();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("OSS4-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_oss4_hmac_sha256_sign_with_query() {
        let auth = Oss4HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_oss4();
        let query_string = "max-keys=100&prefix=test%2F";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("OSS4-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_oss4_hmac_sha256_deterministic() {
        let auth = Oss4HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let headers = create_test_headers_oss4();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result1 = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_oss4_hmac_sha256_access_key_id() {
        let auth = Oss4HmacSha256::new("my-key-id", "my-secret");
        assert_eq!(auth.access_key_id(), "my-key-id");
    }

    #[test]
    fn test_oss4_uses_x_oss_headers() {
        // OSS4 should include x-oss-* headers but not x-acs-* headers
        let auth = Oss4HmacSha256::new("test-key", "test-secret");
        let mut headers = HeaderMap::new();
        headers.insert(
            "host",
            HeaderValue::from_static("bucket.oss-cn-hangzhou.aliyuncs.com"),
        );
        headers.insert(
            "x-oss-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        );
        headers.insert(
            "x-acs-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        ); // This should be ignored
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        // The signed headers should include x-oss-date but not x-acs-date
        assert!(result.contains("x-oss-date"));
        assert!(!result.contains("x-acs-date"));
    }

    #[test]
    fn test_acs3_uses_x_acs_headers() {
        // ACS3 should include x-acs-* headers but not x-oss-* headers
        let auth = Acs3HmacSha256::new("test-key", "test-secret");
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("ecs.aliyuncs.com"));
        headers.insert(
            "x-acs-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        );
        headers.insert(
            "x-oss-date",
            HeaderValue::from_static("2024-01-01T00:00:00Z"),
        ); // This should be ignored
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        // The signed headers should include x-acs-date but not x-oss-date
        assert!(result.contains("x-acs-date"));
        assert!(!result.contains("x-oss-date"));
    }

    #[test]
    fn test_different_methods_produce_different_signatures() {
        let auth = Acs3HmacSha256::new("test-key", "test-secret");
        let headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let get_result = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let post_result = auth
            .sign(&headers, "/", query_string, "POST", hashed_payload)
            .unwrap();

        assert_ne!(get_result, post_result);
    }

    #[test]
    fn test_different_urls_produce_different_signatures() {
        let auth = Acs3HmacSha256::new("test-key", "test-secret");
        let headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result1 = auth
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&headers, "/test/path", query_string, "GET", hashed_payload)
            .unwrap();

        assert_ne!(result1, result2);
    }

    #[test]
    fn test_hexed_sha256() {
        // Empty string hash
        let empty_hash = hexed_sha256("");
        assert_eq!(
            empty_hash,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );

        // Known test vector
        let hello_hash = hexed_sha256("hello");
        assert_eq!(
            hello_hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_access_key_secret_backward_compatibility() {
        // Test that AccessKeySecret implements AliyunAuth for backward compatibility
        let key_secret = AccessKeySecret::new("test-key", "test-secret");
        let headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = key_secret
            .sign(&headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        // Should produce a valid ACS3-HMAC-SHA256 signature
        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-key,"));
    }
}
