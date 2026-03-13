pub mod algorithms;
pub mod operations;
pub mod tables;

use std::time::Duration;

use coyote_error::{Result, ResultExt as _};
use coyote_namespace::{Namespace, entities::NamespaceId};
use fjall::KeyspaceCreateOptions;
use fjall_utils::{Databases, StorageType, TableRow};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use crate::algorithms::TokenBucket;
use crate::tables::TokenBucketState;
pub use coyote_namespace::entities::RateLimitNamespaceConfig;

pub type RateLimitNamespace = Namespace<RateLimitNamespaceConfig>;

const RATE_LIMIT_KEYSPACE: &str = "mod_rate_limit";

#[derive(Clone)]
pub struct State {
    persistent_tables: fjall::Keyspace,
    ephemeral_tables: fjall::Keyspace,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitStatus {
    Ok,
    Block,
}

impl State {
    pub fn init(dbs: Databases) -> Result<Self> {
        let opts = || KeyspaceCreateOptions::default();
        Ok(Self {
            persistent_tables: dbs
                .persistent
                .keyspace(RATE_LIMIT_KEYSPACE, opts)
                .or_internal_error()?,
            ephemeral_tables: dbs
                .ephemeral
                .keyspace(RATE_LIMIT_KEYSPACE, opts)
                .or_internal_error()?,
        })
    }

    pub fn tables(&self, storage_type: StorageType) -> &fjall::Keyspace {
        match storage_type {
            StorageType::Persistent => &self.persistent_tables,
            StorageType::Ephemeral => &self.ephemeral_tables,
        }
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        storage_type: StorageType,
        identifier: &str,
        delta: u64,
        config: TokenBucket,
    ) -> Result<(RateLimitStatus, u64, Option<Duration>)> {
        self.limit_inner(
            now,
            namespace_id,
            storage_type,
            identifier,
            delta,
            config,
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn limit_inner(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        storage_type: StorageType,
        identifier: &str,
        delta: u64,
        config: TokenBucket,
        update: bool,
    ) -> Result<(RateLimitStatus, u64, Option<Duration>)> {
        let tables = self.tables(storage_type);

        let mut bucket =
            TokenBucketState::fetch(tables, TokenBucketState::key_for(namespace_id, identifier))?
                .unwrap_or(TokenBucketState {
                    tokens: config.bucket_size,
                    last_refill: now,
                });

        let (capacity, new_last_refill) =
            config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

        if capacity < delta {
            let filled_per_millis = config.refill_rate * config.refill_interval.as_millis() as u64;
            let retry_after = (filled_per_millis - capacity).div_ceil(delta);

            return Ok((
                RateLimitStatus::Block,
                capacity,
                Some(Duration::from_millis(retry_after)),
            ));
        }

        bucket.last_refill = new_last_refill;
        bucket.tokens = capacity - delta;

        if update {
            TokenBucketState::insert(
                tables,
                TokenBucketState::key_for(namespace_id, identifier),
                &bucket,
            )?;
        }

        Ok((RateLimitStatus::Ok, bucket.tokens, None))
    }

    pub fn get_remaining(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        storage_type: StorageType,
        identifier: &str,
        config: TokenBucket,
    ) -> Result<(u64, Option<Duration>)> {
        let (result, remaining, retry_after) = self.limit_inner(
            now,
            namespace_id,
            storage_type,
            identifier,
            1,
            config,
            false,
        )?;

        // We 'simulated' consuming 1 token, so we add it back to get the actual remaining capacity
        let actual_remaining = if matches!(result, RateLimitStatus::Ok) {
            remaining + 1
        } else {
            remaining
        };

        Ok((actual_remaining, retry_after))
    }

    pub fn reset(
        &self,
        namespace_id: NamespaceId,
        storage_type: StorageType,
        identifier: &str,
    ) -> Result<()> {
        let tables = self.tables(storage_type);
        TokenBucketState::remove(tables, TokenBucketState::key_for(namespace_id, identifier))?;
        Ok(())
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker<F>(_stores: &[&State], is_shutting_down: F)
where
    F: Fn() -> bool,
{
    loop {
        if is_shutting_down() {
            break;
        }
        // FIXME(@svix-lucho): add background cleanup task. Cleanup logic depends on the algorithm.
        // Delete the expired/non-rate-limited entries first.
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    fn create_test_state() -> (State, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        let persistent_db = fjall::Database::builder(&dir)
            .temporary(true)
            .open()
            .unwrap();
        let ephemeral_db = fjall::Database::builder(&dir2)
            .temporary(true)
            .open()
            .unwrap();
        let dbs = Databases::new(persistent_db, ephemeral_db);
        let state = State::init(dbs).unwrap();
        (state, dir)
    }

    fn ns() -> NamespaceId {
        NamespaceId::nil()
    }

    fn config() -> TokenBucket {
        TokenBucket {
            refill_rate: 1,
            refill_interval: Duration::from_millis(100),
            bucket_size: 5,
        }
    }

    fn config_refill_2() -> TokenBucket {
        TokenBucket {
            refill_rate: 2,
            refill_interval: Duration::from_millis(100),
            bucket_size: 5,
        }
    }

    #[test]
    fn rate_limiting() {
        let (limiter, _dir) = create_test_state();
        let id = "user1";

        let clock = Timestamp::now();
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 3, config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 2, config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 1, config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Block));
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, Some(Duration::from_millis(100)));
    }

    #[test]
    fn tokens_refill_over_time() {
        let (limiter, _dir) = create_test_state();
        let id = "user1";

        let mut clock = Timestamp::now();
        let (result, remaining, retry_after) = limiter
            .limit(
                clock,
                ns(),
                StorageType::Persistent,
                id,
                5,
                config_refill_2(),
            )
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(100);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), StorageType::Persistent, id, config_refill_2())
            .unwrap();
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(200);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), StorageType::Persistent, id, config_refill_2())
            .unwrap();
        assert_eq!(remaining, 5);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter
            .limit(
                clock,
                ns(),
                StorageType::Persistent,
                id,
                2,
                config_refill_2(),
            )
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);
    }

    #[test]
    fn refill_interval_tracking() {
        let (limiter, _dir) = create_test_state();
        let id = "user1";

        fn make_config() -> TokenBucket {
            TokenBucket {
                refill_rate: 2,
                refill_interval: Duration::from_secs(5),
                bucket_size: 6,
            }
        }

        let mut clock = Timestamp::now();

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 2, make_config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 4);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(2);
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 1, make_config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(3);
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), StorageType::Persistent, id, 1, make_config())
            .unwrap();
        assert!(matches!(result, RateLimitStatus::Ok));
        assert_eq!(remaining, 4); // 4 (previous) + 2 (refill) - 1 (consumed)
        assert_eq!(retry_after, None);
    }
}
