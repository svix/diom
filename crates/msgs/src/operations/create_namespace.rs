use std::time::Duration;

use coyote_error::Result;
use coyote_namespace::{
    entities::{MsgsConfig, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{CreateNamespaceResponse, MsgsRaftState, MsgsRequest};
use crate::entities::{Retention, default_retention_bytes, default_retention_ms};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNamespaceOperation {
    pub name: NamespaceName,
    pub retention: Retention,
}

impl CreateNamespaceOperation {
    pub fn new(name: NamespaceName, retention: Retention) -> Self {
        Self { name, retention }
    }

    async fn apply_real(
        self,
        namespace_state: &coyote_namespace::State,
        now: Timestamp,
    ) -> Result<CreateNamespaceResponseData> {
        let op = CreateNamespace::new(
            self.name,
            MsgsConfig {
                retention_period: Duration::from_millis(self.retention.ms.get()),
            },
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
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<MsgsConfig>> for CreateNamespaceResponseData {
    fn from(value: CreateNamespaceOutput<MsgsConfig>) -> Self {
        let ms = u64::try_from(value.config.retention_period.as_millis())
            .ok()
            .and_then(|ms| ms.try_into().ok())
            .unwrap_or_else(default_retention_ms);
        let bytes = value
            .max_storage_bytes
            .unwrap_or_else(default_retention_bytes);

        Self {
            name: value.name,
            retention: Retention { ms, bytes },
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
        CreateNamespaceResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
