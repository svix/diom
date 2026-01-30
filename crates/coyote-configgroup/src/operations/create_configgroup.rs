use coyote_error::Result;
use std::num::NonZeroU64;

use fjall::Database;
use fjall_utils::TableRow;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{ConfigGroupId, ModuleConfig, StorageType},
    tables::ConfigGroup,
};

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateConfigGroup<C: ModuleConfig> {
    timestamp: Timestamp,
    name: String,
    storage_type: Option<StorageType>,
    max_storage_bytes: Option<NonZeroU64>,
    config: C,
}

#[derive(Deserialize, Serialize)]
#[serde(bound = "C: ModuleConfig")]
pub struct CreateConfigGroupOutput<C: ModuleConfig> {
    pub name: String,
    pub config: C,
    pub storage_type: StorageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl<C: ModuleConfig> CreateConfigGroup<C> {
    pub fn new(
        name: String,
        config: C,
        storage_type: Option<StorageType>,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            timestamp: Timestamp::now(),
            name,
            config,
            storage_type,
            max_storage_bytes,
        }
    }

    pub fn apply_operation(
        self,
        db: &Database,
        keyspace: &fjall::Keyspace,
    ) -> Result<CreateConfigGroupOutput<C>> {
        let configgroup = match ConfigGroup::<C>::fetch(keyspace, &self.name)? {
            Some(mut configgroup) => {
                configgroup.storage_type = self.storage_type.unwrap_or(StorageType::Persistent);
                configgroup.updated_at = self.timestamp;
                configgroup.max_storage_bytes = self.max_storage_bytes;
                configgroup
            }
            None => {
                let id = ConfigGroupId::new_v4();
                ConfigGroup {
                    id,
                    name: self.name,
                    storage_type: self.storage_type.unwrap_or(StorageType::Persistent),
                    max_storage_bytes: self.max_storage_bytes,
                    created_at: self.timestamp,
                    updated_at: self.timestamp,
                    config: self.config,
                }
            }
        };

        {
            let (k1, v1) = configgroup.to_fjall_entry()?;
            let mut batch = db.batch();
            batch.insert(keyspace, k1, v1);
            batch.commit()?;
        }

        Ok(CreateConfigGroupOutput {
            name: configgroup.name,
            storage_type: configgroup.storage_type,
            max_storage_bytes: configgroup.max_storage_bytes,
            config: configgroup.config,
            created_at: configgroup.created_at,
            updated_at: configgroup.updated_at,
        })
    }
}
