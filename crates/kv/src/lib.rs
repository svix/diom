pub mod tables;

use std::num::NonZeroU64;

use coyote_configgroup::{
    ConfigGroup,
    entities::{CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig, ModuleConfig},
};
use coyote_error::Result;
use fjall::{Database, KeyspaceCreateOptions};
use fjall_utils::{TableRow, WriteBatchExt};
use hashlink::{LinkedHashMap, linked_hash_map::RawEntryMut};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod operations;

use crate::{
    operations::SetOperation,
    tables::{ExpirationRow, KvPairRow},
};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Clone)]
pub struct KvModel {
    pub expires_at: Option<Timestamp>,
    pub value: Vec<u8>,
}

impl From<KvPairRow> for KvModel {
    fn from(row: KvPairRow) -> Self {
        Self {
            expires_at: row.expires_at,
            value: row.value,
        }
    }
}
#[derive(Clone)]
pub struct KvStore {
    db: fjall::Database,
    tables: fjall::Keyspace,
    policy: EvictionPolicy,
    lru: LinkedHashMap<String, Option<Timestamp>>,
    max_storage_bytes: Option<NonZeroU64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum OperationBehavior {
    #[default]
    Upsert,
    Insert,
    Update,
}

const EXPIRATION_BATCH_SIZE: usize = 100; // FIXME(@svix-lucho): make this configurable?

impl KvStore {
    pub fn new(
        namespace: &str,
        db: Database,
        policy: EvictionPolicy,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        let kv_keyspace = format!("_coyote_kv_{namespace}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&kv_keyspace, || opts).unwrap()
        };

        Self {
            db,
            tables,
            policy,
            lru: LinkedHashMap::new(),
            max_storage_bytes,
        }
    }

    fn update_lru(&mut self, key: &str, expires_at: Option<Timestamp>) {
        match self.lru.raw_entry_mut().from_key(key) {
            RawEntryMut::Occupied(mut occupied) => {
                occupied.to_back();
                *occupied.get_mut() = expires_at;
            }
            RawEntryMut::Vacant(vacant) => {
                vacant.insert(key.to_string(), expires_at);
            }
        }
    }

    pub fn get(&mut self, key: &str) -> Result<Option<KvModel>> {
        self.fetch_non_expired(key)
    }

    // FIXME(@svix-lucho): needs to be passed now() from the caller!
    fn fetch_non_expired(&mut self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(&self.tables, &key.to_string())? else {
            return Ok(None);
        };

        if data.expires_at.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        self.update_lru(key, data.expires_at);

        Ok(Some(data.into()))
    }

    fn insert_with_expiration(&mut self, key: &str, model: &KvModel) -> Result<()> {
        let mut batch = self.db.batch();

        let row = KvPairRow {
            key: key.to_string(),
            value: model.value.clone(),
            expires_at: model.expires_at,
        };

        batch.insert_row(&self.tables, &row)?;

        if let Some(expires_at) = model.expires_at {
            let expiration_row = ExpirationRow::new(expires_at, key.to_string());
            batch.insert_row(&self.tables, &expiration_row)?;
        }

        batch.commit()?;

        self.update_lru(key, model.expires_at);

        Ok(())
    }

    pub fn set_operation(
        &self,
        key: String,
        model: KvModel,
        behavior: OperationBehavior,
    ) -> SetOperation {
        SetOperation::new(key, model, behavior)
    }

    pub fn set(&mut self, key: &str, model: &KvModel, behavior: OperationBehavior) -> Result<()> {
        // TODO: remove this method
        tracing::error!("unsafe method KvStore::set called!");
        self.set_(key, model, behavior)
    }

    fn set_(&mut self, key: &str, model: &KvModel, behavior: OperationBehavior) -> Result<()> {
        match behavior {
            OperationBehavior::Upsert => {
                self.insert_with_expiration(key, model)?;
            }
            OperationBehavior::Insert => {
                let exists = self.fetch_non_expired(key)?.is_some();

                if !exists {
                    self.insert_with_expiration(key, model)?;
                } else {
                    // FIXME(@svix-lucho): Do nothing?
                }
            }
            OperationBehavior::Update => {
                let exists = self.fetch_non_expired(key)?.is_some();
                if exists {
                    self.insert_with_expiration(key, model)?;
                } else {
                    // FIXME(@svix-lucho): Do nothing?
                }
            }
        }

        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        let mut batch = self.db.batch();

        if let Some(data) = KvPairRow::fetch(&self.tables, &key.to_string())? {
            // Delete from the expiration keyspace
            if let Some(expires_at) = data.expires_at {
                let r = ExpirationRow::new(expires_at, key.to_string());
                batch.remove_row::<ExpirationRow>(&self.tables, r.get_key().as_ref())?;
            }
            batch.remove_row::<KvPairRow>(&self.tables, &key.to_string())?;
        }

        batch.commit()?;

        self.lru.remove(key);

        Ok(())
    }

    pub fn disk_space_exceeds_capacity(&self) -> bool {
        self.tables.disk_space() > self.max_storage_bytes.unwrap_or(NonZeroU64::MAX).get()
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::iter(&self.tables)
    }

    // FIXME(@svix-lucho): needs to be passed now() from the caller!
    pub fn evict_lru(&mut self, count: usize) -> Result<()> {
        let mut evicted = 0;

        while evicted < count {
            if let Some((key, _)) = self.lru.pop_front() {
                // Check if key is still present. It could have been expired or the LRU map could be out of sync.
                if self.fetch_non_expired(&key)?.is_some() {
                    // FIXME(@svix-lucho): does this have to go through consensus?
                    let _ = self.delete(&key);
                    evicted += 1;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn clear_expired(&mut self, now: Timestamp) -> Result<()> {
        let mut removed = 0;
        let now_ms = now.as_millisecond();
        let mut expired_keys = Vec::new();

        for item in ExpirationRow::iter(&self.tables)? {
            if item.expiration_time.as_millisecond() < now_ms && removed < EXPIRATION_BATCH_SIZE {
                expired_keys.push(item.key);
                removed += 1;
            } else if item.expiration_time.as_millisecond() >= now_ms {
                // expiration rows are ordered, so we can break early
                break;
            }
        }

        // FIXME(@svix-lucho): we can batch removes here to make this more efficient
        for key in expired_keys {
            let _ = self.delete(&key);
        }

        Ok(())
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
/// It deletes expired entries from the database and evicts entries if the KvStore is configured to do so.
pub async fn worker<F>(configgroup_state: &coyote_configgroup::State, is_shutting_down: F)
where
    F: Fn() -> bool,
{
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(1));

    let clean_up = |mut store: KvStore| {
        let now = Timestamp::now();
        // Expiration cleanup
        let _ = store.clear_expired(now);

        // Eviction
        if store.disk_space_exceeds_capacity() && store.policy == EvictionPolicy::LeastRecentlyUsed
        {
            // FIXME(@svix-lucho): we can add smarter eviction instead of just doing one at a time
            let _ = store.evict_lru(1);
        }
    };

    loop {
        if is_shutting_down() {
            break;
        }

        timer.tick().await;

        let kv_groups = match configgroup_state.fetch_all_groups() {
            Ok(groups) => groups,
            Err(e) => {
                tracing::error!(error = ?e, "Failed to get KV config groups.");
                continue;
            }
        };

        for group in kv_groups {
            let group: ConfigGroup<KeyValueConfig> = match group {
                Ok(g) => g,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to parse KV config group.");
                    continue;
                }
            };
            let db = configgroup_state.give_me_the_right_db(&group);
            let policy = group.config.eviction_policy();
            let store = KvStore::new(
                KeyValueConfig::NAMESPACE,
                db,
                policy,
                group.max_storage_bytes,
            );

            clean_up(store);
        }

        let cache_groups = match configgroup_state.fetch_all_groups() {
            Ok(groups) => groups,
            Err(e) => {
                tracing::error!(error = ?e, "Failed to get Cache config groups.");
                continue;
            }
        };

        for group in cache_groups {
            let group: ConfigGroup<CacheConfig> = match group {
                Ok(g) => g,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to process Cache config group.");
                    continue;
                }
            };
            let db = configgroup_state.give_me_the_right_db(&group);
            let policy = group.config.eviction_policy();
            let store = KvStore::new(CacheConfig::NAMESPACE, db, policy, group.max_storage_bytes);

            clean_up(store);
        }

        let idempotency_groups = match configgroup_state.fetch_all_groups() {
            Ok(groups) => groups,
            Err(e) => {
                tracing::error!(error = ?e, "Failed to get Idempotency config groups.");
                continue;
            }
        };

        for group in idempotency_groups {
            let group: ConfigGroup<IdempotencyConfig> = match group {
                Ok(g) => g,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to process Idempotency config group.");
                    continue;
                }
            };
            let db = configgroup_state.give_me_the_right_db(&group);
            let policy = group.config.eviction_policy();
            let store = KvStore::new(CacheConfig::NAMESPACE, db, policy, group.max_storage_bytes);

            clean_up(store);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use jiff::ToSpan;

    use super::*;

    struct SetupFixture {
        _workdir: tempfile::TempDir,
        store: KvStore,
    }

    impl SetupFixture {
        fn new() -> Self {
            Self::new_with_policy(EvictionPolicy::NoEviction)
        }

        fn new_with_policy(policy: EvictionPolicy) -> Self {
            let workdir = tempfile::tempdir().unwrap();
            let db = Database::builder(workdir.as_ref())
                .temporary(true)
                .open()
                .unwrap();
            let store = KvStore::new("test", db, policy, None);
            Self {
                _workdir: workdir,
                store,
            }
        }
    }

    #[test]
    fn test_insert_and_get() {
        let setup = SetupFixture::new();
        let mut store = setup.store;

        let key = "test:key1";
        let model = KvModel {
            expires_at: None,
            value: b"hello world".to_vec(),
        };

        store.set(key, &model, OperationBehavior::Upsert).unwrap();

        assert!(key_exists(&mut store, "test:key1"));
        let retrieved = store.get("test:key1").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expires_at, None);

        assert!(!key_exists(&mut store, "nonexistent:key"));
    }

    #[test]
    fn test_insert_behaviors() {
        let setup = SetupFixture::new();
        let mut store = setup.store;

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"key1 updated".to_vec(),
            },
            OperationBehavior::Update,
        );
        assert!(res.is_ok());
        assert!(!key_exists(&mut store, "key1"));

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"key1 inserted".to_vec(),
            },
            OperationBehavior::Insert,
        );
        assert!(res.is_ok());
        assert!(key_exists(&mut store, "key1"));
        let result = store.get("key1").unwrap();
        assert!(result.is_some());

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"another value".to_vec(),
            },
            OperationBehavior::Insert,
        );
        assert!(res.is_ok());
        assert!(key_exists(&mut store, "key1"));
        let result = store.get("key1").unwrap();
        assert!(result.is_some());

        assert_eq!(result.unwrap().value, b"key1 inserted");

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"key1 upserted".to_vec(),
            },
            OperationBehavior::Upsert,
        );
        assert!(res.is_ok());
        assert!(key_exists(&mut store, "key1"));
        let result = store.get("key1").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, b"key1 upserted");
    }

    #[test]
    fn test_overwrite() {
        let setup = SetupFixture::new();
        let mut store = setup.store;

        let key = "overwrite:key";
        let model1 = KvModel {
            expires_at: None,
            value: b"first value".to_vec(),
        };
        store.set(key, &model1, OperationBehavior::Upsert).unwrap();

        let model2 = KvModel {
            expires_at: None,
            value: b"second value".to_vec(),
        };
        store.set(key, &model2, OperationBehavior::Upsert).unwrap();

        assert!(key_exists(&mut store, "overwrite:key"));
        let retrieved = store.get("overwrite:key").unwrap().unwrap();
        assert_eq!(retrieved.value, b"second value");
    }

    #[test]
    fn test_clear_expired_removes_expired_entries() {
        let setup = SetupFixture::new();
        let mut store = setup.store;

        let expired_model = KvModel {
            expires_at: Some(Timestamp::now().checked_sub(1.hour()).unwrap()),
            value: b"expired data".to_vec(),
        };
        store
            .set("expired:key", &expired_model, OperationBehavior::Upsert)
            .unwrap();

        let now = Timestamp::now();
        let expired_models = [
            (
                "expired:key:1",
                KvModel {
                    expires_at: Some(now.checked_sub(3.hour()).unwrap()),
                    value: b"expired data 1".to_vec(),
                },
            ),
            (
                "expired:key:2",
                KvModel {
                    expires_at: Some(now.checked_sub(2.hour()).unwrap()),
                    value: b"expired data 2".to_vec(),
                },
            ),
            (
                "expired:key:3",
                KvModel {
                    expires_at: Some(now.checked_sub(1.second()).unwrap()), // really close to now
                    value: b"expired data 3".to_vec(),
                },
            ),
        ];

        for (key, model) in &expired_models {
            store.set(key, model, OperationBehavior::Upsert).unwrap();
        }

        let valid_model = KvModel {
            expires_at: Some(now.checked_add(1.hour()).unwrap()),
            value: b"valid data".to_vec(),
        };
        let valid_key = "valid:key";
        store
            .set(valid_key, &valid_model, OperationBehavior::Upsert)
            .unwrap();

        let permanent_model = KvModel {
            expires_at: None,
            value: b"permanent data".to_vec(),
        };
        let permanent_key = "permanent:key";
        store
            .set(permanent_key, &permanent_model, OperationBehavior::Upsert)
            .unwrap();

        assert_eq!(store.iter().unwrap().count(), 6);

        let now = Timestamp::now();
        store.clear_expired(now).unwrap();

        for (key, _) in &expired_models {
            assert!(!key_exists(&mut store, key));
        }

        assert!(key_exists(&mut store, valid_key));
        let valid = store.get(valid_key).unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        assert!(key_exists(&mut store, permanent_key));
        let permanent = store.get(permanent_key).unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }

    fn insert_key(kv: &mut KvStore, key: &str, expires_at: Option<Timestamp>) {
        kv.set(
            key,
            &KvModel {
                expires_at,
                value: key.as_bytes().to_vec(),
            },
            OperationBehavior::Upsert,
        )
        .unwrap()
    }

    fn key_exists(cache: &mut KvStore, key: &str) -> bool {
        cache.get(key).unwrap().is_some()
    }

    #[test]
    fn test_lru_eviction() {
        let setup = SetupFixture::new_with_policy(EvictionPolicy::LeastRecentlyUsed);
        let mut kv = setup.store;

        for i in 0..3 {
            insert_key(&mut kv, format!("k{i}").as_str(), None);
        }

        kv.get("k2").unwrap();
        kv.get("k2").unwrap();
        kv.get("k1").unwrap();
        kv.get("k1").unwrap();

        kv.evict_lru(1).unwrap();

        assert!(!key_exists(&mut kv, "k0"));
        assert!(key_exists(&mut kv, "k1"));
        assert!(key_exists(&mut kv, "k2"));
    }

    #[test]
    fn test_lru_eviction_with_expiration() {
        let setup = SetupFixture::new_with_policy(EvictionPolicy::LeastRecentlyUsed);
        let mut kv = setup.store;

        insert_key(
            &mut kv,
            "k0",
            Some(Timestamp::now() + Duration::from_secs(1)),
        );

        for i in 1..5 {
            insert_key(&mut kv, format!("k{i}").as_str(), None);
        }

        assert!(key_exists(&mut kv, "k0"));
        assert!(key_exists(&mut kv, "k1"));
        assert!(key_exists(&mut kv, "k2"));
        assert!(key_exists(&mut kv, "k3"));
        assert!(key_exists(&mut kv, "k4"));

        for i in 2..5 {
            kv.get(format!("k{i}").as_str()).unwrap();
        }

        let now = Timestamp::now();
        kv.clear_expired(now + Duration::from_secs(5)).unwrap();
        kv.evict_lru(1).unwrap();

        assert!(!key_exists(&mut kv, "k0")); // expired
        assert!(!key_exists(&mut kv, "k1")); // evicted

        assert!(key_exists(&mut kv, "k2"));
        assert!(key_exists(&mut kv, "k3"));
        assert!(key_exists(&mut kv, "k4"));

        // Evict all remaining keys
        kv.evict_lru(10).unwrap();

        assert!(kv.iter().unwrap().next().is_none());
    }

    #[test]
    fn test_lru_eviction_already_expired() {
        let setup = SetupFixture::new_with_policy(EvictionPolicy::LeastRecentlyUsed);
        let mut kv = setup.store;

        insert_key(
            &mut kv,
            "k0",
            Some(Timestamp::now() + Duration::from_secs(100)),
        );

        insert_key(
            &mut kv,
            "k1",
            Some(Timestamp::now() + Duration::from_secs(5)),
        );
        insert_key(&mut kv, "k2", None);

        assert!(key_exists(&mut kv, "k0"));
        assert!(key_exists(&mut kv, "k1"));
        assert!(key_exists(&mut kv, "k2"));

        kv.get("k1").unwrap();
        kv.get("k2").unwrap();

        let now = Timestamp::now();
        kv.evict_lru(1).unwrap();
        kv.clear_expired(now + Duration::from_secs(10)).unwrap();

        assert!(!key_exists(&mut kv, "k0")); // evicted
        assert!(!key_exists(&mut kv, "k1")); // expired
        assert!(key_exists(&mut kv, "k2"));
    }
}
