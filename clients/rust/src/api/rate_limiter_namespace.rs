// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct RateLimiterNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> RateLimiterNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create rate limiter namespace
    pub async fn create(
        &self,
        rate_limiter_create_namespace_in: RateLimiterCreateNamespaceIn,
    ) -> Result<RateLimiterCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/namespace/create")
            .with_body(rate_limiter_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get rate limiter namespace
    pub async fn get(
        &self,
        rate_limiter_get_namespace_in: RateLimiterGetNamespaceIn,
    ) -> Result<RateLimiterGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/namespace/get")
            .with_body(rate_limiter_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
