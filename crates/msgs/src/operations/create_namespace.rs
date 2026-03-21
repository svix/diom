use std::time::Duration;

use coyote_namespace::{
    entities::{NamespaceName, StorageType, StreamConfig},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};

use crate::entities::{Retention, default_retention_bytes, default_retention_millis};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CreateNamespaceResponse, MsgsRaftState, MsgsRequest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNamespaceOperation {
    pub name: NamespaceName,
    pub retention: Retention,
    pub storage_type: StorageType,
}

impl CreateNamespaceOperation {
    pub fn new(name: NamespaceName, retention: Retention, storage_type: StorageType) -> Self {
        Self {
            name,
            retention,
            storage_type,
        }
    }

    async fn apply_real(
        self,
        namespace_state: &coyote_namespace::State,
        now: Timestamp,
    ) -> coyote_operations::Result<CreateNamespaceResponseData> {
        let op = CreateNamespace::new(
            self.name,
            StreamConfig {
                retention_period: Duration::from_millis(self.retention.millis.get()),
            },
            self.storage_type,
            Some(self.retention.bytes),
        );
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNamespaceResponseData {
    pub name: NamespaceName,
    pub retention: Retention,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<StreamConfig>> for CreateNamespaceResponseData {
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
            created: value.created,
            updated: value.updated,
        }
    }
}

impl MsgsRequest for CreateNamespaceOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &coyote_operations::OpContext,
    ) -> CreateNamespaceResponse {
        CreateNamespaceResponse(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
