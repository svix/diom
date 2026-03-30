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
}

impl From<CreateRateLimitOperation> for CreateNamespace<RateLimitConfig> {
    fn from(value: CreateRateLimitOperation) -> Self {
        CreateNamespace::new(value.name, RateLimitConfig {})
    }
}

impl CreateRateLimitOperation {
    pub fn new(name: String) -> Self {
        Self { name }
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
