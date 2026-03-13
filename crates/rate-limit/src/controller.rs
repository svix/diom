use std::time::Duration;

use coyote_error::Result;
use coyote_namespace::entities::NamespaceId;
use fjall_utils::TableRow;
use jiff::Timestamp;

use crate::{algorithms::TokenBucket, tables::TokenBucketRow};

#[derive(Clone)]
pub struct RateLimitController {
    #[allow(dead_code)]
    db: fjall::Database,
    tables: fjall::Keyspace,
}

impl RateLimitController {
    pub fn new(db: fjall::Database, tables: fjall::Keyspace) -> Self {
        Self { db, tables }
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub fn limit(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        identifier: &str,
        delta: u64,
        config: TokenBucket,
    ) -> Result<(bool, u64, Option<Duration>)> {
        self.limit_inner(now, namespace_id, identifier, delta, config, true)
    }

    #[allow(clippy::too_many_arguments)]
    fn limit_inner(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        identifier: &str,
        delta: u64,
        config: TokenBucket,
        update: bool,
    ) -> Result<(bool, u64, Option<Duration>)> {
        let mut bucket = TokenBucketRow::fetch(
            &self.tables,
            TokenBucketRow::key_for(namespace_id, identifier),
        )?
        .unwrap_or(TokenBucketRow {
            tokens: config.bucket_size,
            last_refill: now,
        });

        let (capacity, new_last_refill) =
            config.get_new_capacity(bucket.tokens, now, bucket.last_refill);

        if capacity < delta {
            let filled_per_millis = config.refill_rate * config.refill_interval.as_millis() as u64;
            let retry_after = (filled_per_millis - capacity).div_ceil(delta);

            return Ok((false, capacity, Some(Duration::from_millis(retry_after))));
        }

        bucket.last_refill = new_last_refill;
        bucket.tokens = capacity - delta;

        if update {
            TokenBucketRow::insert(
                &self.tables,
                TokenBucketRow::key_for(namespace_id, identifier),
                &bucket,
            )?;
        }

        Ok((true, bucket.tokens, None))
    }

    pub fn get_remaining(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        identifier: &str,
        config: TokenBucket,
    ) -> Result<(u64, Option<Duration>)> {
        let (result, remaining, retry_after) =
            self.limit_inner(now, namespace_id, identifier, 1, config, false)?;

        // We 'simulated' consuming 1 token, so we add it back to get the actual remaining capacity
        let actual_remaining = if result { remaining + 1 } else { remaining };

        Ok((actual_remaining, retry_after))
    }

    pub fn reset(&self, namespace_id: NamespaceId, identifier: &str) -> Result<()> {
        TokenBucketRow::remove(
            &self.tables,
            TokenBucketRow::key_for(namespace_id, identifier),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tempfile::tempdir;
    use jiff::Timestamp;

    use super::*;
    use coyote_namespace::entities::NamespaceId;
    use fjall::KeyspaceCreateOptions;

    const TEST_KEYSPACE: &str = "mod_rate_limit";

    fn create_test_controller() -> (RateLimitController, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = fjall::Database::builder(&dir)
            .temporary(true)
            .open()
            .unwrap();
        let opts = || KeyspaceCreateOptions::default();
        let tables = db.keyspace(TEST_KEYSPACE, opts).unwrap();
        (RateLimitController::new(db, tables), dir)
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
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        let clock = Timestamp::now();
        let (result, remaining, retry_after) = limiter.limit(clock, ns(), id, 3, config()).unwrap();
        assert!(result);
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter.limit(clock, ns(), id, 2, config()).unwrap();
        assert!(result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter.limit(clock, ns(), id, 1, config()).unwrap();
        assert!(!result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, Some(Duration::from_millis(100)));
    }

    #[test]
    fn tokens_refill_over_time() {
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        let mut clock = Timestamp::now();
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 5, config_refill_2())
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(100);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), id, config_refill_2())
            .unwrap();
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(200);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), id, config_refill_2())
            .unwrap();
        assert_eq!(remaining, 5);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 2, config_refill_2())
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);
    }

    #[test]
    fn refill_interval_tracking() {
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        fn make_config() -> TokenBucket {
            TokenBucket {
                refill_rate: 2,
                refill_interval: Duration::from_secs(5),
                bucket_size: 6,
            }
        }

        let mut clock = Timestamp::now();

        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 2, make_config()).unwrap();
        assert!(result);
        assert_eq!(remaining, 4);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(2);
        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 1, make_config()).unwrap();
        assert!(result);
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(3);
        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 1, make_config()).unwrap();
        assert!(result);
        assert_eq!(remaining, 4); // 4 (previous) + 2 (refill) - 1 (consumed)
        assert_eq!(retry_after, None);
    }
}
