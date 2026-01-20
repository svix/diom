// Version with no dimensions.
// Dimensions have to be handled by the user with the correct identifier and a list of rate limiters.

use chrono::{DateTime, Duration, Utc};
use fjall::{OptimisticTxDatabase, OptimisticTxKeyspace};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, core::types::EntityKey, error::Result};

#[derive(Clone)]
pub struct RateLimiterStore {
    #[allow(unused)]
    db: OptimisticTxDatabase,

    token_bucket_data: OptimisticTxKeyspace,
    fixed_window_data: OptimisticTxKeyspace,
}

const DIOM_RATE_LIMITER_TOKEN_BUCKET_DATA_KEYSPACE: &str =
    "DIOM_RATE_LIMITER_TOKEN_BUCKET_DATA";
const DIOM_RATE_LIMITER_FIXED_WINDOW_DATA_KEYSPACE: &str =
    "DIOM_RATE_LIMITER_FIXED_WINDOW_DATA";

impl Default for RateLimiterStore {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiterStore {
    pub fn new() -> Self {
        Self::with_path(".data/rate_limiter_default")
    }

    pub fn with_path(path: &str) -> Self {
        let db = fjall::OptimisticTxDatabase::builder(path).open().unwrap();
        let token_bucket_data = db
            .keyspace(
                DIOM_RATE_LIMITER_TOKEN_BUCKET_DATA_KEYSPACE,
                fjall::KeyspaceCreateOptions::default,
            )
            .unwrap();
        let fixed_window_data = db
            .keyspace(
                DIOM_RATE_LIMITER_FIXED_WINDOW_DATA_KEYSPACE,
                fjall::KeyspaceCreateOptions::default,
            )
            .unwrap();

        Self {
            db,
            token_bucket_data,
            fixed_window_data,
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RateLimitModel {
    pub expires_at: Option<Timestamp>,
    pub value: Vec<u8>,
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

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct FixedWindowState {
    count: u64,
    window_start: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TokenBucketState {
    tokens: u64,
    last_refill: DateTime<Utc>,
}

#[derive(Clone)]
pub struct RateLimiter {
    store: RateLimiterStore,
    clock: Clock,
}

pub type Clock = Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>;

pub fn system_clock() -> Clock {
    Arc::new(Utc::now)
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        let store = RateLimiterStore::new();
        Self::with_clock(store, system_clock())
    }

    pub fn with_clock(store: RateLimiterStore, clock: Clock) -> Self {
        RateLimiter { store, clock }
    }

    fn now(&self) -> DateTime<Utc> {
        (self.clock)()
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        identifier: &EntityKey,
        delta: u64,
        algorithm: RateLimitConfig,
    ) -> fjall::Result<(RateLimitResult, u64, Option<Duration>)> {
        let now = self.now();
        // XXX: Unwrap
        let mut tx = self.store.db.write_tx().unwrap();

        match algorithm {
            RateLimitConfig::FixedWindow(config) => {
                let window_start = config.get_window_start(now);
                let max_tokens = config.tokens;

                let entry = tx
                    .take(&self.store.fixed_window_data, identifier.as_bytes())
                    .unwrap(); // XXX: Unwrap
                let mut state = entry.map_or(
                    FixedWindowState {
                        count: 0,
                        window_start,
                    },
                    |item| {
                        let state: FixedWindowState =
                            rmp_serde::from_slice(&item).expect("should deserialize");
                        state
                    },
                );

                if state.window_start != window_start {
                    state.count = 0;
                    state.window_start = window_start;
                }

                let (remaining, result, retry_after) = if state.count + delta > max_tokens {
                    state.count = max_tokens;
                    (0, RateLimitResult::BLOCK, Some(config.size))
                } else {
                    state.count += delta;
                    (max_tokens - state.count, RateLimitResult::OK, None)
                };

                tx.insert(
                    &self.store.fixed_window_data,
                    identifier.as_bytes(),
                    rmp_serde::to_vec(&state).unwrap(),
                );
                let _ = tx.commit().unwrap(); // XXX: Unwrap

                Ok((result, remaining, retry_after))
            }

            RateLimitConfig::TokenBucket(config) => {
                let bucket: Option<TokenBucketState> = tx
                    .take(&self.store.token_bucket_data, identifier.as_bytes())
                    .unwrap() // XXX: Unwrap
                    .map(|item| rmp_serde::from_slice(&item).expect("should deserialize"));

                let mut bucket = bucket.unwrap_or(TokenBucketState {
                    tokens: config.bucket_size,
                    last_refill: now,
                });
                let capacity = config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

                if capacity < delta {
                    let filled_per_millis =
                        config.refill_rate * config.refill_interval.num_milliseconds() as u64;
                    let retry_after = (filled_per_millis - capacity).div_ceil(delta) as i64;

                    return Ok((
                        RateLimitResult::BLOCK,
                        capacity,
                        Some(Duration::milliseconds(retry_after)),
                    ));
                }

                bucket.last_refill = now;
                bucket.tokens = capacity - delta;

                tx.insert(
                    &self.store.token_bucket_data,
                    identifier.as_bytes(),
                    rmp_serde::to_vec(&bucket).unwrap(),
                );
                let _ = tx.commit().unwrap(); // XXX: Unwrap

                Ok((RateLimitResult::OK, bucket.tokens, None))
            }
        }
    }

    pub fn get_remaining(
        &self,
        identifier: &EntityKey,
        algorithm: RateLimitConfig,
    ) -> fjall::Result<u64> {
        let now = self.now();
        match algorithm {
            RateLimitConfig::FixedWindow(config) => {
                let _window_start = config.get_window_start(now);
                let max_tokens = config.tokens;

                let item = self
                    .store
                    .fixed_window_data
                    .get(identifier.as_bytes())
                    .unwrap(); // XXX: Unwrap
                if let Some(item) = item {
                    let state: FixedWindowState =
                        rmp_serde::from_slice(&item).expect("should deserialize");
                    Ok(max_tokens.saturating_sub(state.count))
                } else {
                    Ok(max_tokens)
                }
            }
            RateLimitConfig::TokenBucket(config) => {
                let bucket: Option<TokenBucketState> = self
                    .store
                    .token_bucket_data
                    .get(identifier.as_bytes())
                    .unwrap() // XXX: Unwrap
                    .map(|item| rmp_serde::from_slice(&item).expect("should deserialize"));

                match bucket {
                    Some(bucket) => {
                        Ok(config.get_new_capacity(bucket.tokens, now, bucket.last_refill))
                    }
                    None => Ok(config.bucket_size),
                }
            }
        }
    }

    pub fn reset(
        &mut self,
        identifier: &EntityKey,
        algorithm: RateLimitConfig,
    ) -> fjall::Result<()> {
        // let key = algorithm.entity_key(identifier);
        match algorithm {
            RateLimitConfig::FixedWindow(_) => {
                self.store
                    .fixed_window_data
                    .remove(identifier.as_bytes())
                    .unwrap(); // XXX: Unwrap
            }
            RateLimitConfig::TokenBucket(_) => {
                self.store
                    .token_bucket_data
                    .remove(identifier.as_bytes())
                    .unwrap(); // XXX: Unwrap
            }
        }
        Ok(())
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(_state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        // TODO: Implement cleanup
        // We need to evict unused entries for the rate-limiter.
    }
    Ok(())
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
        let store = RateLimiterStore::with_path(dir.path().to_str().unwrap());
        let limiter = RateLimiter::with_clock(store, clock);
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
            let id: EntityKey = "user1".to_string().into();

            let (result, remaining, _) = limiter.limit(&id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            clock.set(500);
            let (result, remaining, _) = limiter.limit(&id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            clock.set(900);
            let (result, remaining, _) = limiter.limit(&id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
            assert_eq!(remaining, 0);
        }

        #[test]
        fn window_resets_after_interval() {
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id: EntityKey = "user1".to_string().into();

            limiter.limit(&id, 5, config()).unwrap();
            clock.set(500);
            assert_eq!(limiter.get_remaining(&id, config()).unwrap(), 0);

            // New window at t=1000
            clock.set(1000);
            let (result, remaining, _) = limiter.limit(&id, 1, config()).unwrap();
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
            let id: EntityKey = "user1".to_string().into();

            let (result, remaining, _) = limiter.limit(&id, 3, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 2);

            let (result, remaining, _) = limiter.limit(&id, 2, config()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 0);

            let (result, _, _) = limiter.limit(&id, 1, config()).unwrap();
            assert!(matches!(result, RateLimitResult::BLOCK));
        }

        #[test]
        fn tokens_refill_over_time() {
            let clock = MockClock::new(0);
            let (limiter, _dir) = create_test_limiter(clock.as_clock());
            let id: EntityKey = "user1".to_string().into();

            limiter.limit(&id, 5, config_refill_2()).unwrap();
            assert_eq!(limiter.get_remaining(&id, config_refill_2()).unwrap(), 0);

            clock.set(100);
            assert_eq!(limiter.get_remaining(&id, config_refill_2()).unwrap(), 2);
            clock.set(300);
            assert_eq!(limiter.get_remaining(&id, config_refill_2()).unwrap(), 5); // bucket is full

            let (result, remaining, _) = limiter.limit(&id, 2, config_refill_2()).unwrap();
            assert!(matches!(result, RateLimitResult::OK));
            assert_eq!(remaining, 3);
        }
    }
}
