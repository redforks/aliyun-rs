use crate::{
    Request, Result,
    v3::{AccessKeyNSecret, call},
};
use std::rc::Rc;

struct _Connection {
    access_key_secret: AccessKeyNSecret,
    version: &'static str,
    end_point: &'static str,
    http_client: reqwest::Client,
}

#[derive(Clone)]
pub struct Connection(Rc<_Connection>);

impl Connection {
    pub fn new(
        access_key_secret: AccessKeyNSecret,
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

    pub fn call<R: Request>(&self, req: R) -> impl Future<Output = Result<R::Result>> + Send {
        call(&self.0.access_key_secret, &self.0.http_client, req)
    }
}
