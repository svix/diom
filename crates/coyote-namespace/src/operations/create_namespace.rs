use coyote_error::Result;
use std::num::NonZeroU64;

use fjall_utils::WriteBatchExt;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    entities::{ModuleConfig, NamespaceId, StorageType},
    tables::Namespace,
};

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateNamespace<C: ModuleConfig> {
    timestamp: Timestamp,
    name: String,
    #[serde(default)]
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
    config: C,
    id: NamespaceId,
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

impl<C: ModuleConfig> CreateNamespace<C> {
    pub fn new(
        id: NamespaceId,
        name: String,
        config: C,
        storage_type: StorageType,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            timestamp: Timestamp::now(),
            id,
            name,
            config,
            storage_type,
            max_storage_bytes,
        }
    }

    pub fn apply_operation(self, state: &State) -> Result<CreateNamespaceOutput<C>> {
        let db = state.db();
        let keyspace = state.keyspace();
        let namespace = match Namespace::<C>::fetch(keyspace, &self.name)? {
            Some(mut namespace) => {
                namespace.storage_type = self.storage_type;
                namespace.updated_at = self.timestamp;
                namespace.max_storage_bytes = self.max_storage_bytes;
                namespace.config = self.config;
                namespace
            }
            None => Namespace {
                id: self.id,
                name: self.name,
                storage_type: self.storage_type,
                max_storage_bytes: self.max_storage_bytes,
                created_at: self.timestamp,
                updated_at: self.timestamp,
                config: self.config,
            },
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
