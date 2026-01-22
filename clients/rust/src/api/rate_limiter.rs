// this file is @generated
use crate::{Configuration, error::Result, models::*};

#[derive(Default)]
pub struct RateLimiterLimitOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct RateLimiterGetRemainingOptions {
    pub idempotency_key: Option<String>,
}

pub struct RateLimiter<'a> {
    cfg: &'a Configuration,
}

impl<'a> RateLimiter<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Rate Limiter Check and Consume
    pub async fn limit(
        &self,
        rate_limiter_check_in: RateLimiterCheckIn,
        options: Option<RateLimiterLimitOptions>,
    ) -> Result<RateLimiterCheckOut> {
        let RateLimiterLimitOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limiter/limit")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(rate_limiter_check_in)
            .execute(self.cfg)
            .await
    }

    /// Rate Limiter Get Remaining
    pub async fn get_remaining(
        &self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
        options: Option<RateLimiterGetRemainingOptions>,
    ) -> Result<RateLimiterGetRemainingOut> {
        let RateLimiterGetRemainingOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limiter/get-remaining")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(rate_limiter_get_remaining_in)
            .execute(self.cfg)
            .await
    }
}
