use std::num::NonZeroU64;

use coyote_error::Result;
use coyote_namespace::{
    Namespace,
    entities::{CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig, ModuleConfig},
};
use fjall::KeyspaceCreateOptions;
use fjall_utils::{TableRow, WriteBatchExt};
use hashlink::{LinkedHashMap, linked_hash_map::RawEntryMut};
use itertools::Itertools;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{OperationBehavior, tables::{ExpirationRow, KvPairRow}};


const EXPIRATION_BATCH_SIZE: usize = 1_000; // FIXME(@svix-lucho): make this configurable? Probably
                                            // much larger too?

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
    tables: fjall::Keyspace,
}

impl KvController {
    pub fn new(
        db: fjall::Database,
        keyspace_name: &str,
    ) -> Self {
        let tables = {
            let opts = KeyspaceCreateOptions::default();
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self {
            db,
            tables,
        }
    }

    pub fn get(&mut self, key: &str) -> Result<Option<KvModel>> {
        self.fetch_non_expired(key)
    }

    // FIXME(@svix-lucho): needs to be passed now() from the caller!
    fn fetch_non_expired(&mut self, key: &str) -> Result<Option<KvModel>> {
        let Some(data) = KvPairRow::fetch(&self.tables, KvPairRow::key_for(key))? else {
            return Ok(None);
        };

        if data.expiry.is_some_and(|exp| exp < Timestamp::now()) {
            let _ = self.delete(key);
            return Ok(None);
        }

        Ok(Some(data.into()))
    }

    fn insert_with_expiration(&mut self, key: &str, model: &KvModel) -> Result<()> {
        let mut batch = self.db.batch();

        let row = KvPairRow {
            key: key.to_string(),
            value: model.value.clone(),
            expiry: model.expiry,
        };

        batch.insert_row(&self.tables, KvPairRow::key_for(key), &row)?;

        if let Some(expiry) = model.expiry {
            let expiration_row = ExpirationRow::new(expiry, key.to_string());
            batch.insert_row(
                &self.tables,
                ExpirationRow::key_for(expiry, key),
                &expiration_row,
            )?;
        }

        batch.commit()?;

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

    pub fn delete(&self, key: &str) -> Result<()> {
        let mut batch = self.db.batch();

        if let Some(data) = KvPairRow::fetch(&self.tables, KvPairRow::key_for(key))? {
            // Delete from the expiration keyspace
            if let Some(expiry) = data.expiry {
                batch.remove_row(&self.tables, ExpirationRow::key_for(expiry, key))?;
            }
            batch.remove_row(&self.tables, KvPairRow::key_for(key))?;
        }

        batch.commit()?;

        Ok(())
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = KvPairRow>> {
        KvPairRow::values(&self.tables)
    }

    pub fn clear_expired(&mut self, now: Timestamp) -> Result<()> {
        let start = ExpirationRow::key_for(Timestamp::MIN, "").into_fjall_key();
        let end= ExpirationRow::key_for(now, "").into_fjall_key();

        for chunk in &self.tables.range(start..=end).chunks(EXPIRATION_BATCH_SIZE) {
            let mut batch = self.db.batch();
            for item in chunk {
                let k = item.key()?;
                batch.remove(&self.tables, k);
            }
            batch.commit()?;
        }

        Ok(())
    }
}
