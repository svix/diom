// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! # Rate Limiter Module
//!
//! This module implements a token bucket rate limiter with dynamic configuration.
//!
//! ## Data Structure Design
//!
//! The rate limiter uses a single DashMap to store per-key state:
//!
//! 1. Main Store (DashMap) - Storage for token bucket state per key (tokens, last_refill_millis)
//!
//! ## Algorithm: Token Bucket
//!
//! Each key maintains a "bucket" of tokens that refills over time:
//! - **Capacity**: Maximum number of tokens the bucket can hold
//! - **Refill**: Tokens are added at a configured rate (amount per interval)
//! - **Consumption**: Each request consumes a specified number of tokens
//! - **Rate Limiting**: Requests are denied when insufficient tokens are available
//!
//! ## How It Works
//!
//! - **Configuration**: Passed with each request (capacity, refill_amount, refill_interval_seconds)
//! - **State**: Only runtime state is stored (current tokens, last refill timestamp)
//! - **On Request**:
//!   - Create state if first request (starts with full capacity)
//!   - Calculate and add refilled tokens based on elapsed time
//!   - Check if sufficient tokens available and consume if allowed
//! - **Get Remaining**: Query current token count without consuming
//!
//! The design keeps configuration stateless - it's provided with each API call rather than
//! being stored or pre-configured. This allows flexible per-request configuration changes.
//!
//! ## TODO FIXME
//! - Should expire the rate limiter keys when they are "full" or otherwise unused for X amount of
//! time.

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

    pub fn check_and_consume(
        &self,
        key: &str,
        tokens_requested: u64,
        capacity: u64,
        refill_amount: u64,
        refill_interval_seconds: u64,
    ) -> Result<(bool, u64, Option<u64>)> {
        let now = now_millis();
        let refill_interval_millis = refill_interval_seconds * 1000;

        // Get or create entry with initial state
        let mut entry = self
            .store
            .entry(key.to_string())
            .or_insert_with(|| TokenBucketState {
                tokens: capacity,
                last_refill_millis: now,
            });

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

    pub fn get_remaining(
        &self,
        key: &str,
        capacity: u64,
        refill_amount: u64,
        refill_interval_seconds: u64,
    ) -> Result<(u64, Option<u64>)> {
        let now = now_millis();
        let refill_interval_millis = refill_interval_seconds * 1000;

        match self.store.get(key) {
            Some(entry) => {
                // Calculate current tokens based on elapsed time and provided configuration
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
            None => {
                // No state exists yet, so full capacity is available
                Ok((capacity, None))
            }
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
