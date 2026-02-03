//! Build authorization signature V3

use anyhow::{Context, anyhow};
use http::{HeaderValue, Method, header::CONTENT_TYPE};
use std::borrow::Cow;
use tracing::debug;

use crate::{FromBody, IntoBody as _, IntoResponse, Result, ToCodeMessage, auth::AliyunAuth};

// Re-export AccessKeySecret for backward compatibility
pub use crate::auth::AccessKeySecret;

/// Build http request according to authorization signature V3.
pub async fn call<R>(
    auth: &dyn AliyunAuth,
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
    let query_string = auth.canonical_query_string(req.to_query_params());
    let custom_headers = req.to_headers();
    let body = req.to_body();

    let content_type = body.content_type();
    let body = body.into_body()?;

    let mut headers = auth.create_headers(R::ACTION, version)?;
    if let Some(content_type) = content_type {
        headers.insert(CONTENT_TYPE, content_type);
    }
    for (name, value) in custom_headers {
        let header_name = http::header::HeaderName::from_bytes(name.as_bytes())
            .context("Invalid custom header name")?;
        let header_value =
            http::header::HeaderValue::from_str(&value).context("Invalid custom header value")?;
        headers.insert(header_name, header_value);
    }

    // Use the auth trait to sign the request
    let authorization = auth.sign(
        &mut headers,
        &url_path,
        &query_string,
        R::METHOD.as_str(),
        &body,
    )?;

    headers.insert(
        http::header::AUTHORIZATION,
        HeaderValue::try_from(authorization).context("convert to header value")?,
    );
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
    let resp_headers = resp.headers().clone();
    let resp_bytes = resp.bytes().await.context("Get response bytes")?;
    debug!("Response: {:?}", String::from_utf8_lossy(&resp_bytes));

    let resp = if status.is_success() {
        // Use ResponseWrap to deserialize response bytes
        let mut wrap = R::ResponseWrap::from_body(resp_bytes.to_vec())?;
        R::from_headers(&mut wrap, &resp_headers);
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
