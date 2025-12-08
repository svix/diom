// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use dashmap::DashMap;
use std::sync::Arc;

use crate::{
    error::{Error, HttpError, Result},
    AppState,
};

/// Get current time in milliseconds since Unix epoch.
/// Uses system time which is consistent across distributed nodes.
/// Note: Subject to NTP adjustments and leap seconds, but rate limiters
/// are designed to tolerate small time discrepancies.
fn now_millis() -> u64 {
    chrono::Utc::now().timestamp_millis() as u64
}

// ============================================================================
// Token Bucket Rate Limiter
// ============================================================================

#[derive(Clone)]
pub struct TokenBucketRateLimiter {
    store: Arc<DashMap<String, TokenBucketState>>,
}

#[derive(Clone, Debug)]
struct TokenBucketState {
    tokens: u64,
    last_refill_millis: u64,
    // Configuration
    capacity: u64,
    refill_amount: u64,
    refill_interval_millis: u64,
}

impl Default for TokenBucketRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenBucketRateLimiter {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    pub fn configure(
        &self,
        key: &str,
        capacity: u64,
        refill_amount: u64,
        refill_interval_seconds: u64,
    ) {
        let now = now_millis();
        let refill_interval_millis = refill_interval_seconds * 1000;

        self.store.insert(
            key.to_string(),
            TokenBucketState {
                tokens: capacity,
                last_refill_millis: now,
                capacity,
                refill_amount,
                refill_interval_millis,
            },
        );
    }

    pub fn check_and_consume(
        &self,
        key: &str,
        tokens_requested: u64,
    ) -> Result<(bool, u64, Option<u64>)> {
        let now = now_millis();

        let mut entry = self.store.get_mut(key).ok_or_else(|| {
            Error::http(HttpError::not_found(
                Some("Rate limiter not configured for this key".into()),
                None,
            ))
        })?;

        let capacity = entry.capacity;
        let refill_amount = entry.refill_amount;
        let refill_interval_millis = entry.refill_interval_millis;

        // Refill tokens based on intervals elapsed
        let elapsed_millis = now.saturating_sub(entry.last_refill_millis);
        let intervals_elapsed = elapsed_millis / refill_interval_millis;

        if intervals_elapsed > 0 {
            let new_tokens = intervals_elapsed.saturating_mul(refill_amount);
            entry.tokens = entry.tokens.saturating_add(new_tokens).min(capacity);
            entry.last_refill_millis = now;
        }

        // Check if enough tokens available
        if entry.tokens >= tokens_requested {
            entry.tokens -= tokens_requested;
            Ok((true, entry.tokens, None))
        } else {
            // Calculate how long until we have enough tokens (in seconds)
            let tokens_needed = tokens_requested - entry.tokens;
            let intervals_needed = if refill_amount > 0 {
                tokens_needed.div_ceil(refill_amount) // Ceiling division
            } else {
                u64::MAX
            };
            let retry_after_millis = intervals_needed.saturating_mul(refill_interval_millis);
            let retry_after_seconds = retry_after_millis.div_ceil(1000); // Ceiling division to seconds
            Ok((false, entry.tokens, Some(retry_after_seconds)))
        }
    }

    pub fn get_remaining(&self, key: &str) -> Result<(u64, Option<u64>)> {
        let now = now_millis();

        match self.store.get(key) {
            Some(entry) => {
                let capacity = entry.capacity;
                let refill_amount = entry.refill_amount;
                let refill_interval_millis = entry.refill_interval_millis;

                let elapsed_millis = now.saturating_sub(entry.last_refill_millis);
                let intervals_elapsed = elapsed_millis / refill_interval_millis;
                let new_tokens = intervals_elapsed.saturating_mul(refill_amount);
                let current_tokens = entry.tokens.saturating_add(new_tokens).min(capacity);

                if current_tokens == 0 {
                    // Calculate retry_after for at least 1 token (in seconds)
                    let retry_after_seconds = refill_interval_millis.div_ceil(1000); // Ceiling division
                    Ok((0, Some(retry_after_seconds)))
                } else {
                    Ok((current_tokens, None))
                }
            }
            None => Err(Error::http(HttpError::not_found(
                Some("Rate limiter not configured for this key".into()),
                None,
            ))),
        }
    }
}

// ============================================================================
// Combined Rate Limiter Store
// ============================================================================

#[derive(Clone)]
pub struct RateLimiterStore {
    pub(crate) limiter: TokenBucketRateLimiter,
}

impl Default for RateLimiterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiterStore {
    pub fn new() -> Self {
        Self {
            limiter: TokenBucketRateLimiter::new(),
        }
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(_state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
