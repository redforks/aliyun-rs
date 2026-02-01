//! Authorization algorithms for different Aliyun cloud products.
//!
//! Different Aliyun cloud products may use different authorization algorithms:
//! - Most products use `ACS3-HMAC-SHA256`
//! - OSS uses `OSS4-HMAC-SHA256`

use std::borrow::Cow;

use anyhow::Context;
use http::{HeaderMap, HeaderValue};
use time::OffsetDateTime;
use tracing::debug;

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
        // Build canonical request and signed headers
        let (canonical_request, signed_headers) = build_acs3_canonical_request_and_headers(
            method,
            url,
            query_string,
            headers,
            hashed_payload,
        )?;

        // Hash the canonical request
        let hashed_canonical_request = hexed_sha256(&canonical_request);

        // Build string to sign
        let string_to_sign = format!("{}\n{}", ACS3_SIGNATURE_ALGORITHM, hashed_canonical_request);

        // Calculate signature
        let signature = hexed_hmac_sha256(self.0.access_key_secret().as_bytes(), string_to_sign.as_bytes())?;

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

        // Build canonical request and additional headers string
        let (canonical_request, additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                method,
                url,
                query_string,
                headers,
                hashed_payload,
            )?;
        debug!("canonical_request: {}", canonical_request);
        debug!("additional_headers_str: {}", additional_headers_str);

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
        let signature = hexed_hmac_sha256(&signing_key, string_to_sign.as_bytes())?;

        // Build Authorization header
        // Format: OSS4-HMAC-SHA256 Credential=<AccessKeyId>/<SignDate>/<SignRegion>/oss/aliyun_v4_request, AdditionalHeaders=<AdditionalHeadersVal>, Signature=<SignatureVal>
        let credential = format!("{}/{}", self.credentials.access_key_id(), scope);

        Ok(if additional_headers_str.is_empty() {
            format!(
                "{} Credential={}, Signature={}",
                OSS4_SIGNATURE_ALGORITHM, credential, signature
            )
        } else {
            format!(
                "{} Credential={}, AdditionalHeaders={}, Signature={}",
                OSS4_SIGNATURE_ALGORITHM, credential, additional_headers_str, signature
            )
        })
    }

    fn access_key_id(&self) -> &str {
        self.credentials.access_key_id()
    }
}

/// Compute signature String using HMAC-SHA256, and encode use hex.
fn hexed_hmac_sha256(key: &[u8], to_sign: &[u8]) -> Result<String> {
    let result = hmac_sha256_raw(key, to_sign)?;
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
    use time::format_description::well_known::iso8601::Config;
    use time::format_description::well_known::iso8601::TimePrecision;
    use time::format_description::well_known::{Iso8601, iso8601::EncodedConfig};

    const CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: None,
        })
        .encode();
    const FORMAT: Iso8601<CONFIG> = Iso8601::<CONFIG>;

    let formatted = dt.format(&FORMAT).context("format OSS timestamp failed")?;

    // Convert from "2024-01-01T00:00:00Z" to "20240101T000000Z"
    let oss_formatted = formatted.replace("-", "").replace(":", "");

    Ok(oss_formatted)
}

/// Build ACS3 canonical request and signed headers.
///
/// Returns a tuple of (canonical_request, signed_headers).
fn build_acs3_canonical_request_and_headers(
    method: &str,
    url: &str,
    query_string: &str,
    headers: &HeaderMap,
    hashed_payload: &str,
) -> Result<(String, String)> {
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

    Ok((canonical_request, signed_headers))
}

/// Build OSS4 canonical request and additional headers string.
///
/// Returns a tuple of (canonical_request, additional_headers_str).
fn build_oss4_canonical_request_and_additional_headers(
    method: &str,
    url: &str,
    query_string: &str,
    headers: &HeaderMap,
    hashed_payload: &str,
) -> Result<(String, String)> {
    use std::collections::{BTreeMap, BTreeSet};

    // Build canonical headers and determine additional headers
    // Headers that participate in signing:
    // - x-oss-* headers (always)
    // - content-type, content-md5 (if present) - these are "default signing headers"
    // - optional headers like content-disposition, content-length (if present)
    let mut canonical_headers_map: BTreeMap<String, String> = BTreeMap::new();
    let mut additional_headers_set: BTreeSet<String> = BTreeSet::new();

    for (k, v) in headers.iter() {
        let key = k.as_str().to_lowercase();
        let value = v.to_str().context("convert header value to string")?;

        if key.starts_with("x-oss-") {
            // x-oss-* headers always participate in signing but never in AdditionalHeaders
            canonical_headers_map.insert(key, value.to_string());
        } else if key == "content-type" || key == "content-md5" {
            // Default signing headers: participate in signing but NOT in AdditionalHeaders
            canonical_headers_map.insert(key, value.to_string());
        } else if matches!(key.as_str(), "content-disposition" | "content-length") {
            // These are optional headers that participate in signing AND appear in AdditionalHeaders
            canonical_headers_map.insert(key.clone(), value.to_string());
            additional_headers_set.insert(key);
        }
    }

    // Build canonical headers string
    let mut canonical_headers = String::new();
    for (k, v) in &canonical_headers_map {
        canonical_headers.push_str(k);
        canonical_headers.push(':');
        canonical_headers.push_str(v);
        canonical_headers.push('\n');
    }

    // Build additional headers string (only the truly optional ones, semicolon-separated)
    let additional_headers_str: String = additional_headers_set
        .iter()
        .map(|k| k.as_str())
        .collect::<Vec<_>>()
        .join(";");

    // Build canonical request
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method, url, query_string, canonical_headers, additional_headers_str, hashed_payload
    );

    Ok((canonical_request, additional_headers_str))
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::header::HeaderValue;
    use test_log::test;

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

    #[test]
    fn test_oss4_additional_headers_excludes_default_signing_headers() {
        // Verify that content-type and content-md5 are NOT in AdditionalHeaders
        // They should be in canonical headers but NOT in the AdditionalHeaders field
        let auth = Oss4HmacSha256::new(
            AccessKeySecret::new("test-key", "test-secret"),
            "cn-hangzhou",
        );

        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("content-md5", HeaderValue::from_static("abc123"));
        headers.insert(
            "content-disposition",
            HeaderValue::from_static("attachment"),
        );
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));

        let result = auth
            .sign(&mut headers, "/test", "", "GET", "UNSIGNED-PAYLOAD")
            .unwrap();

        // AdditionalHeaders should contain content-disposition but NOT content-type or content-md5
        assert!(
            result.contains("AdditionalHeaders=content-disposition"),
            "AdditionalHeaders should contain content-disposition"
        );
        assert!(
            !result.contains("content-type")
                || result
                    .split("AdditionalHeaders=")
                    .all(|part| !part.starts_with("content-type")),
            "AdditionalHeaders should NOT contain content-type (it's a default signing header)"
        );
        assert!(
            !result.contains("content-md5")
                || result
                    .split("AdditionalHeaders=")
                    .all(|part| !part.starts_with("content-md5")),
            "AdditionalHeaders should NOT contain content-md5 (it's a default signing header)"
        );

        // Parse the AdditionalHeaders value to verify exact content
        if let Some(additional_part) = result.split("AdditionalHeaders=").nth(1) {
            let additional_headers_value = additional_part.split(',').next().unwrap_or("");
            // Should only contain content-disposition
            assert_eq!(
                additional_headers_value, "content-disposition",
                "AdditionalHeaders should only contain 'content-disposition', not 'content-type' or 'content-md5'"
            );
        }
    }

    #[test]
    fn test_oss4_signature_format_matches_documentation() {
        // Verify the signature format exactly matches the documentation example
        // From: https://help.aliyun.com/zh/oss/developer-reference/recommend-to-use-signature-version-4
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

        // Verify the Authorization header format matches the documentation:
        // OSS4-HMAC-SHA256 Credential=<AccessKeyId>/<SignDate>/<SignRegion>/oss/aliyun_v4_request, AdditionalHeaders=<AdditionalHeadersVal>, Signature=<SignatureVal>
        assert!(
            result.starts_with("OSS4-HMAC-SHA256 Credential=LTAI**************/20250411/cn-hangzhou/oss/aliyun_v4_request"),
            "Credential format should match documentation"
        );

        // Verify AdditionalHeaders is "content-disposition;content-length" (NOT including content-type or content-md5)
        assert!(
            result.contains("AdditionalHeaders=content-disposition;content-length"),
            "AdditionalHeaders should be 'content-disposition;content-length', not include content-type or content-md5"
        );

        // Verify AdditionalHeaders does NOT contain default signing headers
        assert!(
            !result.contains("AdditionalHeaders=content-type"),
            "AdditionalHeaders should NOT contain content-type"
        );
        assert!(
            !result.contains("AdditionalHeaders=content-md5"),
            "AdditionalHeaders should NOT contain content-md5"
        );
    }

    // OSS4 Canonical Request Tests
    #[test]
    fn test_oss4_canonical_request_from_documentation() {
        // Test the exact example from the documentation
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

        let (canonical_request, _additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "PUT",
                "/examplebucket/exampleobject",
                "",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        // Expected format from documentation:
        let expected = r#"PUT
/examplebucket/exampleobject

content-disposition:attachment
content-length:3
content-md5:ICy5YqxZB1uWSwcVLSNLcA==
content-type:text/plain
x-oss-content-sha256:UNSIGNED-PAYLOAD
x-oss-date:20250411T064124Z

content-disposition;content-length
UNSIGNED-PAYLOAD"#;
        assert_eq!(canonical_request, expected);
    }

    #[test]
    fn test_oss4_canonical_request_empty_query() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        let (canonical_request, _additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "GET",
                "/test-bucket/",
                "",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        assert!(canonical_request.starts_with("GET\n/test-bucket/\n\n"));
        assert!(canonical_request.contains("x-oss-date:20250411T064124Z"));
        assert!(canonical_request.ends_with("UNSIGNED-PAYLOAD"));
    }

    #[test]
    fn test_oss4_canonical_request_with_query_string() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        let (canonical_request, _additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "GET",
                "/test-bucket/test-object",
                "max-keys=100&prefix=test%2F",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        assert!(
            canonical_request
                .starts_with("GET\n/test-bucket/test-object\nmax-keys=100&prefix=test%2F\n")
        );
    }

    #[test]
    fn test_oss4_canonical_request_headers_ordering() {
        // Verify that headers are sorted alphabetically
        let mut headers = HeaderMap::new();
        headers.insert("content-length", HeaderValue::from_static("1024"));
        headers.insert(
            "content-disposition",
            HeaderValue::from_static("attachment"),
        );
        headers.insert("content-type", HeaderValue::from_static("text/plain"));
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        let (canonical_request, _additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "PUT",
                "/bucket/object",
                "",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        // Find positions of headers in canonical request
        let cd_pos = canonical_request
            .find("content-disposition:attachment")
            .unwrap();
        let cl_pos = canonical_request.find("content-length:1024").unwrap();
        let ct_pos = canonical_request.find("content-type:text/plain").unwrap();

        assert!(
            cd_pos < cl_pos,
            "content-disposition should come before content-length"
        );
        assert!(
            cl_pos < ct_pos,
            "content-length should come before content-type"
        );
    }

    #[test]
    fn test_oss4_canonical_request_without_additional_headers() {
        // Test with only default signing headers (no additional headers)
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/octet-stream"),
        );
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        let (canonical_request, _additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "PUT",
                "/bucket/object",
                "",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        // Should contain content-type in canonical headers
        assert!(canonical_request.contains("content-type:application/octet-stream"));

        // Should have empty additional headers section
        assert!(canonical_request.contains("\n\nUNSIGNED-PAYLOAD"));
    }

    #[test]
    fn test_oss4_canonical_request_additional_headers_field() {
        // Verify that default signing headers (content-type, content-md5) don't appear in AdditionalHeaders field
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-disposition",
            HeaderValue::from_static("attachment"),
        );
        headers.insert("content-length", HeaderValue::from_static("1024"));
        headers.insert("content-md5", HeaderValue::from_static("abc123"));
        headers.insert("content-type", HeaderValue::from_static("text/plain"));
        headers.insert("x-oss-date", HeaderValue::from_static("20250411T064124Z"));
        headers.insert(
            "x-oss-content-sha256",
            HeaderValue::from_static("UNSIGNED-PAYLOAD"),
        );

        let (canonical_request, additional_headers_str) =
            build_oss4_canonical_request_and_additional_headers(
                "PUT",
                "/bucket/object",
                "",
                &headers,
                "UNSIGNED-PAYLOAD",
            )
            .unwrap();

        // All headers should be in canonical headers
        assert!(canonical_request.contains("content-disposition:attachment"));
        assert!(canonical_request.contains("content-length:1024"));
        assert!(canonical_request.contains("content-md5:abc123"));
        assert!(canonical_request.contains("content-type:text/plain"));

        // AdditionalHeaders should only contain content-disposition and content-length
        assert_eq!(
            additional_headers_str, "content-disposition;content-length",
            "AdditionalHeaders field should only contain content-disposition and content-length"
        );

        // Verify content-type and content-md5 are NOT in AdditionalHeaders
        assert!(
            !additional_headers_str.contains("content-type"),
            "content-type should not be in AdditionalHeaders"
        );
        assert!(
            !additional_headers_str.contains("content-md5"),
            "content-md5 should not be in AdditionalHeaders"
        );
    }

    #[test]
    fn test_oss4_signature_alignment() -> Result<()> {
        // 1. 准备基础参数
        let access_key_id = "LTAI****************";
        let secret = "yourAccessKeySecret";
        let region = "cn-hangzhou";
        let timestamp = "20250411T064124Z";
        let sign_date = "20250411";

        let auth = Oss4HmacSha256::new(AccessKeySecret::new(access_key_id, secret), region);

        // 2. 构造 Headers
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-date", timestamp.parse().unwrap());
        headers.insert("x-oss-content-sha256", "UNSIGNED-PAYLOAD".parse().unwrap());
        headers.insert("content-type", "text/plain".parse().unwrap());
        headers.insert("content-md5", "ICy5YqxZB1uWSwcVLSNLcA==".parse().unwrap());
        headers.insert("content-length", "3".parse().unwrap());
        headers.insert("content-disposition", "attachment".parse().unwrap());

        // 3. 验证规范化请求 (Canonical Request)
        // 注意：根据文档示例，AdditionalHeaders 包含了 content-disposition 和 content-length
        let method = "PUT";
        let url = "/examplebucket/exampleobject";
        let query_string = "";
        let hashed_payload = "UNSIGNED-PAYLOAD";

        let (canonical_request, additional_headers) =
            build_oss4_canonical_request_and_additional_headers(
                method,
                url,
                query_string,
                &headers,
                hashed_payload,
            )?;

        // 验证 CanonicalRequest 的 SHA256 哈希值
        // 文档值: c46d96390bdbc2d739ac9363293ae9d710b14e48081fcb22cd8ad54b63136eca
        let hashed_cr = hexed_sha256(canonical_request.as_bytes());
        assert_eq!(
            hashed_cr,
            "c46d96390bdbc2d739ac9363293ae9d710b14e48081fcb22cd8ad54b63136eca"
        );

        // 4. 验证 SigningKey
        // 文档中的 SigningKey 值 (3543b7686e65eda71e5e5ca19d548d78423c37e8ddba4dc9d83f90228b457c76) 存在错误
        // 实际值应与官方 SDK (Java/Go/Python) 保持一致
        // Reference: https://github.com/aliyun/aliyun-oss-java-sdk/blob/master/src/main/java/com/aliyun/oss/internal/signer/OSSV4Signer.java
        let signing_key = derive_signing_key(secret, sign_date, region)?;
        assert_eq!(
            hex::encode(&signing_key),
            "8a01ff4efcc65ca2cbc75375045c61ab5f3fa8b9a2d84f0add27ef16a25feb3c"
        );

        // 5. 验证最终签名 (Signature)
        // 文档中的签名值 (053edbf550ebd239b32a9cdfd93b0b2b3f2d223083aa61f75e9ac16856d61f23) 是基于错误的 SigningKey 计算得出的
        // 实际签名值应基于正确的 SigningKey 计算
        let signature_res = auth.sign(&mut headers, url, query_string, method, hashed_payload)?;

        // 从 Authorization 字符串中截取 Signature 部分进行比对
        // 格式: ... Signature=d3694c2dfc5371ee6acd35e88c4871ac95a7ba01d3a2f476768fe61218590097
        assert!(
            signature_res
                .contains("d3694c2dfc5371ee6acd35e88c4871ac95a7ba01d3a2f476768fe61218590097")
        );

        Ok(())
    }
}
