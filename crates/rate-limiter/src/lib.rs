pub mod algorithms;
pub mod tables;

use std::{cmp::max, time::Duration};

use diom_error::Result;
use diom_kv::{KvModel, KvStore, OperationBehavior};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::algorithms::{FixedWindow, RateLimitConfig, TokenBucket};
use crate::tables::{FixedWindowState, TokenBucketState};

#[derive(Clone)]
pub struct RateLimiter {
    pub kv: KvStore,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone, JsonSchema)]
pub enum RateLimitResult {
    OK,
    BLOCK,
}

impl RateLimiter {
    pub fn new(kv: KvStore) -> Self {
        Self { kv }
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &mut self,
        now: Timestamp,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
    ) -> Result<(RateLimitResult, u64, Option<Duration>)> {
        self.limit_inner(now, identifier, delta, algorithm, false)
    }

    fn limit_inner(
        &mut self,
        now: Timestamp,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
        dry_run: bool,
    ) -> Result<(RateLimitResult, u64, Option<Duration>)> {
        match algorithm {
            RateLimitConfig::FixedWindow(fw_config) => {
                let window_start = fw_config.get_window_start(now);
                let max_tokens = fw_config.tokens;

                let identifier_key = fw_config.get_key(identifier);
                let mut state = self
                    .kv
                    .get(identifier_key.as_str())?
                    .map(|m| m.value.into())
                    .unwrap_or(FixedWindowState {
                        key: identifier_key.clone(),
                        count: 0,
                        window_start,
                    });

                if state.window_start != window_start {
                    state.count = 0;
                }

                state.window_start = window_start;

                let (remaining, result, retry_after) = if state.count + delta > max_tokens {
                    state.count = max_tokens;
                    let window_end = window_start + fw_config.size;
                    let time_until_reset = window_end.duration_since(now).try_into().unwrap();
                    (0, RateLimitResult::BLOCK, Some(time_until_reset))
                } else {
                    state.count += delta;
                    (max_tokens - state.count, RateLimitResult::OK, None)
                };

                if !dry_run {
                    self.kv.set(
                        identifier_key.as_str(),
                        &KvModel {
                            value: state.into(),
                            expires_at: None, // FIXME(@svix-lucho): add expiration
                        },
                        OperationBehavior::Upsert,
                    )?;
                }

                Ok((result, remaining, retry_after))
            }

            RateLimitConfig::TokenBucket(tb_config) => {
                let identifier_key = tb_config.get_key(identifier);

                let mut bucket = self
                    .kv
                    .get(identifier_key.as_str())?
                    .map(|m| m.value.into())
                    .unwrap_or(TokenBucketState {
                        key: identifier_key.clone(),
                        tokens: tb_config.bucket_size,
                        last_refill: now,
                    });

                let capacity = tb_config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

                if capacity < delta {
                    let filled_per_millis =
                        tb_config.refill_rate * tb_config.refill_interval.as_millis() as u64;
                    let retry_after = (filled_per_millis - capacity).div_ceil(delta);

                    return Ok((
                        RateLimitResult::BLOCK,
                        capacity,
                        Some(Duration::from_millis(retry_after)),
                    ));
                }

                bucket.last_refill = now;
                bucket.tokens = capacity - delta;

                if !dry_run {
                    self.kv.set(
                        identifier_key.as_str(),
                        &KvModel {
                            value: bucket.clone().into(),
                            expires_at: None, // FIXME(@svix-lucho): add expiration
                        },
                        OperationBehavior::Upsert,
                    )?;
                }

                Ok((RateLimitResult::OK, bucket.tokens, None))
            }
        }
    }

    pub fn get_remaining(
        &mut self,
        now: Timestamp,
        identifier: &str,
        algorithm: RateLimitConfig,
    ) -> Result<(u64, Option<Duration>)> {
        let (result, remaining, retry_after) =
            self.limit_inner(now, identifier, 1, algorithm, true)?;

        // We did a 'dry-run' consuming 1 token, so we add it back to get the actual remaining capacity
        let actual_remaining = if matches!(result, RateLimitResult::OK) {
            remaining + 1
        } else {
            remaining
        };

        Ok((actual_remaining, retry_after))
    }

    pub fn reset(&mut self, identifier: &str, algorithm: RateLimitConfig) -> Result<()> {
        match algorithm {
            RateLimitConfig::FixedWindow(fw_config) => {
                let key = fw_config.get_key(identifier);
                self.kv.delete(key.as_str())?;
            }
            RateLimitConfig::TokenBucket(tb_config) => {
                let key = tb_config.get_key(identifier);
                self.kv.delete(key.as_str())?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diom_kv::EvictionPolicy;
    use fjall::Database;
    use test_utils::TestResult;

    fn create_test_limiter() -> RateLimiter {
        let workdir = tempfile::tempdir().unwrap();
        let db = Database::builder(workdir.as_ref())
            .temporary(true)
            .open()
            .unwrap();
        let kv = KvStore::new("rate-limiter", db, EvictionPolicy::NoEviction);
        RateLimiter::new(kv)
    }

    mod fixed_window {
        use super::*;

        fn config() -> RateLimitConfig {
            RateLimitConfig::FixedWindow(FixedWindow {
                size: Duration::from_secs(1),
                tokens: 5,
            })
        }

        #[test]
        fn rate_limiting() -> TestResult {
            let mut limiter = create_test_limiter();
            let id = "user1";

            let mut clock = Timestamp::UNIX_EPOCH;
            let (result, remaining, retry_after) = limiter.limit(clock, id, 3, config())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(500);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 2, config())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(400);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config())?;
            assert!(matches!(result, RateLimitResult::BLOCK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, Some(Duration::from_millis(100)));

            let (remaining_2, retry_after_2) = limiter.get_remaining(clock, id, config())?;
            assert_eq!(remaining_2, remaining);
            assert_eq!(retry_after_2, retry_after);

            // Resets at t=1000ms
            clock += Duration::from_millis(100);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 4);
            assert_eq!(retry_after, None);

            // Doesn't accumulate
            clock += Duration::from_millis(100000);
            let (remaining, retry_after) = limiter.get_remaining(clock, id, config())?;
            assert_eq!(remaining, 5);
            assert_eq!(retry_after, None);

            Ok(())
        }
    }

    mod token_bucket {
        use super::*;

        fn config() -> RateLimitConfig {
            RateLimitConfig::TokenBucket(TokenBucket {
                refill_rate: 1,
                refill_interval: Duration::from_millis(100),
                bucket_size: 5,
            })
        }

        fn config_refill_2() -> RateLimitConfig {
            RateLimitConfig::TokenBucket(TokenBucket {
                refill_rate: 2,
                refill_interval: Duration::from_millis(100),
                bucket_size: 5,
            })
        }

        #[test]
        fn rate_limiting() -> TestResult {
            let mut limiter = create_test_limiter();
            let id = "user1";

            let clock = Timestamp::now();
            let (result, remaining, retry_after) = limiter.limit(clock, id, 3, config())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) = limiter.limit(clock, id, 2, config())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config())?;
            assert!(matches!(result, RateLimitResult::BLOCK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, Some(Duration::from_millis(100)));
            Ok(())
        }

        #[test]
        fn tokens_refill_over_time() -> TestResult {
            let mut limiter = create_test_limiter();
            let id = "user1";

            let mut clock = Timestamp::now();
            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 5, config_refill_2())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(100);
            let (remaining, retry_after) = limiter.get_remaining(clock, id, config_refill_2())?;
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(200);
            let (remaining, retry_after) = limiter.get_remaining(clock, id, config_refill_2())?;
            assert_eq!(remaining, 5);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 2, config_refill_2())?;
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 3);
            assert_eq!(retry_after, None);
            Ok(())
        }
    }
}
