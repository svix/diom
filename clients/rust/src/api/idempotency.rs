// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Idempotency<'a> {
    cfg: &'a Configuration,
}

impl<'a> Idempotency<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Abandon an idempotent request (remove lock without saving response)
    pub async fn abort(
        &self,
        idempotency_abort_in: IdempotencyAbortIn,
    ) -> Result<IdempotencyAbortOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/abort")
            .with_body_param(idempotency_abort_in)
            .execute(self.cfg)
            .await
    }

    /// Get idempotency group
    pub async fn get_group(
        &self,
        idempotency_get_group_in: IdempotencyGetGroupIn,
    ) -> Result<IdempotencyGetGroupOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/get-group")
            .with_body_param(idempotency_get_group_in)
            .execute(self.cfg)
            .await
    }
}
