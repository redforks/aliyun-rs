#![allow(dead_code)]
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::{CodeMessage, QueryValue, Request, Result, v3::AccessKeySecret};
use serde::Deserialize;

#[derive(Clone, Copy)]
pub enum Endpoint {
    CnShanghaiFinance1,
    NaSouth1,
}

impl From<Endpoint> for &'static str {
    fn from(value: Endpoint) -> Self {
        match value {
            Endpoint::CnShanghaiFinance1 => "ecs.cn-shanghai-finance-1.aliyuncs.com",
            Endpoint::NaSouth1 => "ecs.na-south-1.aliyuncs.com",
        }
    }
}

mod sealed {
    /// prevent Request type used with Connection of other mod.
    pub trait Bound {}
}

#[derive(Clone)]
pub struct Connection(crate::common::Connection);

impl Connection {
    pub fn new(endpoint: Endpoint, app_key_secret: AccessKeySecret) -> Self {
        Self(crate::common::Connection::new(
            app_key_secret,
            "2020-01-01",
            endpoint.into(),
        ))
    }

    fn call<R: Request + sealed::Bound>(
        &self,
        req: R,
    ) -> impl Future<Output = Result<R::Response>> + Send {
        self.0.call(req)
    }

    pub fn send_sms(&self, req: SendSms) -> impl Future<Output = Result<SendSmsResponse>> + Send {
        self.call(req)
    }
}

#[derive(derive_setters::Setters, Debug)]
#[setters(generate = false)]
pub struct SendSms {
    phone_numbers: String,
    sign_name: String,
    template_code: String,
    #[setters(generate = true, strip_option)]
    template_param: Option<String>,
    #[setters(generate = true, strip_option)]
    sms_up_extend_code: Option<String>,
    #[setters(generate = true, strip_option)]
    out_id: Option<String>,
}

impl sealed::Bound for SendSms {}

impl SendSms {
    pub fn new(
        phone_numbers: impl Into<String>,
        sign_name: impl Into<String>,
        template_code: impl Into<String>,
    ) -> Self {
        SendSms {
            phone_numbers: phone_numbers.into(),
            sign_name: sign_name.into(),
            template_code: template_code.into(),
            template_param: None,
            sms_up_extend_code: None,
            out_id: None,
        }
    }
}

impl Request for SendSms {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "SendSms";

    type Body = ();

    type Response = SendSmsResponse;

    fn to_query_params(&self) -> crate::Result<BTreeMap<Cow<'static, str>, QueryValue<'_>>> {
        let mut params = BTreeMap::new();
        params.insert("PhoneNumbers".into(), (&self.phone_numbers).into());
        params.insert("SignName".into(), (&self.sign_name).into());
        params.insert("TemplateCode".into(), (&self.template_code).into());
        if let Some(ref v) = self.template_param {
            params.insert("TemplateParam".into(), v.into());
        }
        if let Some(ref v) = self.sms_up_extend_code {
            params.insert("SmsUpExtendCode".into(), v.into());
        }
        if let Some(ref v) = self.out_id {
            params.insert("OutId".into(), v.into());
        }
        Ok(params)
    }

    fn to_body(self) -> Result<Self::Body> {
        todo!()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendSmsResponse {
    #[serde(flatten)]
    code_message: CodeMessage,
    biz_id: String,
    request_id: String,
}
