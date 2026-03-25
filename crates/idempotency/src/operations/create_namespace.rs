use std::num::NonZeroU64;

use diom_error::Result;
use diom_namespace::{
    entities::IdempotencyConfig,
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateIdempotencyResponse, IdempotencyRaftState, IdempotencyRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdempotencyOperation {
    pub(crate) name: String,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateIdempotencyOperation> for CreateNamespace<IdempotencyConfig> {
    fn from(value: CreateIdempotencyOperation) -> Self {
        CreateNamespace::new(value.name, IdempotencyConfig {}, value.max_storage_bytes)
    }
}

impl CreateIdempotencyOperation {
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
    ) -> Result<CreateIdempotencyResponseData> {
        let op: CreateNamespace<IdempotencyConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdempotencyResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<IdempotencyConfig>> for CreateIdempotencyResponseData {
    fn from(value: CreateNamespaceOutput<IdempotencyConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl IdempotencyRequest for CreateIdempotencyOperation {
    async fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CreateIdempotencyResponse {
        CreateIdempotencyResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
