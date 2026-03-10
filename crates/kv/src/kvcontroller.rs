use std::sync::Arc;

use coyote_core::sync::GcFence;
use coyote_error::Result;
use coyote_namespace::entities::NamespaceId;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::{TableRow, WriteBatchExt};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tables::{ExpirationRow, KvPairRow};

const EXPIRATION_BATCH_SIZE: usize = 1_000; // FIXME(@svix-lucho): make this configurable? Probably
// much larger too?

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
}

impl From<KvPairRow> for KvModel {
    fn from(row: KvPairRow) -> Self {
        Self {
            expiry: row.expiry,
            value: row.value,
        }
    }
}

#[derive(Clone)]
pub struct KvController {
    db: fjall::Database,
    keyspace: fjall::Keyspace,
    keyspace_name: &'static str,
    kv_fence: Arc<GcFence<String>>,
}

impl KvController {
    pub fn new(db: fjall::Database, keyspace_name: &'static str) -> Self {
        let tables = {
            let opts = KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self {
            db,
            keyspace: tables,
            keyspace_name,
            kv_fence: Arc::new(GcFence::new()),
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
        namespace_id: NamespaceId,
        key: &str,
        value: Vec<u8>,
        expiry: Option<Timestamp>,
    ) -> Result<()> {
        let mut batch = self.db.batch();

        let _guard = self.kv_fence.want_to_write(key);

        let row = KvPairRow { value, expiry };

        batch.insert_row(&self.keyspace, KvPairRow::key_for(namespace_id, key), &row)?;

        if let Some(expiry) = expiry {
            let expiration_row = ExpirationRow::new();
            let key = ExpirationRow::key_for(namespace_id, expiry, key);
            batch.insert_row(&self.keyspace, key, &expiration_row)?;
        }

        batch.commit()?;

        Ok(())
    }

    #[tracing::instrument(skip(self, value))]
    pub fn set(
        &self,
        namespace_id: NamespaceId,
        key: &str,
        value: Vec<u8>,
        expiry: Option<Timestamp>,
        behavior: OperationBehavior,
        now: Timestamp,
    ) -> Result<()> {
        match behavior {
            OperationBehavior::Upsert => {
                self.insert_with_expiration(namespace_id, key, value, expiry)?;
            }
            OperationBehavior::Insert => {
                let exists = self.fetch(namespace_id, key, now)?.is_some();

                if !exists {
                    self.insert_with_expiration(namespace_id, key, value, expiry)?;
                } else {
                    // FIXME(@svix-lucho): Do nothing?
                }
            }
            OperationBehavior::Update => {
                let exists = self.fetch(namespace_id, key, now)?.is_some();
                if exists {
                    self.insert_with_expiration(namespace_id, key, value, expiry)?;
                } else {
                    // FIXME(@svix-lucho): Do nothing?
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn delete(&self, namespace_id: NamespaceId, key: &str) -> Result<()> {
        let mut batch = self.db.batch();

        let _guard = self.kv_fence.want_to_write(key);

        if let Some(data) = KvPairRow::fetch(&self.keyspace, KvPairRow::key_for(namespace_id, key))?
        {
            // Delete from the expiration keyspace
            if let Some(expiry) = data.expiry {
                batch.remove_row(
                    &self.keyspace,
                    ExpirationRow::key_for(namespace_id, expiry, key),
                )?;
            }
            batch.remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, key))?;
        }

        batch.commit()?;

        Ok(())
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::values(&self.keyspace)
    }

    #[tracing::instrument(skip(self), fields(keyspace))]
    pub fn clear_expired(&self, now: Timestamp) -> Result<usize> {
        tracing::Span::current().record("keyspace", self.keyspace_name);

        let mut start =
            ExpirationRow::key_for(NamespaceId::nil(), Timestamp::UNIX_EPOCH, "").into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        let mut cleared = 0;
        let mut conflicts = 0;

        tracing::debug!(%now, "starting background clear for expired data");

        let marker = self.kv_fence.start_marking();

        loop {
            // mark

            // make sure we don't reuse an iterator from a previous loop because it might
            // have been invalidated by concurrent writes.
            tracing::trace!(?start, ?end, "scanning for expiry");
            let chunk = self
                .keyspace
                .range(start..=end.clone())
                .take(EXPIRATION_BATCH_SIZE)
                .map(|item| item.key())
                .collect::<Result<Vec<_>, _>>()?;

            let mut to_destroy = Vec::new();

            if let Some(last) = chunk.last() {
                start = last.clone();
            } else {
                break;
            }

            for k in chunk {
                let (namespace_id, main_key) = ExpirationRow::extract_key_from_fjall_key(&k)?;
                to_destroy.push((namespace_id, main_key.to_owned(), k.to_vec()));
            }

            marker.intent_to_gc(to_destroy.iter().map(|s| &s.1));

            // sweep

            let mut batch = self.db.batch();

            let valid = marker.drain_all();

            for (namespace_id, main_key, k) in to_destroy {
                if valid.contains(&main_key) {
                    cleared += 1;
                    batch
                        .remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, &main_key))?;
                    batch.remove(&self.keyspace, k);
                } else {
                    conflicts += 1;
                    tracing::trace!(?namespace_id, ?main_key, "something else touched this row");
                }
            }
            batch.commit()?;
        }

        tracing::debug!(cleared, conflicts, "finished clearing keys");

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
                b"hello world".to_vec(),
                None,
                OperationBehavior::Upsert,
                Timestamp::now(),
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

        let res = controller.set(
            ns(),
            "key1",
            b"key1 updated".to_vec(),
            None,
            OperationBehavior::Update,
            Timestamp::now(),
        );
        assert!(res.is_ok());
        assert!(!key_exists(&controller, "key1"));

        let res = controller.set(
            ns(),
            "key1",
            b"key1 inserted".to_vec(),
            None,
            OperationBehavior::Insert,
            Timestamp::now(),
        );
        assert!(res.is_ok());
        assert!(key_exists(&controller, "key1"));
        let result = controller.fetch(ns(), "key1", Timestamp::now()).unwrap();
        assert!(result.is_some());

        let res = controller.set(
            ns(),
            "key1",
            b"another value".to_vec(),
            None,
            OperationBehavior::Insert,
            Timestamp::now(),
        );
        assert!(res.is_ok());
        assert!(key_exists(&controller, "key1"));
        let result = controller.fetch(ns(), "key1", Timestamp::now()).unwrap();
        assert!(result.is_some());

        assert_eq!(result.unwrap().value, b"key1 inserted");

        let res = controller.set(
            ns(),
            "key1",
            b"key1 upserted".to_vec(),
            None,
            OperationBehavior::Upsert,
            Timestamp::now(),
        );
        assert!(res.is_ok());
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
                b"first value".to_vec(),
                None,
                OperationBehavior::Upsert,
                Timestamp::now(),
            )
            .unwrap();
        controller
            .set(
                ns(),
                key,
                b"second value".to_vec(),
                None,
                OperationBehavior::Upsert,
                Timestamp::now(),
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
                b"expired data".to_vec(),
                Some(now.checked_sub(1.hour()).unwrap()),
                OperationBehavior::Upsert,
                now,
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
                    value.to_vec(),
                    Some(*expiry),
                    OperationBehavior::Upsert,
                    now,
                )
                .unwrap();
        }

        let valid_key = "valid:key";
        controller
            .set(
                ns(),
                valid_key,
                b"valid data".to_vec(),
                Some(now.checked_add(1.hour()).unwrap()),
                OperationBehavior::Upsert,
                now,
            )
            .unwrap();

        let permanent_key = "permanent:key";
        controller
            .set(
                ns(),
                permanent_key,
                b"permanent data".to_vec(),
                None,
                OperationBehavior::Upsert,
                now,
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

        assert_eq!(controller.clear_expired(Timestamp::now()).unwrap(), 4);

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
}
