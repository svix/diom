// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct Cache<'a> {
    cfg: &'a Configuration,
}

impl<'a> Cache<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Cache Set
    pub async fn set(&self, cache_set_in: CacheSetIn) -> Result<CacheSetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/cache/set")
            .with_body_param(cache_set_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Get
    pub async fn get(&self, cache_get_in: CacheGetIn) -> Result<CacheGetOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/cache/get")
            .with_body_param(cache_get_in)
            .execute(self.cfg)
            .await
    }

    /// Get cache group
    pub async fn get_group(&self, cache_get_group_in: CacheGetGroupIn) -> Result<CacheGetGroupOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/cache/get-group")
            .with_body_param(cache_get_group_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Delete
    pub async fn delete(&self, cache_delete_in: CacheDeleteIn) -> Result<CacheDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/cache/delete")
            .with_body_param(cache_delete_in)
            .execute(self.cfg)
            .await
    }
}
