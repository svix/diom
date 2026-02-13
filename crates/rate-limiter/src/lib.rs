pub mod algorithms;
pub mod operations;
pub mod tables;

use std::time::Duration;

use coyote_error::Result;
use fjall::{Database, KeyspaceCreateOptions};
use fjall_utils::TableRow;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::algorithms::{FixedWindow, RateLimitConfig, TokenBucket};
use crate::{
    operations::{LimitOperation, ResetOperation},
    tables::{FixedWindowState, TokenBucketState},
};

#[derive(Clone)]
pub struct RateLimiter {
    #[allow(dead_code)]
    db: fjall::Database,
    tables: fjall::Keyspace,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitStatus {
    OK,
    BLOCK,
}

impl RateLimiter {
    pub fn new(path: &str, db: Database) -> Self {
        let rate_limiter_keyspace = format!("_coyote_rate_limiter_{path}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&rate_limiter_keyspace, || opts).unwrap()
        };

        Self { db, tables }
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        now: Timestamp,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
    ) -> Result<(RateLimitStatus, u64, Option<Duration>)> {
        self.limit_inner(now, identifier, delta, algorithm, true)
    }

    fn limit_inner(
        &self,
        now: Timestamp,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
        update: bool,
    ) -> Result<(RateLimitStatus, u64, Option<Duration>)> {
        match algorithm {
            RateLimitConfig::FixedWindow(fw_config) => {
                let window_start = fw_config.get_window_start(now);
                let max_tokens = fw_config.tokens;

                let identifier_key = identifier.to_string();
                let mut state = FixedWindowState::fetch(&self.tables, &identifier_key)?.unwrap_or(
                    FixedWindowState {
                        key: identifier_key.clone(),
                        count: 0,
                        window_start,
                    },
                );

                if state.window_start != window_start {
                    state.count = 0;
                }

                state.window_start = window_start;

                let (remaining, result, retry_after) = if state.count + delta > max_tokens {
                    state.count = max_tokens;
                    let window_end = window_start + fw_config.size;
                    let time_until_reset = window_end.duration_since(now).try_into().unwrap();
                    (0, RateLimitStatus::BLOCK, Some(time_until_reset))
                } else {
                    state.count += delta;
                    (max_tokens - state.count, RateLimitStatus::OK, None)
                };

                if update {
                    FixedWindowState::insert(&self.tables, &state)?;
                }

                Ok((result, remaining, retry_after))
            }

            RateLimitConfig::TokenBucket(tb_config) => {
                let identifier_key = identifier.to_string();
                let mut bucket = TokenBucketState::fetch(&self.tables, &identifier_key)?.unwrap_or(
                    TokenBucketState {
                        key: identifier_key.clone(),
                        tokens: tb_config.bucket_size,
                        last_refill: now,
                    },
                );

                let (capacity, new_last_refill) =
                    tb_config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

                if capacity < delta {
                    let filled_per_millis =
                        tb_config.refill_rate * tb_config.refill_interval.as_millis() as u64;
                    let retry_after = (filled_per_millis - capacity).div_ceil(delta);

                    return Ok((
                        RateLimitStatus::BLOCK,
                        capacity,
                        Some(Duration::from_millis(retry_after)),
                    ));
                }

                bucket.last_refill = new_last_refill;
                bucket.tokens = capacity - delta;

                if update {
                    TokenBucketState::insert(&self.tables, &bucket)?;
                }

                Ok((RateLimitStatus::OK, bucket.tokens, None))
            }
        }
    }

    pub fn get_remaining(
        &self,
        now: Timestamp,
        identifier: &str,
        algorithm: RateLimitConfig,
    ) -> Result<(u64, Option<Duration>)> {
        let (result, remaining, retry_after) =
            self.limit_inner(now, identifier, 1, algorithm, false)?;

        // We 'simulated' consuming 1 token, so we add it back to get the actual remaining capacity
        let actual_remaining = if matches!(result, RateLimitStatus::OK) {
            remaining + 1
        } else {
            remaining
        };

        Ok((actual_remaining, retry_after))
    }

    pub fn reset(&self, identifier: &str, algorithm: RateLimitConfig) -> Result<()> {
        let identifier_key = identifier.to_string();
        match algorithm {
            RateLimitConfig::FixedWindow(_) => {
                FixedWindowState::remove(&self.tables, &identifier_key)?;
            }
            RateLimitConfig::TokenBucket(_) => {
                TokenBucketState::remove(&self.tables, &identifier_key)?;
            }
        }
        Ok(())
    }

    pub fn limit_operation(key: String, units: u64, method: RateLimitConfig) -> LimitOperation {
        let now = Timestamp::now();
        LimitOperation::new(key, now, units, method)
    }

    pub fn reset_operation(key: String, method: RateLimitConfig) -> ResetOperation {
        ResetOperation::new(key, method)
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker<F>(_stores: &[&RateLimiter], is_shutting_down: F)
where
    F: Fn() -> bool,
{
    loop {
        if is_shutting_down() {
            break;
        }
        // FIXME(@svix-lucho): add background cleanup task. Cleanup logic depends on the algorithm.
        // Delete the expired/non-rate-limited entries first.
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    fn create_test_limiter() -> (RateLimiter, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = Database::builder(&dir).temporary(true).open().unwrap();
        let limiter = RateLimiter::new("rate-limiter", db);
        (limiter, dir)
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
        fn rate_limiting() {
            let (limiter, _dir) = create_test_limiter();
            let id = "user1";

            let mut clock = Timestamp::UNIX_EPOCH;
            let (result, remaining, retry_after) = limiter.limit(clock, id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(500);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(400);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::BLOCK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, Some(Duration::from_millis(100)));

            let (remaining_2, retry_after_2) = limiter.get_remaining(clock, id, config()).unwrap();
            assert_eq!(remaining_2, remaining);
            assert_eq!(retry_after_2, retry_after);

            // Resets at t=1000ms
            clock += Duration::from_millis(100);
            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 4);
            assert_eq!(retry_after, None);

            // Doesn't accumulate
            clock += Duration::from_millis(100000);
            let (remaining, retry_after) = limiter.get_remaining(clock, id, config()).unwrap();
            assert_eq!(remaining, 5);
            assert_eq!(retry_after, None);
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
        fn rate_limiting() {
            let (limiter, _dir) = create_test_limiter();
            let id = "user1";

            let clock = Timestamp::now();
            let (result, remaining, retry_after) = limiter.limit(clock, id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) = limiter.limit(clock, id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitStatus::BLOCK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, Some(Duration::from_millis(100)));
        }

        #[test]
        fn tokens_refill_over_time() {
            let (limiter, _dir) = create_test_limiter();
            let id = "user1";

            let mut clock = Timestamp::now();
            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 5, config_refill_2()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 0);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(100);
            let (remaining, retry_after) =
                limiter.get_remaining(clock, id, config_refill_2()).unwrap();
            assert_eq!(remaining, 2);
            assert_eq!(retry_after, None);

            clock += Duration::from_millis(200);
            let (remaining, retry_after) =
                limiter.get_remaining(clock, id, config_refill_2()).unwrap();
            assert_eq!(remaining, 5);
            assert_eq!(retry_after, None);

            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 2, config_refill_2()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 3);
            assert_eq!(retry_after, None);
        }

        #[test]
        fn refill_interval_tracking() {
            let (limiter, _dir) = create_test_limiter();
            let id = "user1";

            fn make_config() -> RateLimitConfig {
                RateLimitConfig::TokenBucket(TokenBucket {
                    refill_rate: 2,
                    refill_interval: Duration::from_secs(5),
                    bucket_size: 6,
                })
            }

            let mut clock = Timestamp::now();

            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 2, make_config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 4);
            assert_eq!(retry_after, None);

            clock += Duration::from_secs(2);
            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 1, make_config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 3);
            assert_eq!(retry_after, None);

            clock += Duration::from_secs(3);
            let (result, remaining, retry_after) =
                limiter.limit(clock, id, 1, make_config()).unwrap();
            assert!(matches!(result, RateLimitStatus::OK));
            assert_eq!(remaining, 4); // 4 (previous) + 2 (refill) - 1 (consumed)
            assert_eq!(retry_after, None);
        }
    }
}
