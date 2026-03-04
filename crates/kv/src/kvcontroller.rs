use coyote_error::Result;
use coyote_namespace::entities::NamespaceId;
use fjall::{KeyspaceCreateOptions, KvSeparationOptions};
use fjall_utils::{TableRow, WriteBatchExt};
use itertools::Itertools;
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
}

impl KvController {
    pub fn new(db: fjall::Database, keyspace_name: &str) -> Self {
        let tables = {
            let opts = KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()));
            db.keyspace(keyspace_name, || opts).unwrap()
        };

        Self {
            db,
            keyspace: tables,
        }
    }

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

        let row = KvPairRow {
            key: key.to_string(),
            value,
            expiry,
        };

        batch.insert_row(&self.keyspace, KvPairRow::key_for(namespace_id, key), &row)?;

        if let Some(expiry) = expiry {
            let expiration_row = ExpirationRow::new();
            batch.insert_row(
                &self.keyspace,
                ExpirationRow::key_for(namespace_id, expiry, key),
                &expiration_row,
            )?;
        }

        batch.commit()?;

        Ok(())
    }

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

    pub fn delete(&self, namespace_id: NamespaceId, key: &str) -> Result<()> {
        let mut batch = self.db.batch();

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

    pub fn clear_expired(&mut self, now: Timestamp) -> Result<()> {
        let start = ExpirationRow::key_for(NamespaceId::nil(), Timestamp::MIN, "").into_fjall_key();
        let end = ExpirationRow::key_for(NamespaceId::max(), now, "").into_fjall_key();

        for chunk in &self
            .keyspace
            .range(start..=end)
            .chunks(EXPIRATION_BATCH_SIZE)
        {
            let mut batch = self.db.batch();
            for item in chunk {
                let k = item.key()?;
                let (namespace_id, main_key) = ExpirationRow::extract_key_from_fjall_key(&k)?;
                batch.remove_row(&self.keyspace, KvPairRow::key_for(namespace_id, main_key))?;

                batch.remove(&self.keyspace, k);
            }
            batch.commit()?;
        }

        Ok(())
    }
}
