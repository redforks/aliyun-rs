use serde::Deserialize;

use crate::{Request, Result};
use std::borrow::Cow;

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
    use crate::Result;
    use serde::Deserialize;

    /// prevent Request type used with Connection of other mod.
    pub trait Bound {}

    #[derive(Deserialize)]
    pub struct QuerySmsTemplateListResponse {}

    impl From<QuerySmsTemplateListResponse> for Result<()> {
        fn from(value: QuerySmsTemplateListResponse) -> Self {
            todo!()
        }
    }
}

#[derive(Clone)]
pub struct Connection(crate::common::Connection);

impl Connection {
    pub fn new(
        endpoint: Endpoint,
        app_key: impl Into<Cow<'static, str>>,
        app_securet: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self(crate::common::Connection::new(
            (app_key.into(), app_securet.into()),
            "2020-01-01",
            endpoint.into(),
        ))
    }

    pub fn call<R: Request + sealed::Bound>(
        &self,
        req: R,
    ) -> impl Future<Output = Result<R::Result>> + Send {
        self.0.call(req)
    }

    /// Helps for IDE auto complete
    pub fn query_sms(&self, req: QuerySmsTemplateList) -> impl Future<Output = Result<()>> + Send {
        self.call(req)
    }
}

#[derive(derive_setters::Setters)]
pub struct QuerySmsTemplateList {
    //
}

impl sealed::Bound for QuerySmsTemplateList {}

impl QuerySmsTemplateList {
    pub fn new(required_args: ()) -> Self {
        Self {}
    }
}

impl Request for QuerySmsTemplateList {
    const METHOD: http::Method = http::Method::GET;

    const ACTION: &'static str = "QuerySmsTemplateList";

    type Body = ();

    type Result = ();

    type Response = sealed::QuerySmsTemplateListResponse;

    fn to_body(self) -> Result<Self::Body> {
        todo!()
    }
}
