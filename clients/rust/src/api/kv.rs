// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct KvSetOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct KvGetOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct KvDeleteOptions {
    pub idempotency_key: Option<String>,
}

pub struct Kv<'a> {
    cfg: &'a Configuration,
}

impl<'a> Kv<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// KV Set
    pub async fn set(&self, kv_set_in: KvSetIn, options: Option<KvSetOptions>) -> Result<KvSetOut> {
        let KvSetOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/set")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(kv_set_in)
            .execute(self.cfg)
            .await
    }

    /// KV Get
    pub async fn get(&self, kv_get_in: KvGetIn, options: Option<KvGetOptions>) -> Result<KvGetOut> {
        let KvGetOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/get")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(kv_get_in)
            .execute(self.cfg)
            .await
    }

    /// KV Delete
    pub async fn delete(
        &self,
        kv_delete_in: KvDeleteIn,
        options: Option<KvDeleteOptions>,
    ) -> Result<KvDeleteOut> {
        let KvDeleteOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/delete")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(kv_delete_in)
            .execute(self.cfg)
            .await
    }
}
