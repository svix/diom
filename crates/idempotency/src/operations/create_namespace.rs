use std::num::NonZeroU64;

use coyote_namespace::{
    entities::{IdempotencyConfig, NamespaceId, StorageType, gen_namespace_id},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateIdempotencyResponse, IdempotencyRaftState, IdempotencyRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdempotencyOperation {
    pub(crate) name: String,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
    id: NamespaceId,
}

impl From<CreateIdempotencyOperation> for CreateNamespace<IdempotencyConfig> {
    fn from(value: CreateIdempotencyOperation) -> Self {
        CreateNamespace::new(
            value.id,
            value.name,
            IdempotencyConfig {},
            value.storage_type,
            value.max_storage_bytes,
        )
    }
}

impl CreateIdempotencyOperation {
    pub fn new(
        name: String,
        storage_type: StorageType,
        max_storage_bytes: Option<NonZeroU64>,
    ) -> Self {
        Self {
            id: gen_namespace_id(),
            name,
            storage_type,
            max_storage_bytes,
        }
    }

    fn apply_real(
        self,
        namespace_state: &coyote_namespace::State,
    ) -> coyote_operations::Result<CreateIdempotencyResponseData> {
        let op: CreateNamespace<IdempotencyConfig> = self.into();
        let out = op.apply_operation(namespace_state)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdempotencyResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<IdempotencyConfig>> for CreateIdempotencyResponseData {
    fn from(value: CreateNamespaceOutput<IdempotencyConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            storage_type: value.storage_type,
            created: value.created_at,
            updated: value.updated_at,
        }
    }
}

impl IdempotencyRequest for CreateIdempotencyOperation {
    fn apply(self, state: IdempotencyRaftState<'_>) -> CreateIdempotencyResponse {
        CreateIdempotencyResponse(self.apply_real(state.namespace))
    }
}
