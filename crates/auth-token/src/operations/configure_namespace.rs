use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_id::UuidV7RandomBytes;
use diom_namespace::{
    entities::{AuthTokenConfig, NamespaceName},
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use serde::{Deserialize, Serialize};

use crate::operations::{AuthTokenRaftState, AuthTokenRequest, ConfigureNamespaceResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureAuthTokenNamespaceOperation {
    pub(crate) name: NamespaceName,
    id_random_bytes: UuidV7RandomBytes,
}

impl From<ConfigureAuthTokenNamespaceOperation> for CreateNamespace<AuthTokenConfig> {
    fn from(value: ConfigureAuthTokenNamespaceOperation) -> Self {
        CreateNamespace::new(value.name, AuthTokenConfig {}, value.id_random_bytes)
    }
}

impl ConfigureAuthTokenNamespaceOperation {
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
    ) -> Result<ConfigureAuthTokenNamespaceResponseData> {
        let op: CreateNamespace<AuthTokenConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureAuthTokenNamespaceResponseData {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl From<CreateNamespaceOutput<AuthTokenConfig>> for ConfigureAuthTokenNamespaceResponseData {
    fn from(value: CreateNamespaceOutput<AuthTokenConfig>) -> Self {
        Self {
            name: value.name,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl AuthTokenRequest for ConfigureAuthTokenNamespaceOperation {
    async fn apply(
        self,
        state: AuthTokenRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> ConfigureNamespaceResponse {
        ConfigureNamespaceResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
