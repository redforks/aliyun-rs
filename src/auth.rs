//! Authorization algorithms for different Aliyun cloud products.
//!
//! Different Aliyun cloud products may use different authorization algorithms:
//! - Most products use `ACS3-HMAC-SHA256`
//! - OSS uses `OSS4-HMAC-SHA256`

use std::borrow::Cow;

use anyhow::Context;
use http::{HeaderMap, HeaderValue};
use time::OffsetDateTime;

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
    /// * `url` - The canonical URI path (e.g., "/")
    /// * `query_string` - The canonical query string (already URL-encoded and sorted)
    /// * `method` - The HTTP method (GET, POST, PUT, etc.)
    /// * `hashed_payload` - The hex-encoded SHA256 hash of the request body
    ///
    /// # Returns
    /// The complete Authorization header value.
    fn sign(
        &self,
        headers: &mut HeaderMap,
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
        headers: &mut HeaderMap,
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
/// It uses a different signing process than ACS3-HMAC-SHA256:
/// - Credential format: `<AccessKeyId>/<SignDate>/<SignRegion>/oss/aliyun_v4_request`
/// - Signing key derivation: multi-step HMAC with date, region, service, and version
/// - String to sign includes timestamp and scope
#[derive(Clone, Debug)]
pub struct Oss4HmacSha256 {
    pub credentials: AccessKeySecret,
    pub region: Cow<'static, str>,
}

impl Oss4HmacSha256 {
    pub fn new(key_secret: AccessKeySecret, region: impl Into<Cow<'static, str>>) -> Self {
        Self {
            credentials: key_secret,
            region: region.into(),
        }
    }

    /// Create from AccessKeySecret with a specific region.
    pub fn with_region(credentials: AccessKeySecret, region: impl Into<Cow<'static, str>>) -> Self {
        Self {
            credentials,
            region: region.into(),
        }
    }
}

const OSS4_SIGNATURE_ALGORITHM: &str = "OSS4-HMAC-SHA256";

impl AliyunAuth for Oss4HmacSha256 {
    fn sign(
        &self,
        headers: &mut HeaderMap,
        url: &str,
        query_string: &str,
        method: &str,
        hashed_payload: &str,
    ) -> Result<String> {
        use std::collections::BTreeMap;

        // Insert x-oss-date header if not present
        if !headers.contains_key("x-oss-date") {
            let timestamp = format_oss_timestamp(OffsetDateTime::now_utc())?;
            headers.insert(
                "x-oss-date",
                HeaderValue::from_str(&timestamp).context("convert timestamp to header value")?,
            );
        }

        // Insert x-oss-content-sha256 header if not present
        if !headers.contains_key("x-oss-content-sha256") {
            headers.insert(
                "x-oss-content-sha256",
                HeaderValue::from_static("UNSIGNED-PAYLOAD"),
            );
        }

        // Extract timestamp from x-oss-date header (required)
        let timestamp = headers
            .get("x-oss-date")
            .context("x-oss-date header is required for OSS V4 signing")?
            .to_str()
            .context("convert x-oss-date header value to string")?;

        // Extract sign date (YYYYMMDD) from timestamp (format: 20250411T064124Z)
        let sign_date = &timestamp[..8];

        // Build canonical headers and additional headers
        // Required headers that must participate in signing:
        // - x-oss-content-sha256 (required)
        // Headers that participate if present:
        // - content-type
        // - content-md5
        // - x-oss-* headers
        // Additional headers specified by the caller
        let mut required_headers: BTreeMap<String, &str> = BTreeMap::new();
        let mut additional_headers: BTreeMap<String, &str> = BTreeMap::new();

        for (k, v) in headers.iter() {
            let key = k.as_str().to_lowercase();
            let value = v.to_str().context("convert header value to string")?;

            if key.starts_with("x-oss-") {
                // x-oss-* headers are required in signing
                required_headers.insert(key, value);
            } else if key == "content-type" || key == "content-md5" {
                // content-type and content-md5 are additional headers if present
                additional_headers.insert(key, value);
            }
        }

        // Build canonical headers string (required + additional, sorted)
        let mut all_headers: BTreeMap<&str, &str> = BTreeMap::new();
        for (k, v) in &required_headers {
            all_headers.insert(k.as_str(), *v);
        }
        for (k, v) in &additional_headers {
            all_headers.insert(k.as_str(), *v);
        }

        let mut canonical_headers = String::new();
        for (k, v) in &all_headers {
            canonical_headers.push_str(k);
            canonical_headers.push(':');
            canonical_headers.push_str(v.trim());
            canonical_headers.push('\n');
        }

        // Build additional headers string (only the optional ones, semicolon-separated)
        let additional_headers_str: String = additional_headers
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<_>>()
            .join(";");

        // Build canonical request
        // Format:
        // HTTP Verb + "\n" +
        // Canonical URI + "\n" +
        // Canonical Query String + "\n" +
        // Canonical Headers + "\n" +
        // Additional Headers + "\n" +
        // Hashed PayLoad
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method, url, query_string, canonical_headers, additional_headers_str, hashed_payload
        );

        // Hash the canonical request
        let hashed_canonical_request = hexed_sha256(&canonical_request);

        // Build scope
        let scope = format!("{}/{}/oss/aliyun_v4_request", sign_date, self.region);

        // Build string to sign
        // Format:
        // "OSS4-HMAC-SHA256" + "\n" +
        // TimeStamp + "\n" +
        // Scope + "\n" +
        // Hex(SHA256Hash(<CanonicalRequest>))
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            OSS4_SIGNATURE_ALGORITHM, timestamp, scope, hashed_canonical_request
        );

        // Calculate SigningKey using multi-step HMAC
        // DateKey = HMAC-SHA256("aliyun_v4" + SK, Date);
        // DateRegionKey = HMAC-SHA256(DateKey, Region);
        // DateRegionServiceKey = HMAC-SHA256(DateRegionKey, "oss");
        // SigningKey = HMAC-SHA256(DateRegionServiceKey, "aliyun_v4_request");
        let signing_key = derive_signing_key(
            self.credentials.access_key_secret(),
            sign_date,
            &self.region,
        )?;

        // Calculate signature
        let signature = hexed_hmac_sha256_with_key(&signing_key, string_to_sign.as_bytes())?;

        // Build Authorization header
        // Format: OSS4-HMAC-SHA256 Credential=<AccessKeyId>/<SignDate>/<SignRegion>/oss/aliyun_v4_request, AdditionalHeaders=<AdditionalHeadersVal>, Signature=<SignatureVal>
        let credential = format!("{}/{}", self.credentials.access_key_id(), scope);

        if additional_headers_str.is_empty() {
            Ok(format!(
                "{} Credential={}, Signature={}",
                OSS4_SIGNATURE_ALGORITHM, credential, signature
            ))
        } else {
            Ok(format!(
                "{} Credential={}, AdditionalHeaders={}, Signature={}",
                OSS4_SIGNATURE_ALGORITHM, credential, additional_headers_str, signature
            ))
        }
    }

    fn access_key_id(&self) -> &str {
        self.credentials.access_key_id()
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

/// Compute signature String using HMAC-SHA256 with raw key bytes, and encode use hex.
fn hexed_hmac_sha256_with_key(key: &[u8], to_sign: &[u8]) -> Result<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key).context("Create hmac from signing key")?;
    mac.update(to_sign);
    let result = mac.finalize().into_bytes();
    Ok(hex::encode(result))
}

/// Compute HMAC-SHA256 with raw key bytes, returning raw bytes.
fn hmac_sha256_raw(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key).context("Create hmac from key")?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Derive the signing key for OSS V4 signature.
///
/// The derivation process:
/// ```text
/// DateKey = HMAC-SHA256("aliyun_v4" + SK, Date);
/// DateRegionKey = HMAC-SHA256(DateKey, Region);
/// DateRegionServiceKey = HMAC-SHA256(DateRegionKey, "oss");
/// SigningKey = HMAC-SHA256(DateRegionServiceKey, "aliyun_v4_request");
/// ```
fn derive_signing_key(secret: &str, date: &str, region: &str) -> Result<Vec<u8>> {
    let initial_key = format!("aliyun_v4{}", secret);
    let date_key = hmac_sha256_raw(initial_key.as_bytes(), date.as_bytes())?;
    let date_region_key = hmac_sha256_raw(&date_key, region.as_bytes())?;
    let date_region_service_key = hmac_sha256_raw(&date_region_key, b"oss")?;
    let signing_key = hmac_sha256_raw(&date_region_service_key, b"aliyun_v4_request")?;
    Ok(signing_key)
}

/// SHA256 hash the input and hex encode the result.
fn hexed_sha256(s: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(s);
    hex::encode(hasher.finalize())
}

/// Format datetime to OSS timestamp format like: 20240101T000000Z
fn format_oss_timestamp(dt: OffsetDateTime) -> Result<String> {
    use time::format_description::well_known::{Iso8601, iso8601::EncodedConfig};
    use time::format_description::well_known::iso8601::Config;
    use time::format_description::well_known::iso8601::TimePrecision;

    const CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: None,
        })
        .encode();
    const FORMAT: Iso8601<CONFIG> = Iso8601::<CONFIG>;

    let formatted = dt
        .format(&FORMAT)
        .context("format OSS timestamp failed")?;

    // Convert from "2024-01-01T00:00:00Z" to "20240101T000000Z"
    let oss_formatted = formatted
        .replace("-", "")
        .replace(":", "");

    Ok(oss_formatted)
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
        headers.insert("x-oss-date", HeaderValue::from_static("20240101T000000Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );
        headers
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_empty_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let mut headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_with_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let mut headers = create_test_headers_acs3();
        let query_string = "PageSize=10&RegionId=cn-hangzhou";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_acs3_hmac_sha256_deterministic() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let mut headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result1 = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
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
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("test-access-key-id", "test-access-key-secret"),
            "cn-hangzhou",
        );
        let mut headers = create_test_headers_oss4();
        let query_string = "";
        let hashed_payload = "UNSIGNED-PAYLOAD";

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("OSS4-HMAC-SHA256 Credential=test-access-key-id/"));
        assert!(result.contains("/cn-hangzhou/oss/aliyun_v4_request"));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_oss4_hmac_sha256_sign_with_query() {
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("test-access-key-id", "test-access-key-secret"),
            "cn-hangzhou",
        );
        let mut headers = create_test_headers_oss4();
        let query_string = "max-keys=100&prefix=test%2F";
        let hashed_payload = "UNSIGNED-PAYLOAD";

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert!(result.starts_with("OSS4-HMAC-SHA256 Credential=test-access-key-id/"));
        assert!(result.contains("/cn-hangzhou/oss/aliyun_v4_request"));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_oss4_hmac_sha256_deterministic() {
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("test-access-key-id", "test-access-key-secret"),
            "cn-hangzhou",
        );
        let mut headers = create_test_headers_oss4();
        let query_string = "";
        let hashed_payload = "UNSIGNED-PAYLOAD";

        let result1 = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_oss4_hmac_sha256_access_key_id() {
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("my-key-id", "my-secret"),
            "cn-hangzhou",
        );
        assert_eq!(auth.access_key_id(), "my-key-id");
    }

    #[test]
    fn test_oss4_uses_x_oss_headers() {
        // OSS4 should include x-oss-* headers but not x-acs-* headers
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("test-key", "test-secret"),
            "cn-hangzhou",
        );
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20240101T000000Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );
        headers.insert("x-acs-date", HeaderValue::from_static("20240101T000000Z")); // This should be ignored
        let query_string = "";
        let hashed_payload = "UNSIGNED-PAYLOAD";

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        // The result should not contain x-acs-date
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
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();

        // The signed headers should include x-acs-date but not x-oss-date
        assert!(result.contains("x-acs-date"));
        assert!(!result.contains("x-oss-date"));
    }

    #[test]
    fn test_different_methods_produce_different_signatures() {
        let auth = Acs3HmacSha256::new("test-key", "test-secret");
        let mut headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let get_result = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let post_result = auth
            .sign(&mut headers, "/", query_string, "POST", hashed_payload)
            .unwrap();

        assert_ne!(get_result, post_result);
    }

    #[test]
    fn test_different_urls_produce_different_signatures() {
        let auth = Acs3HmacSha256::new("test-key", "test-secret");
        let mut headers = create_test_headers_acs3();
        let query_string = "";
        let hashed_payload = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let result1 = auth
            .sign(&mut headers, "/", query_string, "GET", hashed_payload)
            .unwrap();
        let result2 = auth
            .sign(&mut headers, "/test/path", query_string, "GET", hashed_payload)
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
    fn test_oss4_signing_key_derivation() {
        // Test the SigningKey derivation is deterministic and produces 32 bytes (256 bits)
        let signing_key = derive_signing_key("test-secret", "20250411", "cn-hangzhou").unwrap();

        // Signing key should be 32 bytes (256 bits for SHA256)
        assert_eq!(signing_key.len(), 32);

        // Should be deterministic
        let signing_key2 = derive_signing_key("test-secret", "20250411", "cn-hangzhou").unwrap();
        assert_eq!(signing_key, signing_key2);

        // Different inputs should produce different keys
        let signing_key3 = derive_signing_key(
            "test-secret",
            "20250412", // Different date
            "cn-hangzhou",
        )
        .unwrap();
        assert_ne!(signing_key, signing_key3);

        let signing_key4 = derive_signing_key(
            "test-secret",
            "20250411",
            "cn-shanghai", // Different region
        )
        .unwrap();
        assert_ne!(signing_key, signing_key4);
    }

    #[test]
    fn test_oss4_full_signature_example() {
        // Test the full signature computation based on the documentation example
        // This validates the entire signing process produces the correct format
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("LTAI**************", "yourAccessKeySecret"),
            "cn-hangzhou",
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "content-disposition",
            HeaderValue::from_static("attachment"),
        );
        headers.insert("content-length", HeaderValue::from_static("3"));
        headers.insert(
            "content-md5",
            HeaderValue::from_static("ICy5YqxZB1uWSwcVLSNLcA=="),
        );
        headers.insert("content-type", HeaderValue::from_static("text/plain"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));

        let result = auth
            .sign(
                &mut headers,
                "/examplebucket/exampleobject",
                "",
                "PUT",
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        // Verify the Authorization header format
        assert!(result.starts_with("OSS4-HMAC-SHA256 Credential=LTAI**************/20250411/cn-hangzhou/oss/aliyun_v4_request"));

        // Verify AdditionalHeaders are included
        assert!(result.contains("AdditionalHeaders="));

        // Verify Signature is present
        assert!(result.contains("Signature="));

        // Verify signature is 64 hex characters (256 bits = 32 bytes = 64 hex chars)
        if let Some(sig_pos) = result.find("Signature=") {
            let sig_start = sig_pos + "Signature=".len();
            let sig = &result[sig_start..];
            assert_eq!(sig.len(), 64, "Signature should be 64 hex characters");
            assert!(
                sig.chars().all(|c| c.is_ascii_hexdigit()),
                "Signature should be hex digits"
            );
        }
    }

    #[test]
    fn test_oss4_canonical_request_hash() {
        // Test that the canonical request hash matches the expected value from the docs
        // Note: The canonical request format from the documentation
        let canonical_request = "PUT
/examplebucket/exampleobject

content-disposition:attachment
content-length:3
content-md5:ICy5YqxZB1uWSwcVLSNLcA==
content-type:text/plain
x-oss-content-sha256:UNSIGNED-PAYLOAD
x-oss-date:20250411T064124Z

content-disposition;content-length
UNSIGNED-PAYLOAD";

        let hash = hexed_sha256(canonical_request);
        let expected_hash = "c46d96390bdbc2d739ac9363293ae9d710b14e48081fcb22cd8ad54b63136eca";
        assert_eq!(hash, expected_hash);
    }
}
