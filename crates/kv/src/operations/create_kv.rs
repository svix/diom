use std::num::NonZeroU64;

use coyote_configgroup::{
    entities::{KeyValueConfig, StorageType},
    operations::create_configgroup::{CreateConfigGroup, CreateConfigGroupOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateKvRequest, CreateKvResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKvOperation {
    name: String,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateKvOperation> for CreateConfigGroup<KeyValueConfig> {
    fn from(value: CreateKvOperation) -> Self {
        CreateConfigGroup::new(
            value.name,
            KeyValueConfig {},
            value.storage_type,
            value.max_storage_bytes,
        )
    }
}

impl CreateKvOperation {
    pub fn new(
        name: String,
        storage_type: StorageType,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            storage_type,
            max_storage_bytes,
        }
    }

    fn apply_real(
        self,
        configgroup_state: &coyote_configgroup::State,
    ) -> coyote_operations::Result<CreateKvResponseData> {
        let op: CreateConfigGroup<KeyValueConfig> = self.into();
        let out = op.apply_operation(configgroup_state)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKvResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<CreateConfigGroupOutput<KeyValueConfig>> for CreateKvResponseData {
    fn from(value: CreateConfigGroupOutput<KeyValueConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            storage_type: value.storage_type,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl CreateKvRequest for CreateKvOperation {
    fn apply(self, state: &coyote_configgroup::State) -> CreateKvResponse {
        CreateKvResponse(self.apply_real(state))
    }
}
