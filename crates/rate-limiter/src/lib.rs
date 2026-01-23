pub mod tables;

use chrono::{DateTime, Duration, Utc};
use coyote_error::Result;
use fjall::KeyspaceCreateOptions;
use fjall_utils::TableRow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::tables::{FixedWindowState, TokenBucketState};

#[derive(Clone)]
pub struct RateLimiter {
    #[allow(dead_code)]
    db: fjall::Database,
    tables: fjall::Keyspace,
    clock: Clock,
}

pub struct FixedWindowConfig {
    /// Window size
    pub size: Duration,
    /// Max tokens allowed per window
    pub tokens: u64,
}

impl FixedWindowConfig {
    fn get_window_start(&self, now: DateTime<Utc>) -> DateTime<Utc> {
        let size_ms = self.size.num_milliseconds();
        let now_ms = now.timestamp_millis();
        let window_start_ms = (now_ms / size_ms) * size_ms;
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
    FixedWindow(FixedWindowConfig),
    TokenBucket(TokenBucket),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone, JsonSchema)]
pub enum RateLimitResult {
    OK,
    BLOCK,
}

pub type Clock = Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>;

pub fn system_clock() -> Clock {
    Arc::new(Utc::now)
}

impl RateLimiter {
    // FIXME(@svix-lucho): get the db from the app state (caller)
    pub fn new() -> Self {
        Self::with_path(".data/rate_limiter_default")
    }

    pub fn with_path(path: &str) -> Self {
        Self::with_path_and_clock(path, system_clock())
    }

    pub fn with_path_and_clock(path: &str, clock: Clock) -> Self {
        let db = fjall::Database::builder(path).open().unwrap();

        let rate_limiter_keyspace = format!("_coyote_rate_limiter_{path}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&rate_limiter_keyspace, || opts).unwrap()
        };

        Self { db, tables, clock }
    }

    fn now(&self) -> DateTime<Utc> {
        (self.clock)()
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        identifier: &str,
        delta: u64,
        algorithm: RateLimitConfig,
    ) -> Result<(RateLimitResult, u64, Option<Duration>)> {
        // FIXME(@svix-lucho): should receive now() from the caller
        let now = self.now();

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
                    state.window_start = window_start;
                }

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

    pub fn get_remaining(&self, identifier: &str, algorithm: RateLimitConfig) -> Result<u64> {
        let now = self.now();
        match algorithm {
            RateLimitConfig::FixedWindow(config) => {
                let _window_start = config.get_window_start(now);
                let max_tokens = config.tokens;
                let identifier_key = identifier.to_string();

                if let Some(state) = FixedWindowState::fetch(&self.tables, &identifier_key)? {
                    Ok(max_tokens.saturating_sub(state.count))
                } else {
                    Ok(max_tokens)
                }
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
    use std::sync::Mutex;
    use tempfile::tempdir;

    struct MockClock {
        time: Mutex<DateTime<Utc>>,
    }

    impl MockClock {
        fn new(initial_ms: i64) -> Arc<Self> {
            Arc::new(MockClock {
                time: Mutex::new(DateTime::from_timestamp_millis(initial_ms).unwrap()),
            })
        }

        fn set(&self, ms: i64) {
            *self.time.lock().unwrap() = DateTime::from_timestamp_millis(ms).unwrap();
        }

        fn as_clock(self: &Arc<Self>) -> Clock {
            let clock = Arc::clone(self);
            Arc::new(move || *clock.time.lock().unwrap())
        }
    }

    fn create_test_limiter(clock: Clock) -> (RateLimiter, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let limiter = RateLimiter::with_path_and_clock(dir.path().to_str().unwrap(), clock);
        (limiter, dir)
    }

    mod fixed_window {
        use super::*;

        fn config() -> RateLimitConfig {
            RateLimitConfig::FixedWindow(FixedWindowConfig {
                size: Duration::seconds(1),
                tokens: 5,
            })
        }

        #[test]
        fn rate_limiting() {
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id = "user1";

            let (result, remaining, _) = limiter.limit(id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            clock.set(500);
            let (result, remaining, _) = limiter.limit(id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            clock.set(900);
            let (result, remaining, _) = limiter.limit(id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
            assert_eq!(remaining, 0);
        }

        #[test]
        fn window_resets_after_interval() {
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id = "user1";

            limiter.limit(id, 5, config()).unwrap();
            clock.set(500);
            assert_eq!(limiter.get_remaining(id, config()).unwrap(), 0);

            // New window at t=1000
            clock.set(1000);
            let (result, remaining, _) = limiter.limit(id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 4);
        }
    }

    mod token_bucket {
        use super::*;

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
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id = "user1";

            let (result, remaining, _) = limiter.limit(id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            let (result, remaining, _) = limiter.limit(id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            let (result, _, _) = limiter.limit(id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
        }

        #[test]
        fn tokens_refill_over_time() {
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id = "user1";

            limiter.limit(id, 5, config_refill_2()).unwrap();
            assert_eq!(limiter.get_remaining(id, config_refill_2()).unwrap(), 0);

            clock.set(100);
            assert_eq!(limiter.get_remaining(id, config_refill_2()).unwrap(), 2);
            clock.set(300);
            assert_eq!(limiter.get_remaining(id, config_refill_2()).unwrap(), 5); // bucket is full

            let (result, remaining, _) = limiter.limit(id, 2, config_refill_2()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 3);
        }
    }
}
