use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{NamespaceName, RateLimitConfig},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{ConfigureRateLimitResponse, RateLimitRaftState, RateLimitRequest};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureRateLimitOperation {
    pub(crate) name: NamespaceName,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<ConfigureRateLimitOperation> for CreateNamespace<RateLimitConfig> {
    fn from(value: ConfigureRateLimitOperation) -> Self {
        CreateNamespace::new(value.name, RateLimitConfig {}, value.id_random_bytes)
    }
}

impl ConfigureRateLimitOperation {
    pub fn new(name: NamespaceName) -> Self {
        Self {
            name,
            id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: UnixTimestampMs,
    ) -> Result<ConfigureRateLimitResponseData> {
        let op: CreateNamespace<RateLimitConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureRateLimitResponseData {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<RateLimitConfig>> for ConfigureRateLimitResponseData {
    fn from(value: CreateNamespaceOutput<RateLimitConfig>) -> Self {
        Self {
            name: value.name,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl RateLimitRequest for ConfigureRateLimitOperation {
    async fn apply(
        self,
        state: RateLimitRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> ConfigureRateLimitResponse {
        ConfigureRateLimitResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
