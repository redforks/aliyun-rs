use crate::{IntoResponse, Request, Result, auth::AliyunAuth, v3::call};
use std::{future::Future, sync::Arc};

struct _Connection<A: AliyunAuth> {
    auth: A,
    version: &'static str,
    end_point: &'static str,
    http_client: reqwest::Client,
}

#[derive(Clone)]
pub struct Connection<A: AliyunAuth>(Arc<_Connection<A>>);

impl<A: AliyunAuth> Connection<A> {
    pub fn new(
        auth: A,
        version: &'static str,
        end_point: &'static str,
    ) -> Self {
        Self(Arc::new(_Connection {
            auth,
            version,
            end_point,
            http_client: reqwest::Client::default(),
        }))
    }

    pub fn with_client(
        auth: A,
        version: &'static str,
        end_point: &'static str,
        http_client: reqwest::Client,
    ) -> Self {
        Self(Arc::new(_Connection {
            auth,
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
            &self.0.auth,
            &self.0.http_client,
            self.0.version,
            self.0.end_point,
            req,
        )
    }
}
