use std::num::NonZeroU64;

use coyote_configgroup::{
    entities::{CacheConfig, EvictionPolicy, StorageType},
    operations::create_configgroup::{CreateConfigGroup, CreateConfigGroupOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateCacheRequest, CreateCacheResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheOperation {
    name: String,
    eviction_policy: EvictionPolicy,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateCacheOperation> for CreateConfigGroup<CacheConfig> {
    fn from(value: CreateCacheOperation) -> Self {
        CreateConfigGroup::new(
            value.name,
            CacheConfig {
                eviction_policy: value.eviction_policy,
            },
            value.storage_type,
            value.max_storage_bytes,
        )
    }
}

impl CreateCacheOperation {
    pub fn new(
        name: String,
        eviction_policy: EvictionPolicy,
        storage_type: StorageType,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            eviction_policy,
            storage_type,
            max_storage_bytes,
        }
    }

    fn apply_real(
        self,
        configgroup_state: &coyote_configgroup::State,
    ) -> coyote_operations::Result<CreateCacheResponseData> {
        let op: CreateConfigGroup<CacheConfig> = self.into();
        let out = op.apply_operation(configgroup_state)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub eviction_policy: EvictionPolicy,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<CreateConfigGroupOutput<CacheConfig>> for CreateCacheResponseData {
    fn from(value: CreateConfigGroupOutput<CacheConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            storage_type: value.storage_type,
            eviction_policy: value.config.eviction_policy,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl CreateCacheRequest for CreateCacheOperation {
    fn apply(self, state: &coyote_configgroup::State) -> CreateCacheResponse {
        CreateCacheResponse(self.apply_real(state))
    }
}
