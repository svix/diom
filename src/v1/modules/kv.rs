use std::time::Duration;

use fjall::Slice;
use fjall::{Database, Keyspace, UserKey, UserValue};
use format_bytes::format_bytes;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::types::EntityKey;
use crate::{AppState, error::Result};
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Kv2Model {
    pub expires_at: Option<Timestamp>,
    pub value: Vec<u8>,
}

impl Kv2Model {
    fn data_key(key: &str) -> fjall::Slice {
        key.as_bytes().into()
    }

    fn expiration_key(key: &String, timestamp: Timestamp) -> Slice {
        format_bytes!(
            b"{}\0{}",
            timestamp.as_millisecond().to_string().as_bytes(),
            key.as_bytes(),
        )
        .into()
    }
}

impl From<&Kv2Model> for Vec<u8> {
    fn from(val: &Kv2Model) -> Self {
        rmp_serde::to_vec(&val).expect("should serialize")
    }
}

impl From<(UserKey, UserValue)> for Kv2Model {
    fn from((_key, value): (UserKey, UserValue)) -> Self {
        rmp_serde::from_slice(&value).expect("should deserialize")
    }
}

#[derive(Clone)]
pub struct Kv2Store {
    #[allow(unused)]
    db: Database,

    data: Keyspace,
    expiration: Keyspace,
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

const DIOM_KV2_DATA_KEYSPACE: &str = "DIOM_KV2_DATA";
const DIOM_KV2_EXPIRATION_KEYSPACE: &str = "DIOM_KV2_EXPIRATION";

impl Default for Kv2Store {
    fn default() -> Self {
        Self::new("default")
    }
}

impl Kv2Store {
    pub fn new(namespace: &str) -> Self {
        // Should we share all the same file space and use different keyspaces or what?
        let db = fjall::Database::builder(format!(".data/kv_{}", namespace))
            .open()
            .unwrap();
        let data = db
            .keyspace(
                DIOM_KV2_DATA_KEYSPACE,
                fjall::KeyspaceCreateOptions::default,
            )
            .unwrap();
        let expiration = db
            .keyspace(
                DIOM_KV2_EXPIRATION_KEYSPACE,
                fjall::KeyspaceCreateOptions::default,
            )
            .unwrap();

        Self {
            db,
            data,
            expiration,
        }
    }

    // TBD: needs to be passed now() from the caller?
    pub fn get(&self, key: &str) -> Result<Option<Kv2Model>> {
        let Some(data) = self.data.get(key).map_err(crate::error::Error::generic)? else {
            return Ok(None);
        };

        let model: Kv2Model = rmp_serde::from_slice(&data).expect("should deserialize");

        if model.expires_at.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        Ok(Some(model))
    }

    pub fn set(
        &self,
        key: &EntityKey,
        model: &Kv2Model,
        behavior: OperationBehavior,
    ) -> Result<()> {
        match behavior {
            OperationBehavior::Upsert => {
                let serialized: Vec<u8> = model.into();
                let mut batch = self.db.batch();

                batch.insert(
                    &self.data,
                    Kv2Model::data_key(&key.0),
                    Slice::from(serialized),
                );
                if let Some(expires_at) = model.expires_at {
                    batch.insert(
                        &self.expiration,
                        Kv2Model::expiration_key(&key.0, expires_at),
                        Slice::from(key.0.as_bytes()),
                    );
                }

                batch.commit().unwrap();
            }
            OperationBehavior::Insert => {
                // XXX: Not atomic bro
                let exists = self.get(key).is_ok_and(|e| e.is_some());
                if !exists {
                    let _ = self.set(key, model, OperationBehavior::Upsert);
                } else {
                    // Do nothing?
                }
            }
            OperationBehavior::Update => {
                // XXX: Not atomic bro
                let exists = self.get(key).is_ok_and(|e| e.is_some());
                if exists {
                    let _ = self.set(key, model, OperationBehavior::Upsert);
                } else {
                    // Do nothing?
                }
            }
        }

        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let key_string = key.to_string();
        if let Some(data) = self.data.get(key).map_err(crate::error::Error::generic)? {
            let model: Kv2Model = rmp_serde::from_slice(&data).expect("should deserialize");
            let mut batch = self.db.batch();
            batch.remove(&self.data, Kv2Model::data_key(&key_string));
            if let Some(expires_at) = model.expires_at {
                batch.remove(
                    &self.expiration,
                    Kv2Model::expiration_key(&key_string, expires_at),
                );
            }
            batch.commit().unwrap();
        }
        Ok(())
    }

    pub fn approximate_len(&self) -> fjall::Result<usize> {
        self.data.len()
    }

    pub fn clear_expired(&self, now: Timestamp) -> Result<()> {
        let mut removed = 0;
        let now_ms = now.as_millisecond();

        while let Some(item) = self.expiration.first_key_value() {
            let (exp_key, data_key) = item.into_inner().expect("should get key and value?");

            // exp_key format: "{timestamp_ms}\0{original_key}"
            let sep_pos = exp_key.iter().position(|&b| b == 0);
            let timestamp_ms: i64 = match sep_pos {
                Some(pos) => {
                    let ts_str = std::str::from_utf8(&exp_key[..pos]).unwrap_or("0");
                    ts_str.parse().unwrap_or(i64::MAX)
                }
                None => continue,
            };

            if timestamp_ms < now_ms {
                let original_key = std::str::from_utf8(&data_key).expect("key should be utf8");
                let _ = self.delete(original_key);
                removed += 1;

                if removed > 100 {
                    break;
                }
            } else {
                break; // sorted by timestamp, so we're done
            }
        }
        Ok(())
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    loop {
        if crate::is_shutting_down() {
            break;
        }
        let now = Timestamp::now();
        let _ = state.kv_store.clear_expired(now);
        let _ = state.cache_store.kv.clear_expired(now);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::ToSpan;

    fn create_test_store(path: &str) -> Kv2Store {
        let db = Database::builder(path).temporary(true).open().unwrap();
        let data = db
            .keyspace("kv2_data", fjall::KeyspaceCreateOptions::default)
            .unwrap();
        let expiration = db
            .keyspace("kv2_expiration", fjall::KeyspaceCreateOptions::default)
            .unwrap();
        Kv2Store {
            db,
            data,
            expiration,
        }
    }

    #[test]
    fn test_insert_and_get() {
        let store = create_test_store(".fjall_test_kv2");

        let key = EntityKey("test:key1".to_string());
        let model = Kv2Model {
            expires_at: None,
            value: b"hello world".to_vec(),
        };

        // Insert
        store.set(&key, &model, OperationBehavior::Upsert).unwrap();

        // Get and verify
        let retrieved = store.get("test:key1").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expires_at, None);
    }

    #[test]
    fn test_get_nonexistent() {
        let store = create_test_store(".fjall_test_kv2_nonexistent");

        let result = store.get("nonexistent:key").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_with_expiration() {
        let store = create_test_store(".fjall_test_kv2_expiry");

        let key = EntityKey("expiring:key".to_string());
        let expires_at = Timestamp::now().checked_add(1.hour()).unwrap();
        let model = Kv2Model {
            expires_at: Some(expires_at),
            value: b"temporary data".to_vec(),
        };

        store.set(&key, &model, OperationBehavior::Upsert).unwrap();

        let retrieved = store.get("expiring:key").unwrap().unwrap();
        assert_eq!(retrieved.expires_at, Some(expires_at));
        assert_eq!(retrieved.value, b"temporary data");
    }

    #[test]
    fn test_multiple_inserts() {
        let store = create_test_store(".fjall_test_kv2_multi");

        let items = vec![
            (
                EntityKey("user:1".to_string()),
                Kv2Model {
                    expires_at: None,
                    value: b"alice".to_vec(),
                },
            ),
            (
                EntityKey("user:2".to_string()),
                Kv2Model {
                    expires_at: None,
                    value: b"bob".to_vec(),
                },
            ),
            (
                EntityKey("user:3".to_string()),
                Kv2Model {
                    expires_at: None,
                    value: b"charlie".to_vec(),
                },
            ),
        ];

        // Insert all items
        for (key, model) in &items {
            store.set(key, model, OperationBehavior::Upsert).unwrap();
        }

        // Verify count
        assert_eq!(store.approximate_len().unwrap(), items.len());

        // Verify each item
        for (key, expected_model) in &items {
            let retrieved = store.get(&key.0).unwrap().unwrap();
            assert_eq!(retrieved.value, expected_model.value);
        }
    }

    #[test]
    fn test_overwrite() {
        let store = create_test_store(".fjall_test_kv2_overwrite");

        let key = EntityKey("overwrite:key".to_string());
        let model1 = Kv2Model {
            expires_at: None,
            value: b"first value".to_vec(),
        };
        store.set(&key, &model1, OperationBehavior::Upsert).unwrap();

        let model2 = Kv2Model {
            expires_at: None,
            value: b"second value".to_vec(),
        };
        store.set(&key, &model2, OperationBehavior::Upsert).unwrap();

        let retrieved = store.get("overwrite:key").unwrap().unwrap();
        assert_eq!(retrieved.value, b"second value");
    }

    #[test]
    fn test_clear_expired_removes_expired_entries() {
        let store = create_test_store(".fjall_test_kv2_clear_expired");

        // Insert an entry that's already expired (1 hour in the past)
        let expired_key = EntityKey("expired:key".to_string());
        let expired_model = Kv2Model {
            expires_at: Some(Timestamp::now().checked_sub(1.hour()).unwrap()),
            value: b"expired data".to_vec(),
        };
        store
            .set(&expired_key, &expired_model, OperationBehavior::Upsert)
            .unwrap();

        let now = Timestamp::now();
        let expired_models = [
            (
                EntityKey("expired:key:1".to_string()),
                Kv2Model {
                    expires_at: Some(now.checked_sub(3.hour()).unwrap()),
                    value: b"expired data 1".to_vec(),
                },
            ),
            (
                EntityKey("expired:key:2".to_string()),
                Kv2Model {
                    expires_at: Some(now.checked_sub(2.hour()).unwrap()),
                    value: b"expired data 2".to_vec(),
                },
            ),
            (
                EntityKey("expired:key:3".to_string()),
                Kv2Model {
                    expires_at: Some(now.checked_sub(1.second()).unwrap()), // really close to now
                    value: b"expired data 3".to_vec(),
                },
            ),
        ];

        for (key, model) in &expired_models {
            store.set(key, model, OperationBehavior::Upsert).unwrap();
        }

        let valid_key = EntityKey("valid:key".to_string());
        let valid_model = Kv2Model {
            expires_at: Some(now.checked_add(1.hour()).unwrap()),
            value: b"valid data".to_vec(),
        };
        store
            .set(&valid_key, &valid_model, OperationBehavior::Upsert)
            .unwrap();

        // Insert an entry with no expiration
        let permanent_key = EntityKey("permanent:key".to_string());
        let permanent_model = Kv2Model {
            expires_at: None,
            value: b"permanent data".to_vec(),
        };
        store
            .set(&permanent_key, &permanent_model, OperationBehavior::Upsert)
            .unwrap();

        assert_eq!(store.approximate_len().unwrap(), 6);

        let now = Timestamp::now();
        store.clear_expired(now).unwrap();

        for (key, _) in &expired_models {
            assert!(store.get(&key.0).unwrap().is_none());
        }

        let valid = store.get("valid:key").unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        let permanent = store.get("permanent:key").unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }
}
