use std::time::Duration;

mod store;
use fjall::{OptimisticTxDatabase, Slice};
use fjall::{OptimisticTxKeyspace, UserKey, UserValue};
use format_bytes::format_bytes;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Result type for KV operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for KV operations.
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(msg: impl std::fmt::Display) -> Self {
        Self(msg.to_string())
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KvModel {
    pub expires_at: Option<Timestamp>,
    pub value: Vec<u8>,
}

impl KvModel {
    fn data_key(key: &str) -> fjall::Slice {
        key.as_bytes().into()
    }

    fn expiration_key(key: &str, timestamp: Timestamp) -> Slice {
        format_bytes!(
            b"{}\0{}",
            &timestamp.as_millisecond().to_be_bytes(),
            key.as_bytes(),
        )
        .into()
    }
}

impl From<&KvModel> for Vec<u8> {
    fn from(val: &KvModel) -> Self {
        rmp_serde::to_vec(&val).expect("should serialize")
    }
}

impl From<(UserKey, UserValue)> for KvModel {
    fn from((_key, value): (UserKey, UserValue)) -> Self {
        rmp_serde::from_slice(&value).expect("should deserialize")
    }
}

#[derive(Clone)]
pub struct KvStore {
    #[allow(unused)]
    db: OptimisticTxDatabase, // should it be SingleWriterTxDatabase?

    data: OptimisticTxKeyspace,
    expiration: OptimisticTxKeyspace,
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

impl Default for KvStore {
    fn default() -> Self {
        Self::new("default")
    }
}

const EXPIRATION_BATCH_SIZE: usize = 100; // configurable?

impl KvStore {
    pub fn new(namespace: &str) -> Self {
        // Should we share all the same file space and use different keyspaces or what?
        let db = OptimisticTxDatabase::builder(format!("db/kv/{namespace}"))
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

    pub fn new_temporary(namespace: &str) -> Self {
        let db = fjall::OptimisticTxDatabase::builder(format!("db/kv/{namespace}"))
            .temporary(true)
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
    pub fn get(&self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) = self.data.get(key).map_err(Error::new)? else {
            return Ok(None);
        };

        let model: KvModel = rmp_serde::from_slice(&data).expect("should deserialize");

        if model.expires_at.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        Ok(Some(model))
    }

    pub fn set(&self, key: &str, model: &KvModel, behavior: OperationBehavior) -> Result<()> {
        let mut tx = self.db.write_tx().unwrap();

        match behavior {
            OperationBehavior::Upsert => {
                let serialized: Vec<u8> = model.into();

                tx.insert(&self.data, KvModel::data_key(key), Slice::from(serialized));
                if let Some(expires_at) = model.expires_at {
                    tx.insert(
                        &self.expiration,
                        KvModel::expiration_key(key, expires_at),
                        Slice::from(key.as_bytes()),
                    );
                }

                let _ = tx.commit().unwrap();
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
        let mut tx = self.db.write_tx().unwrap();
        if let Some(data) = tx
            .take(&self.data, KvModel::data_key(&key_string))
            .map_err(Error::new)?
        {
            let model: KvModel = rmp_serde::from_slice(&data).expect("should deserialize");

            // Delete from the expiration keyspace
            if let Some(expires_at) = model.expires_at {
                tx.remove(
                    &self.expiration,
                    KvModel::expiration_key(&key_string, expires_at),
                );
            }
        }
        let _ = tx.commit().unwrap();
        Ok(())
    }

    pub fn approximate_len(&self) -> usize {
        self.data.approximate_len()
    }

    pub fn clear_expired(&self, now: Timestamp) -> Result<()> {
        let mut removed = 0;
        let now_ms = now.as_millisecond();

        while let Some(item) = self.expiration.first_key_value() {
            let (exp_key, data_key) = item.into_inner().expect("should get key and value?");

            // exp_key format: "{timestamp_ms:i64 be}\0{original_key}"
            let timestamp_ms = i64::from_be_bytes(exp_key[..8].try_into().unwrap());

            if timestamp_ms < now_ms && removed < EXPIRATION_BATCH_SIZE {
                let original_key = std::str::from_utf8(&data_key).expect("key should be utf8");
                let _ = self.delete(original_key);
                removed += 1;
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> fjall::Iter {
        self.data.as_ref().iter()
    }

    // silly
    pub fn total_size(&self) -> u64 {
        self.data.as_ref().disk_space() + self.expiration.as_ref().disk_space()
    }
}

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker<F>(stores: &[&KvStore], is_shutting_down: F)
where
    F: Fn() -> bool,
{
    loop {
        if is_shutting_down() {
            break;
        }
        let now = Timestamp::now();
        for store in stores {
            let _ = store.clear_expired(now);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::ToSpan;

    #[test]
    fn test_insert_and_get() {
        let store = KvStore::new_temporary(".fjall_test_kv2");

        let key = "test:key1";
        let model = KvModel {
            expires_at: None,
            value: b"hello world".to_vec(),
        };

        // Insert
        store.set(key, &model, OperationBehavior::Upsert).unwrap();

        // Get and verify
        let retrieved = store.get("test:key1").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expires_at, None);
    }

    #[test]
    fn test_get_nonexistent() {
        let store = KvStore::new_temporary(".fjall_test_kv2_nonexistent");

        let result = store.get("nonexistent:key").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_with_expiration() {
        let store = KvStore::new_temporary(".fjall_test_kv2_expiry");

        let key = "expiring:key";
        let expires_at = Timestamp::now().checked_add(1.hour()).unwrap();
        let model = KvModel {
            expires_at: Some(expires_at),
            value: b"temporary data".to_vec(),
        };

        store.set(key, &model, OperationBehavior::Upsert).unwrap();

        let retrieved = store.get("expiring:key").unwrap().unwrap();
        assert_eq!(retrieved.expires_at, Some(expires_at));
        assert_eq!(retrieved.value, b"temporary data");
    }

    #[test]
    fn test_multiple_inserts() {
        let store = KvStore::new_temporary(".fjall_test_kv2_multi");

        let items = vec![
            (
                "user:1",
                KvModel {
                    expires_at: None,
                    value: b"alice".to_vec(),
                },
            ),
            (
                "user:2",
                KvModel {
                    expires_at: None,
                    value: b"bob".to_vec(),
                },
            ),
            (
                "user:3",
                KvModel {
                    expires_at: None,
                    value: b"charlie".to_vec(),
                },
            ),
        ];

        // Insert all items
        for (key, model) in &items {
            store.set(key, model, OperationBehavior::Upsert).unwrap();
        }

        // Verify each item
        for (key, expected_model) in &items {
            let retrieved = store.get(key).unwrap().unwrap();
            assert_eq!(retrieved.value, expected_model.value);
        }
    }

    #[test]
    fn test_overwrite() {
        let store = KvStore::new_temporary(".fjall_test_kv2_overwrite");

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

        let retrieved = store.get("overwrite:key").unwrap().unwrap();
        assert_eq!(retrieved.value, b"second value");
    }

    #[test]
    fn test_clear_expired_removes_expired_entries() {
        let store = KvStore::new_temporary(".fjall_test_kv2_clear_expired");

        // Insert an entry that's already expired (1 hour in the past)
        let expired_key = "expired:key";
        let expired_model = KvModel {
            expires_at: Some(Timestamp::now().checked_sub(1.hour()).unwrap()),
            value: b"expired data".to_vec(),
        };
        store
            .set(expired_key, &expired_model, OperationBehavior::Upsert)
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

        let valid_key = "valid:key";
        let valid_model = KvModel {
            expires_at: Some(now.checked_add(1.hour()).unwrap()),
            value: b"valid data".to_vec(),
        };
        store
            .set(valid_key, &valid_model, OperationBehavior::Upsert)
            .unwrap();

        // Insert an entry with no expiration
        let permanent_key = "permanent:key";
        let permanent_model = KvModel {
            expires_at: None,
            value: b"permanent data".to_vec(),
        };
        store
            .set(permanent_key, &permanent_model, OperationBehavior::Upsert)
            .unwrap();

        assert_eq!(store.iter().count(), 6);

        let now = Timestamp::now();
        store.clear_expired(now).unwrap();

        for (key, _) in &expired_models {
            assert!(store.get(key).unwrap().is_none());
        }

        let valid = store.get("valid:key").unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        let permanent = store.get("permanent:key").unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }
}
