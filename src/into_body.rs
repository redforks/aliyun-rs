use crate::Result;
use anyhow::Context as _;
use http::HeaderValue;
use reqwest::Body;
use tracing::debug;

pub trait IntoBody {
    fn content_type(&self) -> Option<HeaderValue>;
    fn into_body(self) -> Result<Body>;
}

impl IntoBody for () {
    fn content_type(&self) -> Option<HeaderValue> {
        None
    }

    fn into_body(self) -> Result<Body> {
        Ok(b"".as_slice().into())
    }
}

pub(crate) struct Form<T: crate::ToFormData>(pub T);

impl<T: crate::ToFormData> IntoBody for Form<T> {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static(
            "application/x-www-form-urlencoded",
        ))
    }

    fn into_body(self) -> Result<Body> {
        let params = self.0.to_form_data();
        let encoded = params
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), v.url_encode()))
            .collect::<Vec<_>>()
            .join("&");
        Ok(encoded.into())
    }
}

/// Body wrapper for binary data using application/octet-stream content type.
pub(crate) struct OctetStream(pub Vec<u8>);

impl IntoBody for OctetStream {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static("application/octet-stream"))
    }

    fn into_body(self) -> Result<Body> {
        Ok(self.0.into())
    }
}

/// Body wrapper for XML-serialized data using application/xml content type.
pub(crate) struct XmlBody<T: serde::Serialize>(pub T);

impl<T: serde::Serialize> IntoBody for XmlBody<T> {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static("application/xml"))
    }

    fn into_body(self) -> Result<Body> {
        let xml = quick_xml::se::to_string(&self.0).context("Failed to serialize body to XML")?;
        debug!("XML Request body: {}", xml);
        Ok(xml.into())
    }
}

/// Body wrapper for JSON-serialized data using application/json content type.
pub(crate) struct JsonBody<T: serde::Serialize>(pub T);

impl<T: serde::Serialize> IntoBody for JsonBody<T> {
    fn content_type(&self) -> Option<HeaderValue> {
        Some(HeaderValue::from_static("application/json"))
    }

    fn into_body(self) -> Result<Body> {
        let json = serde_json::to_string(&self.0).context("Failed to serialize body to JSON")?;
        debug!("JSON Request body: {}", json);
        Ok(json.into())
    }
}
