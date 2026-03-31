use diom_error::Result;
use diom_id::UuidV7RandomBytes;
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
    id_random_bytes: UuidV7RandomBytes,
}

impl From<CreateCacheOperation> for CreateNamespace<CacheConfig> {
    fn from(value: CreateCacheOperation) -> Self {
        CreateNamespace::new(
            value.name,
            CacheConfig {
                eviction_policy: value.eviction_policy,
            },
            value.id_random_bytes,
        )
    }
}

impl CreateCacheOperation {
    pub fn new(name: String, eviction_policy: EvictionPolicy) -> Self {
        Self {
            name,
            eviction_policy,
            id_random_bytes: UuidV7RandomBytes::new_random(),
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
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<CacheConfig>> for CreateCacheResponseData {
    fn from(value: CreateNamespaceOutput<CacheConfig>) -> Self {
        Self {
            name: value.name,
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
