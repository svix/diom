use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{IdempotencyConfig, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{ConfigureIdempotencyResponse, IdempotencyRaftState, IdempotencyRequest};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureIdempotencyOperation {
    pub(crate) name: NamespaceName,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<ConfigureIdempotencyOperation> for CreateNamespace<IdempotencyConfig> {
    fn from(value: ConfigureIdempotencyOperation) -> Self {
        CreateNamespace::new(value.name, IdempotencyConfig {}, value.id_random_bytes)
    }
}

impl ConfigureIdempotencyOperation {
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
    ) -> Result<ConfigureIdempotencyResponseData> {
        let op: CreateNamespace<IdempotencyConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureIdempotencyResponseData {
    pub name: NamespaceName,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<IdempotencyConfig>> for ConfigureIdempotencyResponseData {
    fn from(value: CreateNamespaceOutput<IdempotencyConfig>) -> Self {
        Self {
            name: value.name,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl IdempotencyRequest for ConfigureIdempotencyOperation {
    async fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> ConfigureIdempotencyResponse {
        ConfigureIdempotencyResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
