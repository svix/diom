use std::num::NonZeroU64;

use diom_error::Result;
use diom_namespace::{
    entities::{CacheConfig, EvictionPolicy},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CacheRaftState, CacheRequest, CreateCacheResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheOperation {
    pub(crate) name: String,
    eviction_policy: EvictionPolicy,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateCacheOperation> for CreateNamespace<CacheConfig> {
    fn from(value: CreateCacheOperation) -> Self {
        CreateNamespace::new(
            value.name,
            CacheConfig {
                eviction_policy: value.eviction_policy,
            },
            value.max_storage_bytes,
        )
    }
}

impl CreateCacheOperation {
    pub fn new(
        name: String,
        eviction_policy: EvictionPolicy,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            eviction_policy,
            max_storage_bytes,
        }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: Timestamp,
    ) -> Result<CreateCacheResponseData> {
        let op: CreateNamespace<CacheConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCacheResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<CacheConfig>> for CreateCacheResponseData {
    fn from(value: CreateNamespaceOutput<CacheConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            eviction_policy: value.config.eviction_policy,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl CacheRequest for CreateCacheOperation {
    async fn apply(
        self,
        state: CacheRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CreateCacheResponse {
        CreateCacheResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
