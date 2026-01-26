use std::{collections::HashMap, time::Duration};

use itertools::Itertools;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, error::Result};

use coyote_kv::{KvModel, KvStore, OperationBehavior};

#[derive(Clone)]
pub struct CacheStore {
    pub(crate) kv: KvStore,
    pub(crate) lru_clock: HashMap<String, u64>,
}

const MAX_LRU_CLOCK_SIZE: usize = 128;

impl CacheStore {
    pub fn new(kv: KvStore) -> Self {
        Self {
            kv,
            lru_clock: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, model: CacheModel) -> Result<()> {
        if let Some(c) = self.lru_clock.get_mut(key) {
            *c += 1;
        }
        self.kv
            .set(key, &model.into(), OperationBehavior::Upsert)
            .map_err(|e| crate::error::Error::generic(e))
    }

    pub fn get(&mut self, key: &str) -> Result<Option<CacheModel>> {
        if let Some(c) = self.lru_clock.get_mut(key) {
            *c += 1;
        }
        self.kv
            .get(key)
            .map(|m| m.map(Into::into))
            .map_err(|e| crate::error::Error::generic(e))
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        self.lru_clock.remove(key);
        self.kv
            .delete(key)
            .map_err(|e| crate::error::Error::generic(e))
    }

    pub fn reset_lru_clock(&mut self) -> Result<()> {
        self.lru_clock.clear();

        // Take a random sample of the KV store to count access counts
        // FIXME: iter is always sorted - we want to take a random sample
        // Also, taking this many elements is inefficient...
        // Strip the table prefix to get the original key
        for kv in self.kv.iter()?.take(MAX_LRU_CLOCK_SIZE) {
            self.lru_clock.insert(kv.key.to_string(), 0);
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

const MAX_CAPACITY: u64 = 1024 * 8; // FIXME: of course, make it configurable

/// This is the worker function for this module, it does background cleanup and accounting.
/// FIXME(@svix-lucho): This has to be redone, it's... not great.
pub async fn worker(mut state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }

        // XXX: this is very basic...  we could do batch removes and be based on the size of the elements.
        if state.cache_store.kv.disk_space() > MAX_CAPACITY {
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

        for i in 0..3 {
            cache
                .set(
                    format!("k{i}").as_str(),
                    CacheModel {
                        expires_at: None,
                        value: vec![i],
                    },
                )
                .unwrap();
        }

        cache.reset_lru_clock().unwrap();

        cache.get("k2").unwrap();
        cache.get("k2").unwrap();
        cache.get("k1").unwrap();
        cache.get("k1").unwrap();

        cache.evict_lru(1).unwrap();

        assert!(cache.get("k0").unwrap().is_none());
        assert!(cache.get("k1").unwrap().is_some());
        assert!(cache.get("k2").unwrap().is_some());
    }
}
