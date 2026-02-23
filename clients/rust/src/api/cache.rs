// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct CacheSetOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct CacheGetOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct CacheGetGroupOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct CacheDeleteOptions {
    pub idempotency_key: Option<String>,
}

pub struct Cache<'a> {
    cfg: &'a Configuration,
}

impl<'a> Cache<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Cache Set
    pub async fn set(
        &self,
        cache_set_in: CacheSetIn,
        options: Option<CacheSetOptions>,
    ) -> Result<CacheSetOut> {
        let CacheSetOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/cache/set")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(cache_set_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Get
    pub async fn get(
        &self,
        cache_get_in: CacheGetIn,
        options: Option<CacheGetOptions>,
    ) -> Result<CacheGetOut> {
        let CacheGetOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/cache/get")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(cache_get_in)
            .execute(self.cfg)
            .await
    }

    /// Get cache group
    pub async fn get_group(
        &self,
        cache_get_group_in: CacheGetGroupIn,
        options: Option<CacheGetGroupOptions>,
    ) -> Result<CacheGetGroupOut> {
        let CacheGetGroupOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/cache/get-group")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(cache_get_group_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Delete
    pub async fn delete(
        &self,
        cache_delete_in: CacheDeleteIn,
        options: Option<CacheDeleteOptions>,
    ) -> Result<CacheDeleteOut> {
        let CacheDeleteOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/cache/delete")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(cache_delete_in)
            .execute(self.cfg)
            .await
    }
}
