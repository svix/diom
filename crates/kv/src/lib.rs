use std::num::NonZeroU64;

use coyote_error::Result;
use coyote_namespace::entities::{
    CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig, ModuleConfig, NamespaceId,
};
use fjall::KeyspaceCreateOptions;
use fjall_utils::{TableRow, WriteBatchExt};
use hashlink::{LinkedHashMap, linked_hash_map::RawEntryMut};
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod kvcontroller;
pub mod operations;
pub mod tables;

use crate::{
    kvcontroller::KvController,
    tables::{KvPairRow, OldExpirationRow},
};

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
pub struct State {
    pub controller: KvController,
}

impl State {
    pub fn init(db: fjall::Database) -> Result<Self> {
        Ok(Self {
            controller: KvController::new(db, "mod_kv"),
        })
    }
}

#[derive(Clone)]
pub struct KvStore {
    db: fjall::Database,
    tables: fjall::Keyspace,
    namespace_id: NamespaceId,
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
        db: fjall::Database,
        policy: EvictionPolicy,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        let kv_keyspace = format!("mod_kv_{namespace}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&kv_keyspace, || opts).unwrap()
        };

        Self {
            db,
            tables,
            namespace_id: NamespaceId::nil(), // FIXME: we'll soon kill kvstore.
            policy,
            lru: LinkedHashMap::new(),
            max_storage_bytes,
        }
    }

    fn update_lru(&mut self, key: &str, expiry: Option<Timestamp>) {
        match self.lru.raw_entry_mut().from_key(key) {
            RawEntryMut::Occupied(mut occupied) => {
                occupied.to_back();
                *occupied.get_mut() = expiry;
            }
            RawEntryMut::Vacant(vacant) => {
                vacant.insert(key.to_string(), expiry);
            }
        }
    }

    pub fn get(&mut self, key: &str) -> Result<Option<KvModel>> {
        self.fetch_non_expired(key)
    }

    // FIXME(@svix-lucho): needs to be passed now() from the caller!
    fn fetch_non_expired(&mut self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) =
            KvPairRow::fetch(&self.tables, KvPairRow::key_for(self.namespace_id, key))?
        else {
            return Ok(None);
        };

        if data.expiry.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        self.update_lru(key, data.expiry);

        Ok(Some(data.into()))
    }

    fn insert_with_expiration(&mut self, key: &str, model: &KvModel) -> Result<()> {
        let mut batch = self.db.batch();

        let row = KvPairRow {
            key: key.to_string(),
            value: model.value.clone(),
            expiry: model.expiry,
        };

        batch.insert_row(
            &self.tables,
            KvPairRow::key_for(self.namespace_id, key),
            &row,
        )?;

        if let Some(expiry) = model.expiry {
            let expiration_row = OldExpirationRow::new(expiry, key.to_string());
            batch.insert_row(
                &self.tables,
                OldExpirationRow::key_for(expiry, key),
                &expiration_row,
            )?;
        }

        batch.commit()?;

        self.update_lru(key, model.expiry);

        Ok(())
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

        if let Some(data) =
            KvPairRow::fetch(&self.tables, KvPairRow::key_for(self.namespace_id, key))?
        {
            // Delete from the expiration keyspace
            if let Some(expiry) = data.expiry {
                batch.remove_row(&self.tables, OldExpirationRow::key_for(expiry, key))?;
            }
            batch.remove_row(&self.tables, KvPairRow::key_for(self.namespace_id, key))?;
        }

        batch.commit()?;

        self.lru.remove(key);

        Ok(())
    }

    pub fn disk_space_exceeds_capacity(&self) -> bool {
        self.tables.disk_space() > self.max_storage_bytes.unwrap_or(NonZeroU64::MAX).get()
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::values(&self.tables)
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

        for item in OldExpirationRow::values(&self.tables)? {
            if item.expiry.as_millisecond() < now_ms && removed < EXPIRATION_BATCH_SIZE {
                expired_keys.push(item.key);
                removed += 1;
            } else if item.expiry.as_millisecond() >= now_ms {
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
pub async fn worker<F>(namespace_state: &coyote_namespace::State, is_shutting_down: F)
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

        let kv_namespaces = match namespace_state.fetch_all_namespaces::<KeyValueConfig>() {
            Ok(namespaces) => namespaces,
            Err(e) => {
                tracing::error!(error = ?e, "Failed to get KV namespaces.");
                continue;
            }
        };

        for namespace in kv_namespaces {
            let db = namespace_state.give_me_the_right_db(&namespace);
            let policy = namespace.config.eviction_policy();
            let store = KvStore::new(
                KeyValueConfig::NAMESPACE,
                db,
                policy,
                namespace.max_storage_bytes,
            );

            clean_up(store);
        }

        let cache_namespaces = match namespace_state.fetch_all_namespaces::<CacheConfig>() {
            Ok(namespaces) => namespaces,
            Err(e) => {
                tracing::error!(error = ?e, "Failed to get Cache namespaces.");
                continue;
            }
        };

        for namespace in cache_namespaces {
            let db = namespace_state.give_me_the_right_db(&namespace);
            let policy = namespace.config.eviction_policy();
            let store = KvStore::new(
                CacheConfig::NAMESPACE,
                db,
                policy,
                namespace.max_storage_bytes,
            );

            clean_up(store);
        }

        let idempotency_namespaces =
            match namespace_state.fetch_all_namespaces::<IdempotencyConfig>() {
                Ok(namespaces) => namespaces,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to get Idempotency namespaces.");
                    continue;
                }
            };

        for namespace in idempotency_namespaces {
            let db = namespace_state.give_me_the_right_db(&namespace);
            let policy = namespace.config.eviction_policy();
            let store = KvStore::new(
                CacheConfig::NAMESPACE,
                db,
                policy,
                namespace.max_storage_bytes,
            );

            clean_up(store);
        }
    }
}
