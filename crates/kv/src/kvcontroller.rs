use std::time::Instant;

use diom_core::{
    PersistableValue,
    task::spawn_blocking_in_current_span,
    types::{ByteString, DurationMs, UnixTimestampMs},
};
use diom_error::{Error, Result};
use diom_id::NamespaceId;
use fjall::{Database, Keyspace};
use fjall_utils::{SerializableKeyspaceCreateOptions, TableRow, WriteBatchExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tables::{ExpirationRow, KvPairRow};

const EXPIRATION_BATCH_SIZE: usize = 10_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, PersistableValue)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OperationBehavior {
    #[default]
    Upsert,
    Insert,
    Update,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub struct KvModel {
    pub expiry: Option<UnixTimestampMs>,
    pub value: ByteString,
    /// Opaque version token for optimistic concurrency control.
    pub version: u64,
}

/// Input model for [`KvController::set`]. `version` is the expected current
/// version for OCC — `None` skips the check.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub struct KvModelIn {
    pub value: ByteString,
    pub expiry: Option<UnixTimestampMs>,
    pub version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvSetResult {
    pub version: u64,
}

impl From<KvPairRow> for KvModel {
    fn from(row: KvPairRow) -> Self {
        Self {
            expiry: row.expiry,
            value: row.value,
            version: row.version,
        }
    }
}

#[derive(Clone)]
pub struct KvController {
    db: Database,
    keyspace: Keyspace,
    keyspace_name: &'static str,
}

impl KvController {
    pub fn new(db: Database, keyspace_name: &'static str) -> Self {
        let tables = SerializableKeyspaceCreateOptions::default()
            .with_default_kv_separation()
            .create_and_record(&db, keyspace_name)
            .expect("should be able to open keyspace");

        Self {
            db,
            keyspace: tables,
            keyspace_name,
        }
    }

    #[tracing::instrument(skip_all)]
    fn fetch_inner(
        keyspace: &Keyspace,
        namespace_id: NamespaceId,
        key: &str,
        now: UnixTimestampMs,
    ) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(keyspace, KvPairRow::key_for(namespace_id, key))? else {
            return Ok(None);
        };

        if data.expiry.is_some_and(|exp| exp < now) {
            return Ok(None);
        }

        Ok(Some(data.into()))
    }

    #[tracing::instrument(skip_all)]
    pub async fn fetch<
        K: AsRef<str> + std::fmt::Debug + 'static + Send,
        TS: Into<UnixTimestampMs>,
    >(
        &self,
        namespace_id: NamespaceId,
        key: K,
        now: TS,
    ) -> Result<Option<KvModel>> {
        let keyspace = self.keyspace.clone();
        let now = now.into();
        spawn_blocking_in_current_span(move || {
            Self::fetch_inner(&keyspace, namespace_id, key.as_ref(), now)
        })
        .await?
    }

    fn insert_with_expiration(
        db: &Database,
        keyspace: &Keyspace,
        namespace_id: NamespaceId,
        key: &str,
        model: KvModel,
        old_expiry: Option<UnixTimestampMs>,
    ) -> Result<()> {
        let mut batch = db.batch();

        let row = KvPairRow {
            value: model.value,
            expiry: model.expiry,
            version: model.version,
        };

        batch.insert_row(keyspace, KvPairRow::key_for(namespace_id, key), &row)?;

        if let Some(ts) = old_expiry {
            let key = ExpirationRow::key_for(namespace_id, ts, key);
            batch.remove_row(keyspace, key)?;
        }

        if let Some(expiry) = row.expiry {
            let expiration_row = ExpirationRow::new();
            let key = ExpirationRow::key_for(namespace_id, expiry, key);
            batch.insert_row(keyspace, key, &expiration_row)?;
        }

        batch.commit()?;

        Ok(())
    }

    #[tracing::instrument(skip_all, fields(?behavior))]
    pub async fn set<
        K: AsRef<str> + std::fmt::Debug + 'static + Send,
        TS: Into<UnixTimestampMs>,
    >(
        &self,
        namespace_id: NamespaceId,
        key: K,
        model: KvModelIn,
        behavior: OperationBehavior,
        now: TS,
        // This is a monotonically increasing global counter (e.g. raft offset)
        global_counter: u64,
    ) -> Result<KvSetResult> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();
        let now = now.into();

        spawn_blocking_in_current_span(move || {
            let key = key.as_ref();
            let current = Self::fetch_inner(&keyspace, namespace_id, key, now)?;
            // OCC check: if the caller supplied an expected version, verify it.
            if let Some(expected) = model.version {
                let current_version = current.as_ref().map(|m| m.version).unwrap_or(0);
                if current_version != expected {
                    return Err(Error::conflict("version mismatch", None));
                }
            }

            let new_version = global_counter + 1;

            let new_model = KvModel {
                value: model.value,
                expiry: model.expiry,
                version: new_version,
            };

            match behavior {
                OperationBehavior::Upsert => {
                    Self::insert_with_expiration(
                        &db,
                        &keyspace,
                        namespace_id,
                        key,
                        new_model,
                        current.and_then(|c| c.expiry),
                    )?;
                }
                OperationBehavior::Insert => {
                    if current.is_some() {
                        return Err(Error::conflict("key already exists", None));
                    } else {
                        Self::insert_with_expiration(
                            &db,
                            &keyspace,
                            namespace_id,
                            key,
                            new_model,
                            current.and_then(|c| c.expiry),
                        )?;
                    }
                }
                OperationBehavior::Update => {
                    if current.is_some() {
                        Self::insert_with_expiration(
                            &db,
                            &keyspace,
                            namespace_id,
                            key,
                            new_model,
                            current.and_then(|c| c.expiry),
                        )?;
                    } else {
                        return Err(Error::conflict("key not found", None));
                    }
                }
            };

            Ok(KvSetResult {
                version: new_version,
            })
        })
        .await?
    }

    #[tracing::instrument(skip_all)]
    pub async fn delete<T: AsRef<str> + std::fmt::Debug + 'static + Send>(
        &self,
        namespace_id: NamespaceId,
        key: T,
        version: Option<u64>,
        now: UnixTimestampMs,
    ) -> Result<bool> {
        let db = self.db.clone();
        let keyspace = self.keyspace.clone();

        spawn_blocking_in_current_span(move || {
            let key = key.as_ref();
            let current = Self::fetch_inner(&keyspace, namespace_id, key, now)?;

            // OCC check: if the caller supplied an expected version, verify it.
            if let Some(expected) = version {
                let current_version = current.as_ref().map(|m| m.version).unwrap_or(0);
                if current_version != expected {
                    return Err(Error::conflict("version mismatch", None));
                }
            }

            let Some(current) = current else {
                return Ok(false);
            };
            let mut batch = db.batch();

            if let Some(expiry) = current.expiry {
                batch.remove_row(&keyspace, ExpirationRow::key_for(namespace_id, expiry, key))?;
            }
            batch.remove_row(&keyspace, KvPairRow::key_for(namespace_id, key))?;

            batch.commit()?;
            Ok(true)
        })
        .await?
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::values(&self.keyspace)
    }

    #[tracing::instrument(skip_all)]
    pub async fn has_expired(&self, now: UnixTimestampMs) -> bool {
        let keyspace = self.keyspace.clone();

        let start = ExpirationRow::key_for(NamespaceId::nil(), UnixTimestampMs::UNIX_EPOCH, "")
            .into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        spawn_blocking_in_current_span(move || keyspace.range(start..=end).next().is_some())
            .await
            .inspect_err(|err| tracing::warn!(?err, "unhandled error looking for expired keys"))
            .unwrap_or(false)
    }

    /// Keep calling keep calling the given asynchronous thunk until we have no more entries
    /// that expire before the given time. This is meant to be used from the leader callback on
    /// things that use KvController internally, and should not be called anywhere else.
    #[doc(hidden)]
    pub async fn clear_expired_in_raft_until_done<F, O>(
        &self,
        now: UnixTimestampMs,
        thunk: F,
    ) -> diom_operations::BackgroundResult<()>
    where
        F: AsyncFn() -> diom_operations::BackgroundResult<O>,
    {
        const WARN_RUNNING_BEHIND_DURATION: DurationMs = DurationMs::from_secs(5);

        let start = Instant::now();
        while self.has_expired(now).await {
            if start.elapsed() > WARN_RUNNING_BEHIND_DURATION {
                tracing::warn!(
                    keyspace = self.keyspace_name,
                    elapsed = ?start.elapsed(),
                    "clear_expired is running significantly behind"
                );
            }
            tracing::trace!(timestamp=%now, "scheduling a job to clear items expired before");
            thunk().await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip_all, fields(
        keyspace_name = self.keyspace_name,
        cleared
    ))]
    pub async fn clear_expired_in_raft(&self, now: UnixTimestampMs) -> Result<usize> {
        let start = ExpirationRow::key_for(NamespaceId::nil(), UnixTimestampMs::UNIX_EPOCH, "")
            .into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        let keyspace = self.keyspace.clone();
        let db = self.db.clone();

        let cleared = spawn_blocking_in_current_span(move || -> Result<usize> {
            let mut cleared = 0;
            let mut keys = keyspace
                .range(start.clone()..=end.clone())
                .take(EXPIRATION_BATCH_SIZE)
                .map(|item| item.key());
            let Some(Ok(first)) = keys.next() else {
                tracing::trace!("nothing to clean up");
                return Ok(cleared);
            };
            let Some(Ok(last)) = keys.last() else {
                return Ok(cleared);
            };

            tracing::trace!(first_key=?first, last_key=?last, "about to prune some expired keys");

            let start_batch = Instant::now();
            let num_this_batch =
                tracing::debug_span!("clear_expired_in_raft:remove_chunk").in_scope(|| {
                    let mut batch = db.batch();
                    let mut num_this_batch = 0;

                    for item in keyspace.range(first..=last) {
                        cleared += 1;
                        num_this_batch += 1;
                        let k = item.key()?;
                        let (namespace_id, main_key) = ExpirationRow::extract_key_from_fjall_key(&k)?;
                        batch.remove_row(&keyspace, KvPairRow::key_for(namespace_id, main_key))?;

                        batch.remove(&keyspace, k);
                    }
                    batch.commit()?;
                    Ok::<_, Error>(num_this_batch)
                })?;
            tracing::trace!(num_this_batch, elapsed=?start_batch.elapsed(), "cleared a batch of items");

            if cleared > 0 {
                tracing::debug!(cleared, "cleared some keys");
            } else {
                tracing::trace!("no expired keys");
            }
            Ok(cleared)
        }).await??;

        tracing::Span::current().record("cleared", cleared);

        Ok(cleared)
    }
}

#[allow(clippy::disallowed_methods)]
#[cfg(test)]
mod tests {
    use diom_id::NamespaceId;
    use jiff::{SignedDuration, ToSpan};

    use super::*;

    fn now() -> jiff::Timestamp {
        jiff::Timestamp::now()
    }

    struct SetupFixture {
        _workdir: tempfile::TempDir,
        controller: KvController,
    }

    impl SetupFixture {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let db = Database::builder(workdir.as_ref())
                .temporary(true)
                .open()
                .unwrap();
            let controller = KvController::new(db, "mod_kv_test");
            Self {
                _workdir: workdir,
                controller,
            }
        }
    }

    fn ns() -> NamespaceId {
        NamespaceId::nil()
    }

    async fn key_exists_as_of<TS: Into<UnixTimestampMs>>(
        controller: &KvController,
        key: &str,
        now: TS,
    ) -> bool {
        let key = key.to_string();
        controller.fetch(ns(), key, now).await.unwrap().is_some()
    }

    async fn key_exists(controller: &KvController, key: &str) -> bool {
        let key = key.to_string();
        controller.fetch(ns(), key, now()).await.unwrap().is_some()
    }

    #[tokio::test]
    async fn test_insert_and_get() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let key = "test:key1";
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"hello world".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await
            .unwrap();

        assert!(key_exists(&controller, "test:key1").await);
        let retrieved = controller.fetch(ns(), "test:key1", now()).await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expiry, None);

        assert!(!key_exists(&controller, "nonexistent:key").await);
    }

    #[tokio::test]
    async fn test_insert_behaviors() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        // Update on non-existent key returns an error
        let res = controller
            .set(
                ns(),
                "key1",
                KvModelIn {
                    value: b"key1 updated".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Update,
                now(),
                0,
            )
            .await;
        assert!(res.is_err());
        assert!(!key_exists(&controller, "key1").await);

        let res = controller
            .set(
                ns(),
                "key1",
                KvModelIn {
                    value: b"key1 inserted".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Insert,
                now(),
                0,
            )
            .await;
        assert!(res.is_ok());
        assert!(key_exists(&controller, "key1").await);
        let result = controller.fetch(ns(), "key1", now()).await.unwrap();
        assert!(result.is_some());

        // Insert on existing key returns an error
        let res = controller
            .set(
                ns(),
                "key1",
                KvModelIn {
                    value: b"another value".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Insert,
                now(),
                0,
            )
            .await;
        assert!(res.is_err());
        assert!(key_exists(&controller, "key1").await);
        let result = controller.fetch(ns(), "key1", now()).await.unwrap();
        assert!(result.is_some());

        assert_eq!(result.unwrap().value, b"key1 inserted");

        let res = controller
            .set(
                ns(),
                "key1",
                KvModelIn {
                    value: b"key1 upserted".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await;
        assert!(res.is_ok());
        assert!(key_exists(&controller, "key1").await);
        let result = controller.fetch(ns(), "key1", now()).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, b"key1 upserted");
    }

    #[tokio::test]
    async fn test_overwrite() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let key = "overwrite:key";
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"first value".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await
            .unwrap();
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"second value".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await
            .unwrap();

        assert!(key_exists(&controller, "overwrite:key").await);
        let retrieved = controller
            .fetch(ns(), "overwrite:key", now())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.value, b"second value");
    }

    #[tokio::test]
    async fn test_clear_expired_in_raft_removes_expired_entries() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let now = now();

        controller
            .set(
                ns(),
                "expired:key",
                KvModelIn {
                    value: b"expired data".into(),
                    expiry: Some(now.checked_sub(1.hour()).unwrap().into()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .await
            .unwrap();

        let expired_models = [
            (
                "expired:key:1",
                now.checked_sub(3.hour()).unwrap(),
                b"expired data 1".as_slice(),
            ),
            (
                "expired:key:2",
                now.checked_sub(2.hour()).unwrap(),
                b"expired data 2".as_slice(),
            ),
            (
                "expired:key:3",
                now.checked_sub(11.second()).unwrap(),
                b"expired data 3".as_slice(),
            ),
            (
                "expired:key:4",
                now.checked_sub(1.second()).unwrap(),
                b"expired data that is in the grace period".as_slice(),
            ),
        ];

        for (key, expiry, value) in expired_models {
            controller
                .set(
                    ns(),
                    key.to_string(),
                    KvModelIn {
                        value: value.into(),
                        expiry: Some(expiry.into()),
                        version: None,
                    },
                    OperationBehavior::Upsert,
                    now,
                    0,
                )
                .await
                .unwrap();
        }

        let valid_key = "valid:key";
        controller
            .set(
                ns(),
                valid_key,
                KvModelIn {
                    value: b"valid data".into(),
                    expiry: Some(now.checked_add(1.hour()).unwrap().into()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .await
            .unwrap();

        let permanent_key = "permanent:key";
        controller
            .set(
                ns(),
                permanent_key,
                KvModelIn {
                    value: b"permanent data".into(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .await
            .unwrap();

        assert_eq!(controller.iter().unwrap().count(), 7);

        // the key should have expired by now, so key_exists should already be false
        assert!(!key_exists(&controller, "expired:key").await);
        let then = now.checked_sub(6.hours()).unwrap();
        // but if we time travel to the past, it should still be there
        assert!(key_exists_as_of(&controller, "expired:key", then).await);
        for (key, _, _) in &expired_models {
            assert!(key_exists_as_of(&controller, key, then).await);
        }

        assert_eq!(
            controller.clear_expired_in_raft(now.into()).await.unwrap(),
            5
        );

        for (key, _, _) in &expired_models {
            // now it should really and truly be gone
            assert!(!key_exists(&controller, key).await);
            assert!(!key_exists_as_of(&controller, key, then).await);
        }
        assert!(!key_exists(&controller, "expired:key").await);
        assert!(!key_exists_as_of(&controller, "expired:key", then).await);

        assert!(key_exists(&controller, valid_key).await);
        let valid = controller.fetch(ns(), valid_key, now).await.unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        assert!(key_exists(&controller, permanent_key).await);
        let permanent = controller.fetch(ns(), permanent_key, now).await.unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }

    #[tokio::test]
    async fn test_bumping_expiration_deletes_stale_rows() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let ms = jiff::TimestampRound::new()
            .smallest(jiff::Unit::Millisecond)
            .mode(jiff::RoundMode::Trunc);

        let expiry = (now() + SignedDuration::from_secs(10)).round(ms).unwrap();

        let key = "overwrite:key";
        let ns = ns();
        controller
            .set(
                ns,
                key,
                KvModelIn {
                    value: b"first value".into(),
                    expiry: Some(expiry.into()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await
            .unwrap();

        let expiration_rows = ExpirationRow::keys(&controller.keyspace)
            .unwrap()
            .collect::<Vec<fjall::UserKey>>();
        assert_eq!(expiration_rows.len(), 1);
        let ts = ExpirationRow::extract_ts_from_fjall_key(&expiration_rows[0]);
        let (namespace_id, found_key) =
            ExpirationRow::extract_key_from_fjall_key(&expiration_rows[0]).unwrap();
        assert_eq!(ts, expiry.into());
        assert_eq!(namespace_id, ns);
        assert_eq!(found_key, key);

        let later_expiry = (now() + SignedDuration::from_secs(90)).round(ms).unwrap();
        controller
            .set(
                ns,
                key,
                KvModelIn {
                    value: b"first value".into(),
                    expiry: Some(later_expiry.into()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now(),
                0,
            )
            .await
            .unwrap();

        let expiration_rows = ExpirationRow::keys(&controller.keyspace)
            .unwrap()
            .collect::<Vec<fjall::UserKey>>();
        // the old index row should be deleted
        assert_eq!(expiration_rows.len(), 1);
        let ts = ExpirationRow::extract_ts_from_fjall_key(&expiration_rows[0]);
        let (namespace_id, found_key) =
            ExpirationRow::extract_key_from_fjall_key(&expiration_rows[0]).unwrap();
        // and the ts should be updated
        assert_eq!(ts, later_expiry.into());
        assert_eq!(namespace_id, ns);
        assert_eq!(found_key, key);
    }
}
