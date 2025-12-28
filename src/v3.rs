//! Build authorization signature V3

use anyhow::{Context, anyhow};
use http::{
    HeaderMap, HeaderValue, Method,
    header::{CONTENT_TYPE, IntoHeaderName},
};
use std::{borrow::Cow, collections::BTreeMap};
use time::{OffsetDateTime, format_description::well_known::iso8601::TimePrecision};
use tracing::debug;

use crate::{IntoBody as _, QueryValue, Result};

/// Separate the request into several parts by '/', each part encode with percent_encode,
/// and join them with '/'.
fn canonical_uri_path(uri: &str) -> Cow<'_, str> {
    let mut uri = uri.split('/');
    let mut canonical_uri = String::new();
    if let Some(part) = uri.next() {
        canonical_uri.push_str(&percent_encode(part));
    }
    for part in uri {
        canonical_uri.push('/');
        canonical_uri.push_str(&percent_encode(part));
    }
    canonical_uri.into()
}

fn canonical_query_string(values: BTreeMap<Cow<'static, str>, QueryValue>) -> String {
    let mut query = String::new();
    for (k, v) in values {
        let v = v.to_query_value();
        query.push_str(&format!("{}={}&", percent_encode(&k), percent_encode(&v)));
    }
    query.pop(); // remove last '&'
    query
}

fn insert_str_header(headers: &mut HeaderMap, key: impl IntoHeaderName, value: &str) -> Result<()> {
    headers.insert(
        key,
        HeaderValue::from_str(value).context("convert to header value")?,
    );
    Ok(())
}

fn gen_headers<R: super::Request>(
    version: &'static str,
    end_point: &'static str,
    hashed_content: &str,
) -> Result<HeaderMap> {
    let mut r = HeaderMap::new();
    insert_str_header(&mut r, "x-acs-action", R::ACTION)?;
    insert_str_header(&mut r, "x-acs-version", version)?;
    insert_str_header(
        &mut r,
        "x-acs-signature-nonce",
        &uuid::Uuid::new_v4().to_string(),
    )?;
    insert_str_header(
        &mut r,
        "x-acs-date",
        &format_datetime(OffsetDateTime::now_utc())?,
    )?;
    insert_str_header(&mut r, "host", end_point)?;
    insert_str_header(&mut r, "x-acs-content-sha256", hashed_content)?;

    Ok(r)
}

fn canonical_headers(headers: &HeaderMap) -> Result<(String, String)> {
    let headers = headers
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
    for (k, v) in headers {
        canonical_headers.push_str(&format!("{}:{}\n", k, v));
        signed_headers.push_str(&format!("{};", k));
    }
    // remove last ';' of signed_headers, canonical_headers should not remove last '\n'
    signed_headers.pop();

    Ok((canonical_headers, signed_headers))
}

const SIGNATURE_ALGORITHM: &str = "ACS3-HMAC-SHA256";

/// Type of ali access key and secret key.
#[derive(Clone, Debug)]
pub struct AccessKeySecret(pub Cow<'static, str>, pub Cow<'static, str>);

impl AccessKeySecret {
    #[inline]
    pub fn new(key: impl Into<Cow<'static, str>>, secret: impl Into<Cow<'static, str>>) -> Self {
        Self(key.into(), secret.into())
    }
}

/// Build http request according to authorization signature V3.
pub async fn call<R>(
    key_secret: &AccessKeySecret,
    http_client: &reqwest::Client,
    version: &'static str,
    end_point: &'static str,
    req: R,
) -> Result<R::Response>
where
    R: super::Request,
{
    let uri = canonical_uri_path(R::URL_PATH);
    let query_string = canonical_query_string(req.to_query_params()?);
    let body = req.to_body()?;
    let content_type = if R::METHOD == Method::GET {
        None
    } else {
        Some(body.content_type())
    };
    let body = body.into_body()?;
    let body_bytes = body.as_bytes().context("body should be bytes")?;
    let hashed_request_payload = hexed_sha256(body_bytes);
    let mut headers = gen_headers::<R>(version, end_point, &hashed_request_payload)?;
    if let Some(content_type) = content_type {
        headers.insert(CONTENT_TYPE, content_type);
    }
    let (canonical_headers, header_names) = canonical_headers(&headers)?;
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        R::METHOD.as_str(),
        uri,
        query_string,
        canonical_headers,
        header_names,
        hashed_request_payload
    );
    debug!("canonical_request: {:?}", canonical_request);
    let hashed_canonical_request = hexed_sha256(&canonical_request);
    let string_to_sign = format!("{}\n{}", SIGNATURE_ALGORITHM, hashed_canonical_request);
    debug!("string_to_sign: {:?}", string_to_sign);
    let signature = hexed_hmac_sha256(&key_secret.1, string_to_sign.as_bytes())?;
    let ali_access_key_id = &key_secret.0;
    insert_str_header(
        &mut headers,
        http::header::AUTHORIZATION,
        &format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            SIGNATURE_ALGORITHM, ali_access_key_id, header_names, signature
        ),
    )?;
    let mut url = format!("https://{}{}", end_point, R::URL_PATH);
    if !query_string.is_empty() {
        url.push('?');
        url.push_str(&query_string);
    }
    debug!("{:#?}", &headers);
    let resp = match R::METHOD {
        Method::POST => http_client.post(url).headers(headers).body(body),
        Method::GET => http_client.get(url).headers(headers),
        _ => unreachable!(),
    }
    .send()
    .await
    .context("send request")?;

    let status = resp.status();
    let resp_text = resp.text().await.context("Get response text")?;

    let resp = if status.is_success() {
        serde_json::from_str::<R::Response>(&resp_text).context("Decode response as JSON")?
    } else {
        match serde_json::from_str::<crate::CodeMessage>(&resp_text) {
            Ok(code_msg) => return Err(code_msg.into()),
            Err(_) => return Err(anyhow!("HTTP error: {} - {}", status, resp_text).into()),
        }
    };
    Ok(resp)
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

/// Url encoding but convert space to '%20' not '+', '*' to '%2A', and keep '~' as is.
fn percent_encode(s: &str) -> Cow<'_, str> {
    // looks like the default behavior of urlencoding crate is what we want, so just use it.
    // The problem maybe exist in java implementation, so ali doc explicitly mention this.
    urlencoding::encode(s)
}

/// 1. sha256 hash the request s
/// 1. hex encode the hash
fn hexed_sha256(s: impl AsRef<[u8]>) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(s);
    hex::encode(hasher.finalize())
}

/// Format datetime to format like: 2018-01-01T12:00:00Z
fn format_datetime(dt: OffsetDateTime) -> Result<String> {
    use time::format_description::well_known::{
        Iso8601,
        iso8601::{Config, EncodedConfig},
    };

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

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_format_datetime() -> Result<()> {
        let dt = OffsetDateTime::from_unix_timestamp(0).context("create from unix time stamp 0")?;
        assert_eq!(format_datetime(dt)?, "1970-01-01T00:00:00Z");

        let dt = datetime!(2018-01-01 12:00:00.123 UTC);
        assert_eq!(format_datetime(dt)?, "2018-01-01T12:00:00Z");

        Ok(())
    }

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode(" "), "%20");
        assert_eq!(percent_encode("*"), "%2A");
        assert_eq!(percent_encode("~"), "~");
    }

    #[test]
    fn test_canonical_uri_path() {
        assert_eq!(canonical_uri_path("/"), "/");
        assert_eq!(canonical_uri_path("a/b/c/"), "a/b/c/");
        assert_eq!(canonical_uri_path("a/b/c//d"), "a/b/c//d");
    }
}
