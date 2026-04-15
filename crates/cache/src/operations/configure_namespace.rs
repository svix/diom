use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{CacheConfig, EvictionPolicy, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CacheRaftState, CacheRequest, ConfigureCacheResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureCacheOperation {
    pub(crate) name: NamespaceName,
    eviction_policy: EvictionPolicy,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<ConfigureCacheOperation> for CreateNamespace<CacheConfig> {
    fn from(value: ConfigureCacheOperation) -> Self {
        CreateNamespace::new(
            value.name,
            CacheConfig {
                eviction_policy: value.eviction_policy,
            },
            value.id_random_bytes,
        )
    }
}

impl ConfigureCacheOperation {
    pub fn new(name: NamespaceName, eviction_policy: EvictionPolicy) -> Self {
        Self {
            name,
            eviction_policy,
            id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: UnixTimestampMs,
    ) -> Result<ConfigureCacheResponseData> {
        let op: CreateNamespace<CacheConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureCacheResponseData {
    pub name: NamespaceName,
    pub eviction_policy: EvictionPolicy,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<CacheConfig>> for ConfigureCacheResponseData {
    fn from(value: CreateNamespaceOutput<CacheConfig>) -> Self {
        Self {
            name: value.name,
            eviction_policy: value.config.eviction_policy,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl CacheRequest for ConfigureCacheOperation {
    async fn apply(
        self,
        state: CacheRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> ConfigureCacheResponse {
        ConfigureCacheResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
