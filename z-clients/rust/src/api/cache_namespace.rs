// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct CacheNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> CacheNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Configure cache namespace
    pub async fn configure(
        &self,
        cache_configure_namespace_in: CacheConfigureNamespaceIn,
    ) -> Result<CacheConfigureNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.cache.namespace.configure")
            .with_body(cache_configure_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get cache namespace
    pub async fn get(
        &self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> Result<CacheGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.cache.namespace.get")
            .with_body(cache_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
