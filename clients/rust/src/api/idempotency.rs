// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct IdempotencyAbortOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct IdempotencyGetGroupOptions {
    pub idempotency_key: Option<String>,
}

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
        options: Option<IdempotencyAbortOptions>,
    ) -> Result<IdempotencyAbortOut> {
        let IdempotencyAbortOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/abort")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(idempotency_abort_in)
            .execute(self.cfg)
            .await
    }

    /// Get idempotency group
    pub async fn get_group(
        &self,
        idempotency_get_group_in: IdempotencyGetGroupIn,
        options: Option<IdempotencyGetGroupOptions>,
    ) -> Result<IdempotencyGetGroupOut> {
        let IdempotencyGetGroupOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/idempotency/get-group")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(idempotency_get_group_in)
            .execute(self.cfg)
            .await
    }
}
