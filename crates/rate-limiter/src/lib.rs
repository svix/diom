pub mod tables;

use chrono::{DateTime, Duration, Utc};
use diom_error::{Error, Result};
use fjall::KeyspaceCreateOptions;
use fjall_utils::TableRow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tables::{FixedWindowState, TokenBucketState};

#[derive(Clone)]
pub struct RateLimiter {
    #[allow(dead_code)]
    db: fjall::Database,
    tables: fjall::Keyspace,
}

pub struct FixedWindow {
    /// Window size
    pub size: Duration,
    /// Max tokens allowed per window
    pub tokens: u64,
}

impl FixedWindow {
    fn get_window_start(&self, now: DateTime<Utc>) -> DateTime<Utc> {
        let size_ms = self.size.num_milliseconds();
        let now_ms = now.timestamp_millis();
        let window_start_ms = now_ms - (now_ms % size_ms);
        DateTime::from_timestamp_millis(window_start_ms).unwrap()
    }
}

pub struct TokenBucket {
    /// Token refill rate in tokens per refill interval
    pub refill_rate: u64,
    /// Token refill interval
    pub refill_interval: Duration,
    /// Max tokens allowed in the bucket
    pub bucket_size: u64,
}

impl TokenBucket {
    fn get_new_capacity(
        &self,
        current: u64,
        now: DateTime<Utc>,
        last_refill: DateTime<Utc>,
    ) -> u64 {
        let mut capacity = current;
        if last_refill < now {
            let elapsed = now - last_refill;
            let periods = elapsed.num_milliseconds() / self.refill_interval.num_milliseconds();
            capacity += periods as u64 * self.refill_rate;
        }
        capacity.min(self.bucket_size)
    }
}

pub enum RateLimitConfig {
    FixedWindow(FixedWindow),
    TokenBucket(TokenBucket),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone, JsonSchema)]
pub enum RateLimitResult {
    OK,
    BLOCK,
}

impl RateLimiter {
    pub fn init(db: fjall::Database) -> Result<Self, Error> {
        const RATE_LIMITER_KEYSPACE: &str = "_diom_rate_limiter";

        // There's probably more tweaking we can do for each of these tables, but for now,
        // this should suffice.
        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(RATE_LIMITER_KEYSPACE, || opts)?
        };

        Ok(Self { db, tables })
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        now: DateTime<Utc>,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
    ) -> Result<(RateLimitResult, u64, Option<Duration>)> {
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
                    (0, RateLimitResult::BLOCK, Some(fw_config.size))
                } else {
                    state.count += delta;
                    (max_tokens - state.count, RateLimitResult::OK, None)
                };

                FixedWindowState::insert(&self.tables, &state)?;

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

                let capacity = tb_config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

                if capacity < delta {
                    let filled_per_millis =
                        tb_config.refill_rate * tb_config.refill_interval.num_milliseconds() as u64;
                    let retry_after = (filled_per_millis - capacity).div_ceil(delta) as i64;

                    return Ok((
                        RateLimitResult::BLOCK,
                        capacity,
                        Some(Duration::milliseconds(retry_after)),
                    ));
                }

                bucket.last_refill = now;
                bucket.tokens = capacity - delta;

                TokenBucketState::insert(&self.tables, &bucket)?;

                Ok((RateLimitResult::OK, bucket.tokens, None))
            }
        }
    }

    pub fn get_remaining(
        &self,
        now: DateTime<Utc>,
        identifier: &str,
        algorithm: RateLimitConfig,
    ) -> Result<u64> {
        match algorithm {
            RateLimitConfig::FixedWindow(config) => {
                let window_start = config.get_window_start(now);
                let max_tokens = config.tokens;
                let identifier_key = identifier.to_string();

                let used_count = FixedWindowState::fetch(&self.tables, &identifier_key)?
                    .filter(|state| state.window_start == window_start)
                    .map_or(0, |state| state.count);

                Ok(max_tokens.saturating_sub(used_count))
            }
            RateLimitConfig::TokenBucket(config) => {
                let identifier_key = identifier.to_string();
                if let Some(bucket) = TokenBucketState::fetch(&self.tables, &identifier_key)? {
                    Ok(config.get_new_capacity(bucket.tokens, now, bucket.last_refill))
                } else {
                    Ok(config.bucket_size)
                }
            }
        }
    }

    pub fn reset(&mut self, identifier: &str, algorithm: RateLimitConfig) -> Result<()> {
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
    use super::*;

    mod fixed_window {
        use super::*;
        use test_utils::get_test_db;

        fn config() -> RateLimitConfig {
            RateLimitConfig::FixedWindow(FixedWindow {
                size: Duration::seconds(1),
                tokens: 5,
            })
        }

        #[test]
        fn rate_limiting() {
            let (db, _tempdir) = get_test_db();
            let limiter = RateLimiter::init(db).unwrap();
            let id = "user1";

            let mut clock = DateTime::from_timestamp_millis(0).unwrap();
            let (result, remaining, _) = limiter.limit(clock, id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            clock += Duration::milliseconds(500);
            let (result, remaining, _) = limiter.limit(clock, id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            clock += Duration::milliseconds(400);
            let (result, remaining, _) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
            assert_eq!(remaining, 0);

            // Resets at t=1000ms
            clock += Duration::milliseconds(100);
            let (result, remaining, _) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 4);

            // Doesn't accumulate
            clock += Duration::milliseconds(100000);
            let remaining = limiter.get_remaining(clock, id, config()).unwrap();
            assert_eq!(remaining, 5);
        }
    }

    mod token_bucket {
        use super::*;
        use test_utils::get_test_db;

        fn config() -> RateLimitConfig {
            RateLimitConfig::TokenBucket(TokenBucket {
                refill_rate: 1,
                refill_interval: Duration::milliseconds(100),
                bucket_size: 5,
            })
        }

        fn config_refill_2() -> RateLimitConfig {
            RateLimitConfig::TokenBucket(TokenBucket {
                refill_rate: 2,
                refill_interval: Duration::milliseconds(100),
                bucket_size: 5,
            })
        }

        #[test]
        fn rate_limiting() {
            let (db, _tempdir) = get_test_db();
            let limiter = RateLimiter::init(db).unwrap();
            let id = "user1";

            let clock = Utc::now();
            let (result, remaining, _) = limiter.limit(clock, id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            let (result, remaining, _) = limiter.limit(clock, id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            let (result, _, _) = limiter.limit(clock, id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
        }

        #[test]
        fn tokens_refill_over_time() {
            let (db, _tempdir) = get_test_db();
            let limiter = RateLimiter::init(db).unwrap();
            let id = "user1";

            let mut clock = Utc::now();
            limiter.limit(clock, id, 5, config_refill_2()).unwrap();
            assert_eq!(
                limiter.get_remaining(clock, id, config_refill_2()).unwrap(),
                0
            );

            clock += Duration::milliseconds(100);
            assert_eq!(
                limiter.get_remaining(clock, id, config_refill_2()).unwrap(),
                2
            );
            clock += Duration::milliseconds(200);
            assert_eq!(
                limiter.get_remaining(clock, id, config_refill_2()).unwrap(),
                5
            ); // bucket is full
            let (result, remaining, _) = limiter.limit(clock, id, 2, config_refill_2()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 3);
        }
    }
}
