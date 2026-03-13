use std::num::NonZeroU64;

use diom_namespace::{
    entities::{RateLimitNamespaceConfig, StorageType},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CreateRateLimitResponse, RateLimitRaftState, RateLimitRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRateLimitOperation {
    pub(crate) name: String,
    storage_type: StorageType,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateRateLimitOperation> for CreateNamespace<RateLimitNamespaceConfig> {
    fn from(value: CreateRateLimitOperation) -> Self {
        CreateNamespace::new(
            value.name,
            RateLimitNamespaceConfig {},
            value.storage_type,
            value.max_storage_bytes,
        )
    }
}

impl CreateRateLimitOperation {
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
        now: Timestamp,
    ) -> diom_operations::Result<CreateRateLimitResponseData> {
        let op: CreateNamespace<RateLimitNamespaceConfig> = self.into();
        let out = op.apply_operation(namespace_state, now)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRateLimitResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<RateLimitNamespaceConfig>> for CreateRateLimitResponseData {
    fn from(value: CreateNamespaceOutput<RateLimitNamespaceConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            storage_type: value.storage_type,
            created: value.created_at,
            updated: value.updated_at,
        }
    }
}

impl RateLimitRequest for CreateRateLimitOperation {
    fn apply(self, state: RateLimitRaftState<'_>, now: Timestamp) -> CreateRateLimitResponse {
        CreateRateLimitResponse(self.apply_real(state.namespace, now))
    }
}
