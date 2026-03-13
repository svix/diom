use std::num::NonZeroU64;

use diom_namespace::{
    entities::{CacheConfig, EvictionPolicy, StorageType},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CacheRaftState, CacheRequest, CreateCacheResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheOperation {
    pub(crate) name: String,
    eviction_policy: EvictionPolicy,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateCacheOperation> for CreateNamespace<CacheConfig> {
    fn from(value: CreateCacheOperation) -> Self {
        CreateNamespace::new(
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
        namespace_state: &diom_namespace::State,
        now: Timestamp,
    ) -> diom_operations::Result<CreateCacheResponseData> {
        let op: CreateNamespace<CacheConfig> = self.into();
        let out = op.apply_operation(namespace_state, now)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<CacheConfig>> for CreateCacheResponseData {
    fn from(value: CreateNamespaceOutput<CacheConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            storage_type: value.storage_type,
            eviction_policy: value.config.eviction_policy,
            created: value.created_at,
            updated: value.updated_at,
        }
    }
}

impl CacheRequest for CreateCacheOperation {
    fn apply(
        self,
        state: CacheRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CreateCacheResponse {
        CreateCacheResponse(self.apply_real(state.namespace, ctx.timestamp))
    }
}
