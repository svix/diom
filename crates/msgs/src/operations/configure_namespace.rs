use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{MsgsConfig, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use serde::{Deserialize, Serialize};

use super::{ConfigureNamespaceResponse, MsgsRaftState, MsgsRequest};
use crate::entities::Retention;

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureNamespaceOperation {
    pub name: NamespaceName,
    pub retention: Retention,
    id_random_bytes: UuidV7RandomBytes,
}

impl ConfigureNamespaceOperation {
    pub fn new(name: NamespaceName, retention: Retention) -> Self {
        Self {
            name,
            retention,
            id_random_bytes: UuidV7RandomBytes::new_random(),
        }
    }

    async fn apply_real(
        self,
        namespace_state: &diom_namespace::State,
        now: UnixTimestampMs,
    ) -> Result<ConfigureNamespaceResponseData> {
        let op = CreateNamespace::new(
            self.name,
            MsgsConfig {
                retention_period: self.retention.period,
                retention_bytes: self.retention.size_bytes,
            },
            self.id_random_bytes,
        );
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureNamespaceResponseData {
    pub name: NamespaceName,
    pub retention: Retention,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl From<CreateNamespaceOutput<MsgsConfig>> for ConfigureNamespaceResponseData {
    fn from(value: CreateNamespaceOutput<MsgsConfig>) -> Self {
        Self {
            name: value.name,
            retention: Retention {
                period: value.config.retention_period,
                size_bytes: value.config.retention_bytes,
            },
            created: value.created,
            updated: value.updated,
        }
    }
}

impl MsgsRequest for ConfigureNamespaceOperation {
    async fn apply(
        self,
        state: MsgsRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> ConfigureNamespaceResponse {
        ConfigureNamespaceResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
