use coyote_error::Result;
use coyote_id::NamespaceId;
use std::num::NonZeroU64;

use fjall_utils::WriteBatchExt;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ModuleConfig, StorageType},
    tables::Namespace,
};

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateNamespace<C: ModuleConfig> {
    name: String,
    #[serde(default)]
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
    config: C,
}

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateNamespaceOutput<C: ModuleConfig> {
    pub name: String,
    pub config: C,
    pub storage_type: StorageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl<C: ModuleConfig + 'static> CreateNamespace<C> {
    pub fn new(
        name: String,
        config: C,
        storage_type: StorageType,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            config,
            storage_type,
            max_storage_bytes,
        }
    }

    pub async fn async_apply_operation(
        self,
        state: &State,
        timestamp: Timestamp,
    ) -> Result<CreateNamespaceOutput<C>> {
        let db = state.db().clone();
        let keyspace = state.keyspace().clone();
        tokio::task::spawn_blocking(move || self.apply_operation_inner(&db, &keyspace, timestamp))
            .await?
    }

    pub fn apply_operation(
        self,
        state: &State,
        timestamp: Timestamp,
    ) -> Result<CreateNamespaceOutput<C>> {
        let db = state.db();
        let keyspace = state.keyspace();
        self.apply_operation_inner(db, keyspace, timestamp)
    }

    fn apply_operation_inner(
        self,
        db: &fjall::Database,
        keyspace: &fjall::Keyspace,
        timestamp: Timestamp,
    ) -> Result<CreateNamespaceOutput<C>> {
        let namespace = match Namespace::<C>::fetch(keyspace, &self.name)? {
            Some(mut namespace) => {
                namespace.storage_type = self.storage_type;
                namespace.updated_at = timestamp;
                namespace.max_storage_bytes = self.max_storage_bytes;
                namespace.config = self.config;
                namespace
            }
            None => {
                let id = NamespaceId::new(timestamp);
                Namespace {
                    id,
                    name: self.name,
                    storage_type: self.storage_type,
                    max_storage_bytes: self.max_storage_bytes,
                    created_at: timestamp,
                    updated_at: timestamp,
                    config: self.config,
                }
            }
        };

        {
            let k1 = Namespace::<C>::key_for(&namespace.name);
            let mut batch = db.batch().durability(Some(fjall::PersistMode::SyncAll));
            batch.insert_row(keyspace, k1, &namespace)?;
            batch.commit()?;
        }

        Ok(CreateNamespaceOutput {
            name: namespace.name,
            storage_type: namespace.storage_type,
            max_storage_bytes: namespace.max_storage_bytes,
            config: namespace.config,
            created_at: namespace.created_at,
            updated_at: namespace.updated_at,
        })
    }
}
