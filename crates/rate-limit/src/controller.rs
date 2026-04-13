use diom_core::{task::spawn_blocking_in_current_span, types::DurationMs};
use diom_error::{Result, ResultExt as _};
use diom_id::NamespaceId;
use fjall::KeyspaceCreateOptions;
use fjall_utils::TableRow;
use jiff::Timestamp;

use crate::{RATE_LIMIT_KEYSPACE, algorithms::TokenBucket, tables::TokenBucketRow};

#[derive(Clone)]
pub struct RateLimitController {
    #[allow(dead_code)]
    db: fjall::Database,
    tables: fjall::Keyspace,
}

impl RateLimitController {
    pub fn new(db: fjall::Database) -> Result<Self> {
        let opts = || KeyspaceCreateOptions::default();
        let tables = db.keyspace(RATE_LIMIT_KEYSPACE, opts).or_internal_error()?;
        Ok(Self { db, tables })
    }

    fn calculate_capacity(
        tables: &fjall::Keyspace,
        namespace_id: NamespaceId,
        identifier: &str,
        config: &TokenBucket,
        now: Timestamp,
    ) -> Result<(u64, Timestamp)> {
        let bucket =
            TokenBucketRow::fetch(tables, TokenBucketRow::key_for(namespace_id, identifier))?
                .unwrap_or(TokenBucketRow {
                    tokens: config.bucket_size,
                    last_refill: now,
                });
        Ok(config.get_new_capacity(bucket.tokens, now, bucket.last_refill))
    }

    // Using the same identifier but changing the algorithm is considered a different resource.
    pub async fn limit<I: AsRef<str> + 'static + Send>(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        identifier: I,
        wanted: u64,
        config: TokenBucket,
    ) -> Result<(bool, u64, Option<DurationMs>)> {
        let tables = self.tables.clone();

        spawn_blocking_in_current_span(move || {
            let identifier = identifier.as_ref();
            let (capacity, new_last_refill) =
                Self::calculate_capacity(&tables, namespace_id, identifier, &config, now)?;

            if capacity < wanted {
                let retry_after = config.calculate_retry_after(capacity, wanted);
                return Ok((false, capacity, Some(retry_after)));
            }

            let remaining = capacity - wanted;

            TokenBucketRow::insert(
                &tables,
                TokenBucketRow::key_for(namespace_id, identifier),
                &TokenBucketRow {
                    tokens: remaining,
                    last_refill: new_last_refill,
                },
            )?;

            Ok((true, remaining, None))
        })
        .await?
    }

    pub async fn get_remaining<I: AsRef<str> + 'static + Send>(
        &self,
        now: Timestamp,
        namespace_id: NamespaceId,
        identifier: I,
        wanted: u64,
        config: TokenBucket,
    ) -> Result<(u64, Option<DurationMs>)> {
        let tables = self.tables.clone();
        spawn_blocking_in_current_span(move || {
            let identifier = identifier.as_ref();
            let (capacity, _) =
                Self::calculate_capacity(&tables, namespace_id, identifier, &config, now)?;

            if capacity < wanted {
                let retry_after = config.calculate_retry_after(capacity, wanted);
                return Ok((capacity, Some(retry_after)));
            }

            Ok((capacity, None))
        })
        .await?
    }

    pub async fn reset<I: AsRef<str> + 'static + Send>(
        &self,
        namespace_id: NamespaceId,
        identifier: I,
    ) -> Result<()> {
        let tables = self.tables.clone();
        spawn_blocking_in_current_span(move || {
            TokenBucketRow::remove(
                &tables,
                TokenBucketRow::key_for(namespace_id, identifier.as_ref()),
            )
        })
        .await??;
        Ok(())
    }
}

#[allow(clippy::disallowed_methods)]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use diom_core::types::DurationMs;
    use diom_id::NamespaceId;
    use jiff::Timestamp;
    use tempfile::tempdir;

    use super::*;

    fn create_test_controller() -> (RateLimitController, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db = fjall::Database::builder(&dir)
            .temporary(true)
            .open()
            .unwrap();
        (RateLimitController::new(db).unwrap(), dir)
    }

    fn ns() -> NamespaceId {
        NamespaceId::nil()
    }

    fn config() -> TokenBucket {
        TokenBucket {
            refill_rate: 1,
            refill_interval: DurationMs::from_millis(100),
            bucket_size: 5,
        }
    }

    fn config_refill_2() -> TokenBucket {
        TokenBucket {
            refill_rate: 2,
            refill_interval: DurationMs::from_millis(100),
            bucket_size: 5,
        }
    }

    #[tokio::test]
    async fn rate_limiting() {
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        let clock = Timestamp::now();
        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 3, config()).await.unwrap();
        assert!(result);
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 2, config()).await.unwrap();
        assert!(result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) =
            limiter.limit(clock, ns(), id, 1, config()).await.unwrap();
        assert!(!result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, Some(DurationMs::from_millis(100)));
    }

    #[tokio::test]
    async fn tokens_refill_over_time() {
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        let mut clock = Timestamp::now();
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 5, config_refill_2())
            .await
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 0);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(100);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), id, 1, config_refill_2())
            .await
            .unwrap();
        assert_eq!(remaining, 2);
        assert_eq!(retry_after, None);

        clock += Duration::from_millis(200);
        let (remaining, retry_after) = limiter
            .get_remaining(clock, ns(), id, 1, config_refill_2())
            .await
            .unwrap();
        assert_eq!(remaining, 5);
        assert_eq!(retry_after, None);

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 2, config_refill_2())
            .await
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);
    }

    #[tokio::test]
    async fn refill_interval_tracking() {
        let (limiter, _dir) = create_test_controller();
        let id = "user1";

        fn make_config() -> TokenBucket {
            TokenBucket {
                refill_rate: 2,
                refill_interval: DurationMs::from_millis(5000),
                bucket_size: 6,
            }
        }

        let mut clock = Timestamp::now();

        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 2, make_config())
            .await
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 4);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(2);
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 1, make_config())
            .await
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 3);
        assert_eq!(retry_after, None);

        clock += Duration::from_secs(3);
        let (result, remaining, retry_after) = limiter
            .limit(clock, ns(), id, 1, make_config())
            .await
            .unwrap();
        assert!(result);
        assert_eq!(remaining, 4); // 4 (previous) + 2 (refill) - 1 (consumed)
        assert_eq!(retry_after, None);
    }
}
