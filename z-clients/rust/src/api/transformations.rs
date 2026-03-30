// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Transformations<'a> {
    cfg: &'a Configuration,
}

impl<'a> Transformations<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Execute a JavaScript transformation script against a payload and return the result.
    pub async fn execute(&self, transform_in: TransformIn) -> Result<TransformOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.transformations.execute")
            .with_body(transform_in)
            .execute(self.cfg)
            .await
    }
}
