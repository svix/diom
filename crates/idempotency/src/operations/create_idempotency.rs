use std::num::NonZeroU64;

use diom_namespace::{
    entities::{IdempotencyConfig, StorageType},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{CreateIdempotencyRequest, CreateIdempotencyResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdempotencyOperation {
    pub(crate) name: String,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateIdempotencyOperation> for CreateNamespace<IdempotencyConfig> {
    fn from(value: CreateIdempotencyOperation) -> Self {
        CreateNamespace::new(
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
            name,
            storage_type,
            max_storage_bytes,
        }
    }

    fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
    ) -> diom_operations::Result<CreateIdempotencyResponseData> {
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

impl CreateIdempotencyRequest for CreateIdempotencyOperation {
    fn apply(self, state: &diom_namespace::State) -> CreateIdempotencyResponse {
        CreateIdempotencyResponse(self.apply_real(state))
    }
}
