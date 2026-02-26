use std::time::Duration;

use diom_namespace::{
    entities::{StorageType, StreamConfig},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use stream_internals::entities::{Retention, default_retention_bytes, default_retention_millis};

use super::{CreateMsgTopicResponse, StreamRaftState, StreamRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMsgTopicOperation {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
}

impl CreateMsgTopicOperation {
    pub fn new(name: String, retention: Retention, storage_type: StorageType) -> Self {
        Self {
            name,
            retention,
            storage_type,
        }
    }

    fn apply_real(
        self,
        configgroup_state: &diom_namespace::State,
    ) -> diom_operations::Result<CreateMsgTopicResponseData> {
        let op = CreateNamespace::new(
            self.name,
            StreamConfig {
                retention_period: Duration::from_millis(self.retention.millis.get()),
            },
            self.storage_type,
            Some(self.retention.bytes),
        );
        let out = op.apply_operation(configgroup_state)?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMsgTopicResponseData {
    pub name: String,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<CreateNamespaceOutput<StreamConfig>> for CreateMsgTopicResponseData {
    fn from(value: CreateNamespaceOutput<StreamConfig>) -> Self {
        let millis = u64::try_from(value.config.retention_period.as_millis())
            .ok()
            .and_then(|ms| ms.try_into().ok())
            .unwrap_or_else(default_retention_millis);
        let bytes = value
            .max_storage_bytes
            .unwrap_or_else(default_retention_bytes);

        Self {
            name: value.name,
            retention: Retention { millis, bytes },
            storage_type: value.storage_type,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl StreamRequest for CreateMsgTopicOperation {
    fn apply(self, state: StreamRaftState<'_>) -> CreateMsgTopicResponse {
        CreateMsgTopicResponse(self.apply_real(state.namespace))
    }
}
