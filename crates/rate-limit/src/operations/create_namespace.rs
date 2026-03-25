use std::num::NonZeroU64;

use diom_error::Result;
use diom_namespace::{
    entities::RateLimitConfig,
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CreateRateLimitResponse, RateLimitRaftState, RateLimitRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRateLimitOperation {
    pub(crate) name: String,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateRateLimitOperation> for CreateNamespace<RateLimitConfig> {
    fn from(value: CreateRateLimitOperation) -> Self {
        CreateNamespace::new(value.name, RateLimitConfig {}, value.max_storage_bytes)
    }
}

impl CreateRateLimitOperation {
    pub fn new(name: String, max_storage_bytes: Option<NonZeroU64>) -> Self {
        Self {
            name,
            max_storage_bytes,
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
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<RateLimitConfig>> for CreateRateLimitResponseData {
    fn from(value: CreateNamespaceOutput<RateLimitConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
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
