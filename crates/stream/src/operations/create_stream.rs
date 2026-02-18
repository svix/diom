use std::num::NonZeroU64;

use super::{CreateStreamResponse, StreamRaftState, StreamRequest};
use diom_configgroup::entities::StreamConfig;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamOperation {
    name: String,
    retention_period_seconds: Option<NonZeroU64>,
    max_byte_size: Option<NonZeroU64>,
}

impl CreateStreamOperation {
    pub fn new(
        name: String,
        retention_period_seconds: Option<NonZeroU64>,
        max_byte_size: Option<NonZeroU64>,
    ) -> Self {
        Self {
            name,
            retention_period_seconds,
            max_byte_size,
        }
    }

    fn apply_real(
        self,
        configgroup_state: &diom_configgroup::State,
    ) -> diom_operations::Result<CreateStreamResponseData> {
        let op = diom_configgroup::operations::create_configgroup::CreateConfigGroup::new(
            self.name,
            StreamConfig {
                retention_period_seconds: self.retention_period_seconds,
            },
            None,
            self.max_byte_size,
        );
        let out = op.apply_operation(configgroup_state)?;
        Ok(CreateStreamResponseData {
            name: out.name,
            retention_period_seconds: out.config.retention_period_seconds,
            max_byte_size: out.max_storage_bytes,
            created_at: out.created_at,
            updated_at: out.updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStreamResponseData {
    pub name: String,
    pub retention_period_seconds: Option<NonZeroU64>,
    pub max_byte_size: Option<NonZeroU64>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl StreamRequest for CreateStreamOperation {
    fn apply(self, state: StreamRaftState<'_>) -> CreateStreamResponse {
        CreateStreamResponse(self.apply_real(state.configgroup))
    }
}
