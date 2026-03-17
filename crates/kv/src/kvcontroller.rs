use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use coyote_error::{Error, Result};
use coyote_namespace::entities::NamespaceId;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::{StorageType, TableRow, WriteBatchExt};
use itertools::Itertools;
use jiff::Timestamp;
use parking_lot::{Mutex, MutexGuard};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tables::{ExpirationRow, KvPairRow};

const EXPIRATION_BATCH_SIZE: usize = 1_000;
const WARN_LONG_LOCK_DURATION: Duration = Duration::from_millis(100);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
    pub expiry: Option<Timestamp>,
    pub value: Vec<u8>,
    /// Opaque version token for optimistic concurrency control.
    pub version: u64,
}

/// Input model for [`KvController::set`]. `version` is the expected current
/// version for OCC — `None` skips the check.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub struct KvModelIn {
    pub value: Vec<u8>,
    pub expiry: Option<Timestamp>,
    pub version: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvSetResult {
    pub version: u64,
    pub success: bool,
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
    db: Arc<Mutex<fjall::Database>>,
    keyspace: fjall::Keyspace,
    keyspace_name: &'static str,
}

impl KvController {
    pub fn new(db: fjall::Database, keyspace_name: &'static str) -> Self {
        let tables = {
            let opts = KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self {
            db: Arc::new(Mutex::new(db)),
            keyspace: tables,
            keyspace_name,
        }
    }

    #[tracing::instrument(skip(self))]
    pub fn fetch(
        &self,
        namespace_id: NamespaceId,
        key: &str,
        now: Timestamp,
    ) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(&self.keyspace, KvPairRow::key_for(namespace_id, key))?
        else {
            return Ok(None);
        };

        if data.expiry.is_some_and(|exp| exp < now) {
            return Ok(None);
        }

        Ok(Some(data.into()))
    }

    fn insert_with_expiration(
        &self,
        db: MutexGuard<'_, fjall::Database>,
        namespace_id: NamespaceId,
        key: &str,
        model: KvModel,
    ) -> Result<()> {
        let mut batch = db.batch();

        let row = KvPairRow {
            value: model.value,
            expiry: model.expiry,
            version: model.version,
        };

        batch.insert_row(&self.keyspace, KvPairRow::key_for(namespace_id, key), &row)?;

        if let Some(expiry) = row.expiry {
            let expiration_row = ExpirationRow::new();
            let key = ExpirationRow::key_for(namespace_id, expiry, key);
            batch.insert_row(&self.keyspace, key, &expiration_row)?;
        }

        batch.commit()?;

        Ok(())
    }

    #[tracing::instrument(skip(self, model))]
    pub fn set(
        &self,
        namespace_id: NamespaceId,
        key: &str,
        model: KvModelIn,
        behavior: OperationBehavior,
        now: Timestamp,
        // This is a monotonically increasing global counter (e.g. raft offset)
        global_counter: u64,
    ) -> Result<KvSetResult> {
        let mut current = None;
        // OCC check: if the caller supplied an expected version, verify it.
        if let Some(expected) = model.version {
            current = self.fetch(namespace_id, key, now)?;
            let current_version = current.as_ref().map(|m| m.version).unwrap_or(0);
            if current_version != expected {
                return Err(Error::bad_request("version_mismatch", "version mismatch"));
            }
        }

        let new_version = global_counter + 1;

        let new_model = KvModel {
            value: model.value,
            expiry: model.expiry,
            version: new_version,
        };

        let db = self.db.lock();

        match behavior {
            OperationBehavior::Upsert => {
                self.insert_with_expiration(db, namespace_id, key, new_model)?;
            }
            OperationBehavior::Insert => {
                current = if current.is_some() {
                    current
                } else {
                    self.fetch(namespace_id, key, now)?
                };

                if let Some(current) = current {
                    return Ok(KvSetResult {
                        version: current.version,
                        success: false,
                    });
                } else {
                    self.insert_with_expiration(db, namespace_id, key, new_model)?;
                }
            }
            OperationBehavior::Update => {
                current = if current.is_some() {
                    current
                } else {
                    self.fetch(namespace_id, key, now)?
                };
                let exists = current.is_some();

                if exists {
                    self.insert_with_expiration(db, namespace_id, key, new_model)?;
                } else {
                    return Ok(KvSetResult {
                        version: 0,
                        success: false,
                    });
                }
            }
        };

        Ok(KvSetResult {
            version: new_version,
            success: true,
        })
    }

    #[tracing::instrument(skip(self))]
    pub fn delete(&self, namespace_id: NamespaceId, key: &str) -> Result<bool> {
        let db = self.db.lock();

        if let Some(data) = KvPairRow::fetch(&self.keyspace, KvPairRow::key_for(namespace_id, key))?
        {
            let mut batch = db.batch();

            // Delete from the expiration keyspace
            if let Some(expiry) = data.expiry {
                batch.remove_row(
                    &self.keyspace,
                    ExpirationRow::key_for(namespace_id, expiry, key),
                )?;
            }
            batch.remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, key))?;

            batch.commit()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::values(&self.keyspace)
    }

    #[tracing::instrument(skip(self))]
    pub async fn has_expired(&self, now: Timestamp) -> bool {
        let keyspace = self.keyspace.clone();

        let start =
            ExpirationRow::key_for(NamespaceId::nil(), Timestamp::UNIX_EPOCH, "").into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        tokio::task::spawn_blocking(move || keyspace.range(start..=end).next().is_some())
            .await
            .inspect_err(|err| tracing::warn!(?err, "unhandled error looking for expired keys"))
            .unwrap_or(false)
    }

    #[tracing::instrument(skip(self), fields(
        keyspace_name = self.keyspace_name,
        cleared,
    ))]
    pub fn clear_expired(
        &self,
        now: Timestamp,
        max_expirations: usize,
        storage_type: StorageType,
    ) -> Result<usize> {
        let start =
            ExpirationRow::key_for(NamespaceId::nil(), Timestamp::UNIX_EPOCH, "").into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        let mut cleared = 0;
        let start_time = Instant::now();

        for chunk in &self
            .keyspace
            .range(start..=end)
            .take(max_expirations)
            .chunks(EXPIRATION_BATCH_SIZE)
        {
            let db = self.db.lock();
            let mut batch = db.batch();
            for item in chunk {
                cleared += 1;
                let k = item.key()?;
                let (namespace_id, main_key) = ExpirationRow::extract_key_from_fjall_key(&k)?;
                batch.remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, main_key))?;

                batch.remove(&self.keyspace, k);
            }
            batch.commit()?;
        }

        if cleared == max_expirations {
            tracing::warn!(cleared, elapsed=?start_time.elapsed(), "expiration loop is not keeping up");
        } else if cleared > 0 {
            tracing::debug!(cleared, "cleared some keys");
        } else {
            tracing::trace!("no expired keys");
        }
        tracing::Span::current().record("cleared", cleared);

        Ok(cleared)
    }

    #[tracing::instrument(skip(self), fields(
        keyspace_name = self.keyspace_name,
        cleared
    ))]
    pub fn clear_expired_in_background(
        &self,
        now: Timestamp,
        storage_type: StorageType,
    ) -> Result<usize> {
        let grace_period = now - jiff::SignedDuration::from_secs(10);
        let start =
            ExpirationRow::key_for(NamespaceId::nil(), Timestamp::UNIX_EPOCH, "").into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), grace_period, "").into_fjall_key();

        let mut cleared = 0;

        loop {
            let mut keys = self
                .keyspace
                .range(start.clone()..=end.clone())
                .take(EXPIRATION_BATCH_SIZE)
                .map(|item| item.key());
            let Some(Ok(first)) = keys.next() else {
                tracing::trace!("nothing to clean up");
                break;
            };
            let Some(Ok(last)) = keys.last() else {
                break;
            };

            tracing::trace!(first_key=?first, last_key=?last, "about to prune some expired keys");

            let start_batch = Instant::now();
            let num_this_batch = tracing::debug_span!("clear_expired_in_background:remove_chunk")
                .in_scope(|| {
                let db = self.db.lock();
                let start_lock = Instant::now();
                let mut batch = db.batch();
                let mut num_this_batch = 0;

                for item in self.keyspace.range(first..=last) {
                    cleared += 1;
                    num_this_batch += 1;
                    let k = item.key()?;
                    let (namespace_id, main_key) = ExpirationRow::extract_key_from_fjall_key(&k)?;
                    batch.remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, main_key))?;

                    batch.remove(&self.keyspace, k);
                }
                batch.commit()?;
                drop(db);
                let duration = start_lock.elapsed();
                if duration > WARN_LONG_LOCK_DURATION {
                    tracing::warn!(
                        lock_us = duration.as_micros(),
                        "clear_expired_in_background locked kvcontroller for a long time"
                    );
                }
                Ok::<_, Error>(num_this_batch)
            })?;
            tracing::trace!(num_this_batch, elapsed=?start_batch.elapsed(), "cleared a batch of items");

            if num_this_batch < EXPIRATION_BATCH_SIZE {
                break;
            }
        }

        if cleared > 0 {
            tracing::debug!(cleared, "cleared some keys");
        } else {
            tracing::trace!("no expired keys");
        }
        tracing::Span::current().record("cleared", cleared);

        Ok(cleared)
    }
}

#[cfg(test)]
mod tests {
    use jiff::ToSpan;

    use super::*;
    use coyote_namespace::entities::NamespaceId;

    struct SetupFixture {
        _workdir: tempfile::TempDir,
        controller: KvController,
    }

    impl SetupFixture {
        fn new() -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let db = fjall::Database::builder(workdir.as_ref())
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

    fn key_exists_as_of(controller: &KvController, key: &str, now: Timestamp) -> bool {
        controller.fetch(ns(), key, now).unwrap().is_some()
    }

    fn key_exists(controller: &KvController, key: &str) -> bool {
        controller
            .fetch(ns(), key, Timestamp::now())
            .unwrap()
            .is_some()
    }

    #[test]
    fn test_insert_and_get() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let key = "test:key1";
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"hello world".to_vec(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                Timestamp::now(),
                0,
            )
            .unwrap();

        assert!(key_exists(&controller, "test:key1"));
        let retrieved = controller
            .fetch(ns(), "test:key1", Timestamp::now())
            .unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expiry, None);

        assert!(!key_exists(&controller, "nonexistent:key"));
    }

    #[test]
    fn test_insert_behaviors() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        // Update on non-existent key returns false
        let res = controller.set(
            ns(),
            "key1",
            KvModelIn {
                value: b"key1 updated".to_vec(),
                expiry: None,
                version: None,
            },
            OperationBehavior::Update,
            Timestamp::now(),
            0,
        );
        assert!(!res.unwrap().success);
        assert!(!key_exists(&controller, "key1"));

        let res = controller.set(
            ns(),
            "key1",
            KvModelIn {
                value: b"key1 inserted".to_vec(),
                expiry: None,
                version: None,
            },
            OperationBehavior::Insert,
            Timestamp::now(),
            0,
        );
        assert!(res.unwrap().success);
        assert!(key_exists(&controller, "key1"));
        let result = controller.fetch(ns(), "key1", Timestamp::now()).unwrap();
        assert!(result.is_some());

        // Insert on existing key returns false
        let res = controller.set(
            ns(),
            "key1",
            KvModelIn {
                value: b"another value".to_vec(),
                expiry: None,
                version: None,
            },
            OperationBehavior::Insert,
            Timestamp::now(),
            0,
        );
        assert!(!res.unwrap().success);
        assert!(key_exists(&controller, "key1"));
        let result = controller.fetch(ns(), "key1", Timestamp::now()).unwrap();
        assert!(result.is_some());

        assert_eq!(result.unwrap().value, b"key1 inserted");

        let res = controller.set(
            ns(),
            "key1",
            KvModelIn {
                value: b"key1 upserted".to_vec(),
                expiry: None,
                version: None,
            },
            OperationBehavior::Upsert,
            Timestamp::now(),
            0,
        );
        assert!(res.unwrap().success);
        assert!(key_exists(&controller, "key1"));
        let result = controller.fetch(ns(), "key1", Timestamp::now()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, b"key1 upserted");
    }

    #[test]
    fn test_overwrite() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let key = "overwrite:key";
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"first value".to_vec(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                Timestamp::now(),
                0,
            )
            .unwrap();
        controller
            .set(
                ns(),
                key,
                KvModelIn {
                    value: b"second value".to_vec(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                Timestamp::now(),
                0,
            )
            .unwrap();

        assert!(key_exists(&controller, "overwrite:key"));
        let retrieved = controller
            .fetch(ns(), "overwrite:key", Timestamp::now())
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.value, b"second value");
    }

    #[test]
    fn test_clear_expired_removes_expired_entries() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let now = Timestamp::now();

        controller
            .set(
                ns(),
                "expired:key",
                KvModelIn {
                    value: b"expired data".to_vec(),
                    expiry: Some(now.checked_sub(1.hour()).unwrap()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
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
                now.checked_sub(1.second()).unwrap(),
                b"expired data 3".as_slice(),
            ),
        ];

        for (key, expiry, value) in &expired_models {
            controller
                .set(
                    ns(),
                    key,
                    KvModelIn {
                        value: value.to_vec(),
                        expiry: Some(*expiry),
                        version: None,
                    },
                    OperationBehavior::Upsert,
                    now,
                    0,
                )
                .unwrap();
        }

        let valid_key = "valid:key";
        controller
            .set(
                ns(),
                valid_key,
                KvModelIn {
                    value: b"valid data".to_vec(),
                    expiry: Some(now.checked_add(1.hour()).unwrap()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .unwrap();

        let permanent_key = "permanent:key";
        controller
            .set(
                ns(),
                permanent_key,
                KvModelIn {
                    value: b"permanent data".to_vec(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .unwrap();

        assert_eq!(controller.iter().unwrap().count(), 6);

        // the key should have expired by now, so key_exists should already be false
        assert!(!key_exists(&controller, "expired:key"));
        let then = Timestamp::now().checked_sub(6.hours()).unwrap();
        // but if we time travel to the past, it should still be there
        assert!(key_exists_as_of(&controller, "expired:key", then));
        for (key, _, _) in &expired_models {
            assert!(key_exists_as_of(&controller, key, then));
        }

        assert_eq!(
            controller
                .clear_expired(Timestamp::now(), 100, StorageType::Persistent)
                .unwrap(),
            4
        );

        for (key, _, _) in &expired_models {
            // now it should really and truly be gone
            assert!(!key_exists(&controller, key));
            assert!(!key_exists_as_of(&controller, key, then));
        }
        assert!(!key_exists(&controller, "expired:key"));
        assert!(!key_exists_as_of(&controller, "expired:key", then));

        assert!(key_exists(&controller, valid_key));
        let valid = controller.fetch(ns(), valid_key, Timestamp::now()).unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        assert!(key_exists(&controller, permanent_key));
        let permanent = controller
            .fetch(ns(), permanent_key, Timestamp::now())
            .unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }

    #[test]
    fn test_clear_expired_in_background_removes_expired_entries() {
        let setup = SetupFixture::new();
        let controller = setup.controller;

        let now = Timestamp::now();

        controller
            .set(
                ns(),
                "expired:key",
                KvModelIn {
                    value: b"expired data".to_vec(),
                    expiry: Some(now.checked_sub(1.hour()).unwrap()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
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

        for (key, expiry, value) in &expired_models {
            controller
                .set(
                    ns(),
                    key,
                    KvModelIn {
                        value: value.to_vec(),
                        expiry: Some(*expiry),
                        version: None,
                    },
                    OperationBehavior::Upsert,
                    now,
                    0,
                )
                .unwrap();
        }

        let valid_key = "valid:key";
        controller
            .set(
                ns(),
                valid_key,
                KvModelIn {
                    value: b"valid data".to_vec(),
                    expiry: Some(now.checked_add(1.hour()).unwrap()),
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .unwrap();

        let permanent_key = "permanent:key";
        controller
            .set(
                ns(),
                permanent_key,
                KvModelIn {
                    value: b"permanent data".to_vec(),
                    expiry: None,
                    version: None,
                },
                OperationBehavior::Upsert,
                now,
                0,
            )
            .unwrap();

        assert_eq!(controller.iter().unwrap().count(), 7);

        // the key should have expired by now, so key_exists should already be false
        assert!(!key_exists(&controller, "expired:key"));
        let then = Timestamp::now().checked_sub(6.hours()).unwrap();
        // but if we time travel to the past, it should still be there
        assert!(key_exists_as_of(&controller, "expired:key", then));
        for (key, _, _) in &expired_models {
            assert!(key_exists_as_of(&controller, key, then));
        }

        assert_eq!(
            controller
                .clear_expired_in_background(Timestamp::now(), StorageType::Persistent)
                .unwrap(),
            4
        );

        for (key, _, _) in &expired_models {
            // now it should really and truly be gone
            assert!(!key_exists(&controller, key));
            if *key == "expired:key:4" {
                // this one is in the grace period
                assert!(key_exists_as_of(&controller, key, then));
            } else {
                assert!(!key_exists_as_of(&controller, key, then));
            }
        }
        assert!(!key_exists(&controller, "expired:key"));
        assert!(!key_exists_as_of(&controller, "expired:key", then));

        assert!(key_exists(&controller, valid_key));
        let valid = controller.fetch(ns(), valid_key, Timestamp::now()).unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        assert!(key_exists(&controller, permanent_key));
        let permanent = controller
            .fetch(ns(), permanent_key, Timestamp::now())
            .unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }
}
