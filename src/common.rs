use crate::{auth::AliyunAuth, v3::call, IntoResponse, Request, Result};
use std::{future::Future, sync::Arc};

struct _Connection {
    auth: Arc<dyn AliyunAuth>,
    version: &'static str,
    end_point: &'static str,
    http_client: reqwest::Client,
}

#[derive(Clone)]
pub struct Connection(Arc<_Connection>);

impl Connection {
    pub fn new(
        auth: impl AliyunAuth + 'static,
        version: &'static str,
        end_point: &'static str,
    ) -> Self {
        Self(Arc::new(_Connection {
            auth: Arc::new(auth),
            version,
            end_point,
            http_client: reqwest::Client::default(),
        }))
    }

    pub fn with_client(
        auth: impl AliyunAuth + 'static,
        version: &'static str,
        end_point: &'static str,
        http_client: reqwest::Client,
    ) -> Self {
        Self(Arc::new(_Connection {
            auth: Arc::new(auth),
            version,
            end_point,
            http_client,
        }))
    }

    pub fn call<R: Request>(
        &self,
        req: R,
    ) -> impl Future<Output = Result<<R::ResponseWrap as IntoResponse>::Response>> + Send {
        call(
            self.0.auth.as_ref(),
            &self.0.http_client,
            self.0.version,
            self.0.end_point,
            req,
        )
    }
}
