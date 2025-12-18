use crate::{
    Request, Result,
    v3::{AccessKeySecret, call},
};
use std::{future::Future, rc::Rc};

struct _Connection {
    access_key_secret: AccessKeySecret,
    version: &'static str,
    end_point: &'static str,
    http_client: reqwest::Client,
}

#[derive(Clone)]
pub struct Connection(Rc<_Connection>);

impl Connection {
    pub fn new(
        access_key_secret: AccessKeySecret,
        version: &'static str,
        end_point: &'static str,
    ) -> Self {
        Self(Rc::new(_Connection {
            access_key_secret,
            version,
            end_point,
            http_client: reqwest::Client::default(),
        }))
    }

    pub fn call<R: Request>(&self, req: R) -> impl Future<Output = Result<R::Response>> + Send {
        call(
            &self.0.access_key_secret,
            &self.0.http_client,
            &self.0.version,
            &self.0.end_point,
            req,
        )
    }
}
