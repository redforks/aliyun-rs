//! ACS3-HMAC-SHA256 authorization algorithm.
//!
//! This is the standard authorization algorithm used by most Aliyun cloud products
//! like ECS, SMS, FC, OCR, etc.

use super::{AccessKeySecret, AliyunAuth, hexed_hmac_sha256, hexed_sha256, percent_encode};
use crate::{QueryValue, Result};
use anyhow::Context as _;
use http::{HeaderMap, HeaderValue};
use std::borrow::Cow;
use time::OffsetDateTime;
use time::format_description::well_known::{
    Iso8601,
    iso8601::{Config, EncodedConfig, TimePrecision},
};

use reqwest::Body;

/// ACS3-HMAC-SHA256 authorization algorithm.
///
/// This is the standard authorization algorithm used by most Aliyun cloud products
/// like ECS, SMS, FC, OCR, etc.
#[derive(Clone, Debug)]
pub struct Acs3HmacSha256(pub AccessKeySecret);

impl Acs3HmacSha256 {
    const ACS3_SIGNATURE_ALGORITHM: &str = "ACS3-HMAC-SHA256";

    #[cfg(test)]
    pub fn new(key: impl Into<Cow<'static, str>>, secret: impl Into<Cow<'static, str>>) -> Self {
        Self(AccessKeySecret::new(key, secret))
    }
}

/// Format datetime to format like: 2018-01-01T12:00:00Z
fn format_acs3_datetime(dt: OffsetDateTime) -> Result<String> {
    const CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: None,
        })
        .encode();
    const FORMAT: Iso8601<CONFIG> = Iso8601::<CONFIG>;

    dt.format(&FORMAT)
        .context("format rfc3339 failed")
        .map_err(Into::into)
}

impl AliyunAuth for Acs3HmacSha256 {
    fn create_headers(&self, action: &str, version: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-acs-action",
            HeaderValue::try_from(action).context("convert to header value")?,
        );
        headers.insert(
            "x-acs-version",
            HeaderValue::try_from(version).context("convert to header value")?,
        );

        headers.insert(
            "x-acs-signature-nonce",
            HeaderValue::try_from(uuid::Uuid::new_v4().to_string())
                .context("convert to header value")?,
        );

        headers.insert(
            "x-acs-date",
            HeaderValue::try_from(format_acs3_datetime(OffsetDateTime::now_utc())?)
                .context("convert to header value")?,
        );

        Ok(headers)
    }

    fn canonical_query_string(&self, values: Vec<(Cow<'static, str>, QueryValue)>) -> String {
        let mut values = values;
        // Sort by parameter name (before encoding)
        values.sort_by(|a, b| a.0.cmp(&b.0));

        values
            .into_iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    percent_encode(&k),
                    percent_encode(&v.to_query_value())
                )
            })
            .collect::<Vec<_>>()
            .join("&")
    }

    fn sign(
        &self,
        headers: &mut HeaderMap,
        path: &str,
        query_string: &str,
        method: &str,
        body: &Body,
    ) -> Result<String> {
        // Compute hashed payload from body
        let body_bytes = body.as_bytes().context("body should be bytes")?;
        let hashed_payload = hexed_sha256(body_bytes);

        // Set x-acs-content-sha256 header
        headers.insert(
            "x-acs-content-sha256",
            HeaderValue::try_from(hashed_payload.as_str()).context("convert to header value")?,
        );

        // Build canonical headers and signed headers
        let mut headers_map = headers
            .iter()
            .filter_map(|(k, v)| {
                let k = k.as_str().to_lowercase();
                (k.starts_with("x-acs-") || k == "host" || k == "content-type").then(|| {
                    let v = v.to_str().context("convert header value to string")?;
                    Ok((k, v))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        headers_map.sort_by(|a, b| a.0.cmp(&b.0));
        let mut canonical_headers = String::new();
        let mut signed_headers = String::new();
        for (k, v) in headers_map {
            canonical_headers.push_str(&k);
            canonical_headers.push(':');
            canonical_headers.push_str(&v);
            canonical_headers.push('\n');
            signed_headers.push_str(&k);
            signed_headers.push(';');
        }
        // Remove last ';' from signed_headers
        signed_headers.pop();

        // Build canonical request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method, path, query_string, canonical_headers, signed_headers, hashed_payload
        );

        // Build string to sign
        let string_to_sign = format!(
            "{}\n{}",
            Self::ACS3_SIGNATURE_ALGORITHM,
            hexed_sha256(&canonical_request)
        );

        // Calculate signature
        let signature = hexed_hmac_sha256(
            self.0.access_key_secret().as_bytes(),
            string_to_sign.as_bytes(),
        )?;

        // Build Authorization header
        Ok(format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            Self::ACS3_SIGNATURE_ALGORITHM,
            self.0.access_key_id(),
            signed_headers,
            signature
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn create_test_headers_acs3(auth: &Acs3HmacSha256) -> HeaderMap {
        auth.create_headers("DescribeInstances", "2014-05-26")
            .unwrap()
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_empty_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let mut headers = create_test_headers_acs3(&auth);
        let query_string = "";
        let body: Body = b"".as_slice().into();

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", &body)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_acs3_hmac_sha256_sign_with_query() {
        let auth = Acs3HmacSha256::new("test-access-key-id", "test-access-key-secret");
        let mut headers = create_test_headers_acs3(&auth);
        let query_string = "PageSize=10&RegionId=cn-hangzhou";
        let body: Body = b"".as_slice().into();

        let result = auth
            .sign(&mut headers, "/", query_string, "GET", &body)
            .unwrap();

        assert!(result.starts_with("ACS3-HMAC-SHA256 Credential=test-access-key-id,"));
        assert!(result.contains("SignedHeaders="));
        assert!(result.contains("Signature="));
    }

    #[test]
    fn test_format_acs3_datetime() -> Result<()> {
        let dt = OffsetDateTime::from_unix_timestamp(0).context("create from unix time stamp 0")?;
        assert_eq!(format_acs3_datetime(dt)?, "1970-01-01T00:00:00Z");

        let dt = time::macros::datetime!(2018-01-01 12:00:00.123 UTC);
        assert_eq!(format_acs3_datetime(dt)?, "2018-01-01T12:00:00Z");

        Ok(())
    }
}
