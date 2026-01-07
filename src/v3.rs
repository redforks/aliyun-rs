//! Build authorization signature V3

use anyhow::{Context, anyhow};
use http::{
    HeaderMap, HeaderValue, Method,
    header::{CONTENT_TYPE, IntoHeaderName},
};
use std::{borrow::Cow, collections::BTreeMap};
use time::{OffsetDateTime, format_description::well_known::iso8601::TimePrecision};
use tracing::debug;

use crate::{FromBody, IntoBody as _, IntoResponse, QueryValue, Result, ToCodeMessage};

/// Separate the request into several parts by '/', each part encode with percent_encode,
/// and join them with '/'.
fn canonical_uri_path(uri: &str) -> String {
    uri.split('/')
        .map(percent_encode)
        .collect::<Vec<_>>()
        .join("/")
}

fn canonical_query_string(values: Vec<(Cow<'static, str>, QueryValue)>) -> String {
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
    for (k, v) in &headers {
        canonical_headers.push_str(k);
        canonical_headers.push(':');
        canonical_headers.push_str(v);
        canonical_headers.push('\n');
        signed_headers.push_str(k);
        signed_headers.push(';');
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

/// Build http request according to authorization signature V3.
pub async fn call<R>(
    key_secret: &AccessKeySecret,
    http_client: &reqwest::Client,
    version: &'static str,
    end_point: &'static str,
    req: R,
) -> Result<<R::ResponseWrap as IntoResponse>::Response>
where
    R: super::Request,
{
    // Replace path parameters if any
    let path_args = req.get_path_args();
    let url_path: Cow<'static, str> = if path_args.is_empty() {
        R::URL_PATH.into()
    } else {
        let mut path = R::URL_PATH.to_string();
        for (placeholder, value) in path_args.iter() {
            path = path.replace(placeholder, value);
        }
        path.into()
    };
    let uri = canonical_uri_path(&url_path);
    let query_string = canonical_query_string(req.to_query_params());
    let custom_headers = req.to_headers();
    let body = req.to_body();
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
    // Add custom headers from the request
    for (name, value) in custom_headers {
        let header_name = http::header::HeaderName::from_bytes(name.as_bytes())
            .context("Invalid custom header name")?;
        let header_value =
            http::header::HeaderValue::from_str(&value).context("Invalid custom header value")?;
        headers.insert(header_name, header_value);
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
    let mut url = format!("https://{}{}", end_point, url_path);
    if !query_string.is_empty() {
        url.push('?');
        url.push_str(&query_string);
    }
    debug!("{:#?}", &headers);
    let resp = match R::METHOD {
        Method::POST => http_client.post(url).headers(headers).body(body),
        Method::PUT => http_client.put(url).headers(headers).body(body),
        Method::GET => http_client.get(url).headers(headers),
        _ => unreachable!(),
    }
    .send()
    .await
    .context("send request")?;

    let status = resp.status();
    let resp_bytes = resp.bytes().await.context("Get response bytes")?;
    debug!("Response: {:?}", String::from_utf8_lossy(&resp_bytes));

    let resp = if status.is_success() {
        // Use ResponseWrap to deserialize response bytes
        let wrap = R::ResponseWrap::from_body(resp_bytes.to_vec())?;
        wrap.to_code_message().check()?;
        // Convert ResponseWrap to Response type
        wrap.into_response()
    } else {
        // Try JSON first for error responses, then XML
        let resp_text = String::from_utf8_lossy(&resp_bytes);
        match serde_json::from_str::<crate::CodeMessage>(&resp_text) {
            Ok(code_msg) => return Err(code_msg.into()),
            Err(_) => {
                // Try XML for error response
                match quick_xml::de::from_str::<crate::CodeMessage>(&resp_text) {
                    Ok(code_msg) => return Err(code_msg.into()),
                    Err(_) => return Err(anyhow!("HTTP error: {} - {}", status, resp_text).into()),
                }
            }
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

/// Percent-encodes a string per RFC 3986.
///
/// The `urlencoding` crate's default behavior matches Aliyun's requirements:
/// - Spaces are encoded as `%20` (not `+`)
/// - The `*` character is encoded as `%2A`
/// - The `~` character is not encoded
fn percent_encode(s: &str) -> Cow<'_, str> {
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
