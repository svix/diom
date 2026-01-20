use std::{collections::HashMap, time::Duration};

use itertools::Itertools;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::types::EntityKey,
    error::Result,
    v1::modules::kv::{KvModel, KvStore, OperationBehavior},
};

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) kv: KvStore,
    pub(crate) lru_clock: HashMap<EntityKey, u64>,
    // TBD: LFU
}

const MAX_LRU_CLOCK_SIZE: usize = 128;

impl CacheStore {
    pub fn new(kv: KvStore) -> Self {
        Self {
            kv,
            lru_clock: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: EntityKey, model: CacheModel) -> Result<()> {
        if let Some(c) = self.lru_clock.get_mut(&key) {
            *c += 1;
        }
        self.kv.set(&key, &model.into(), OperationBehavior::Upsert)
    }

    pub fn get(&mut self, key: &EntityKey) -> Result<Option<CacheModel>> {
        if let Some(c) = self.lru_clock.get_mut(key) {
            *c += 1;
        }
        self.kv.get(&key.0).map(|m| m.map(Into::into))
    }

    pub fn delete(&mut self, key: &EntityKey) -> Result<()> {
        self.lru_clock.remove(key);
        self.kv.delete(&key.0)
    }

    pub fn reset_lru_clock(&mut self) -> Result<()> {
        self.lru_clock.clear();

        // Take a random sample of the KV store to count access counts
        // bug: iter is always sorted - we want to take a random sample
        for kv in self.kv.iter().take(MAX_LRU_CLOCK_SIZE) {
            let key = EntityKey(
                // WTF
                std::str::from_utf8(kv.key().unwrap().as_ref())
                    .unwrap()
                    .to_string(),
            );
            self.lru_clock.insert(key, 0);
        }

        Ok(())
    }

    pub fn evict_lru(&mut self, count: usize) -> Result<()> {
        for (key, _) in self
            .lru_clock
            .iter()
            .sorted_by_key(|(_, count)| *count)
            .take(count)
        {
            let _ = self.kv.delete(key);
        }
        Ok(())
    }
}

const MAX_CAPACITY: u64 = 1024; // TBD: of course, make it configurable

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(mut state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }

        // XXX: this is very basic...  we could do batch removes and be based on the size of the elements.
        if state.cache_store.kv.total_size() > MAX_CAPACITY {
            let _ = state.cache_store.reset_lru_clock();
            let _ = state.cache_store.evict_lru(1);
            tokio::time::sleep(Duration::from_millis(5)).await;
        } else {
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheModel {
    pub expires_at: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl From<CacheModel> for KvModel {
    fn from(model: CacheModel) -> Self {
        KvModel {
            value: model.value,
            expires_at: model.expires_at,
        }
    }
}

impl From<KvModel> for CacheModel {
    fn from(model: KvModel) -> Self {
        CacheModel {
            value: model.value,
            expires_at: model.expires_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_eviction() {
        let mut cache = CacheStore::new(KvStore::new_temporary("test_lru"));

        // Insert 3 entries
        for i in 0..3 {
            let key = EntityKey(format!("k{i}"));
            cache
                .set(
                    key,
                    CacheModel {
                        expires_at: None,
                        value: vec![i],
                    },
                )
                .unwrap();
        }

        cache.reset_lru_clock().unwrap();

        cache.get(&EntityKey("k2".into())).unwrap();
        cache.get(&EntityKey("k2".into())).unwrap();
        cache.get(&EntityKey("k1".into())).unwrap();
        cache.get(&EntityKey("k1".into())).unwrap();
        cache.get(&EntityKey("k1".into())).unwrap();
        cache.get(&EntityKey("k3".into())).unwrap();

        cache.evict_lru(1).unwrap();

        assert!(cache.get(&EntityKey("k0".into())).unwrap().is_none());
        assert!(cache.get(&EntityKey("k1".into())).unwrap().is_some());
        assert!(cache.get(&EntityKey("k2".into())).unwrap().is_some());
    }
}
