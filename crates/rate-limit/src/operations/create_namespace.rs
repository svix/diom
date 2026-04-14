use diom_core::PersistableValue;
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{NamespaceName, RateLimitConfig},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CreateRateLimitResponse, RateLimitRaftState, RateLimitRequest};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct CreateRateLimitOperation {
    pub(crate) name: NamespaceName,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<CreateRateLimitOperation> for CreateNamespace<RateLimitConfig> {
    fn from(value: CreateRateLimitOperation) -> Self {
        CreateNamespace::new(value.name, RateLimitConfig {}, value.id_random_bytes)
    }
}

impl CreateRateLimitOperation {
    pub fn new(name: NamespaceName) -> Self {
        Self {
            name,
            id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: Timestamp,
    ) -> Result<CreateRateLimitResponseData> {
        let op: CreateNamespace<RateLimitConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRateLimitResponseData {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<RateLimitConfig>> for CreateRateLimitResponseData {
    fn from(value: CreateNamespaceOutput<RateLimitConfig>) -> Self {
        Self {
            name: value.name,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl RateLimitRequest for CreateRateLimitOperation {
    async fn apply(
        self,
        state: RateLimitRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CreateRateLimitResponse {
        CreateRateLimitResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
