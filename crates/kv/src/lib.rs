use std::time::Duration;
pub mod tables;
use coyote_error::Result;
use fjall::{Database, KeyspaceCreateOptions};

use fjall_utils::TableRow;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tables::{ExpirationRow, KvPairRow};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl Default for KvStore {
    fn default() -> Self {
        Self::new("default")
    }
}

const EXPIRATION_BATCH_SIZE: usize = 100; // FIXME(@svix-lucho): make this configurable?

impl KvStore {
    // FIXME(@svix-lucho): receive the db from the caller
    pub fn new(namespace: &str) -> Self {
        let db = Database::builder(format!("db/kv/{namespace}"))
            .open()
            .unwrap();

        let kv_keyspace = format!("_coyote_kv_{namespace}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&kv_keyspace, || opts).unwrap()
        };

        Self { db, tables }
    }

    pub fn new_temporary(namespace: &str) -> Self {
        let db = Database::builder(format!("db/kv/{namespace}"))
            .temporary(true)
            .open()
            .unwrap();

        let kv_keyspace = format!("_coyote_kv_temporary_{namespace}");

        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(&kv_keyspace, || opts).unwrap()
        };

        Self { db, tables }
    }

    // FIXME(@svix-lucho): needs to be passed now() from the caller?
    pub fn get(&self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(&self.tables, &key.to_string())? else {
            return Ok(None);
        };

        if data.expires_at.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        Ok(Some(data.into()))
    }

    fn fetch_non_expired(&self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(&self.tables, &key.to_string())? else {
            return Ok(None);
        };

        if data.expires_at.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        Ok(Some(data.into()))
    }

    fn insert_with_expiration(&self, key: &str, model: &KvModel) -> Result<()> {
        let mut batch = self.db.batch();

        let row = KvPairRow {
            key: key.to_string(),
            value: model.value.clone(),
            expires_at: model.expires_at,
        };

        KvPairRow::insert_batch(&mut batch, &self.tables, &row)?;

        if let Some(expires_at) = model.expires_at {
            let expiration_row = ExpirationRow::new(expires_at, key.to_string());
            ExpirationRow::insert_batch(&mut batch, &self.tables, &expiration_row)?;
        }

        batch.commit()?;

        Ok(())
    }

    pub fn set(&self, key: &str, model: &KvModel, behavior: OperationBehavior) -> Result<()> {
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
                let exists = self.get(key).is_ok_and(|e| e.is_some());
                if exists {
                    self.insert_with_expiration(key, model)?;
                } else {
                    // FIXME(@svix-lucho): Do nothing?
                }
            }
        }

        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let mut batch = self.db.batch();

        if let Some(data) = KvPairRow::fetch(&self.tables, &key.to_string())? {
            // Delete from the expiration keyspace
            if let Some(expires_at) = data.expires_at {
                let r = ExpirationRow::new(expires_at, key.to_string());
                ExpirationRow::remove_batch(&mut batch, &self.tables, r.get_key())?;
            }
            KvPairRow::remove_batch(&mut batch, &self.tables, &key.to_string())?;
        }

        batch.commit()?;

        Ok(())
    }

    pub fn disk_space(&self) -> u64 {
        self.tables.disk_space()
    }

    pub fn clear_expired(&self, now: Timestamp) -> Result<()> {
        let mut removed = 0;
        let now_ms = now.as_millisecond();

        for item in ExpirationRow::iter(&self.tables)? {
            if item.expiration_time.as_millisecond() < now_ms && removed < EXPIRATION_BATCH_SIZE {
                // FIXME(@svix-lucho): we can batch removes here to make this more efficient
                let _ = self.delete(&item.key);
                removed += 1;
            } else if item.expiration_time.as_millisecond() >= now_ms {
                // expiration rows are ordered, so we can break early
                break;
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::iter(&self.tables)
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
        let store = KvStore::new_temporary(".test_kv");

        let key = "test:key1";
        let model = KvModel {
            expires_at: None,
            value: b"hello world".to_vec(),
        };

        store.set(key, &model, OperationBehavior::Upsert).unwrap();

        let retrieved = store.get("test:key1").unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.value, b"hello world");
        assert_eq!(retrieved.expires_at, None);

        let result = store.get("nonexistent:key").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_insert_behaviors() {
        let store = KvStore::new_temporary(".test_kv");

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"key1 updated".to_vec(),
            },
            OperationBehavior::Update,
        );
        assert!(res.is_ok());
        let result = store.get("key1").unwrap();
        assert!(result.is_none());

        let res = store.set(
            "key1",
            &KvModel {
                expires_at: None,
                value: b"key1 inserted".to_vec(),
            },
            OperationBehavior::Insert,
        );
        assert!(res.is_ok());
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
        let result = store.get("key1").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value, b"key1 upserted");
    }

    #[test]
    fn test_overwrite() {
        let store = KvStore::new_temporary(".test_kv");

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
        let store = KvStore::new_temporary(".test_kv");

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
            assert!(store.get(key).unwrap().is_none());
        }

        let valid = store.get(valid_key).unwrap();
        assert!(valid.is_some());
        assert_eq!(valid.unwrap().value, b"valid data");

        let permanent = store.get(permanent_key).unwrap();
        assert!(permanent.is_some());
        assert_eq!(permanent.unwrap().value, b"permanent data");
    }
}
