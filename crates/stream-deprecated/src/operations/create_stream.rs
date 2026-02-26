use std::{num::NonZeroU64, time::Duration};

use super::{CreateStreamResponse, StreamRaftState, StreamRequest};
use diom_namespace::{
    entities::{StorageType, StreamConfig},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamOperation {
    name: String,
    storage_type: StorageType,
    retention_period: Duration,
    max_byte_size: Option<NonZeroU64>,
}

impl CreateStreamOperation {
    pub fn new(
        name: String,
        retention_period: Duration,
        storage_type: StorageType,
        max_byte_size: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            retention_period,
            storage_type,
            max_byte_size,
        }
    }

    fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
    ) -> diom_operations::Result<CreateStreamResponseData> {
        let op: CreateNamespace<StreamConfig> = self.into();
        let out = op.apply_operation(namespace_state)?;
        Ok(out.into())
    }
}

impl From<CreateStreamOperation> for CreateNamespace<StreamConfig> {
    fn from(value: CreateStreamOperation) -> Self {
        CreateNamespace::new(
            value.name,
            StreamConfig {
                retention_period: value.retention_period,
            },
            StorageType::default(),
            value.max_byte_size,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamResponseData {
    pub name: String,
    pub retention_period: Duration,
    pub storage_type: StorageType,
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<CreateNamespaceOutput<StreamConfig>> for CreateStreamResponseData {
    fn from(value: CreateNamespaceOutput<StreamConfig>) -> Self {
        Self {
            name: value.name,
            retention_period: value.config.retention_period,
            max_byte_size: value.max_storage_bytes,
            storage_type: value.storage_type,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl StreamRequest for CreateStreamOperation {
    fn apply(self, state: StreamRaftState<'_>) -> CreateStreamResponse {
        CreateStreamResponse(self.apply_real(state.namespace))
    }
}
