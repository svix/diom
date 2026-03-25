use std::num::NonZeroU64;

use coyote_error::Result;
use coyote_namespace::{
    entities::AuthTokenConfig,
    operations::create_namespace::{CreateNamespace, CreateNamespaceOutput},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::operations::{AuthTokenRaftState, AuthTokenRequest, CreateNamespaceResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthTokenNamespaceOperation {
    pub(crate) name: String,
    max_storage_bytes: Option<NonZeroU64>,
}

impl From<CreateAuthTokenNamespaceOperation> for CreateNamespace<AuthTokenConfig> {
    fn from(value: CreateAuthTokenNamespaceOperation) -> Self {
        CreateNamespace::new(value.name, AuthTokenConfig {}, value.max_storage_bytes)
    }
}

impl CreateAuthTokenNamespaceOperation {
    pub fn new(name: String, max_storage_bytes: Option<NonZeroU64>) -> Self {
        Self {
            name,
            max_storage_bytes,
        }
    }

    async fn apply_real(
        self,
        namespace_state: &coyote_namespace::State,
        now: Timestamp,
    ) -> Result<CreateAuthTokenNamespaceResponseData> {
        let op: CreateNamespace<AuthTokenConfig> = self.into();
        let out = op.apply_operation(namespace_state, now).await?;
        Ok(out.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthTokenNamespaceResponseData {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<AuthTokenConfig>> for CreateAuthTokenNamespaceResponseData {
    fn from(value: CreateNamespaceOutput<AuthTokenConfig>) -> Self {
        Self {
            name: value.name,
            max_storage_bytes: value.max_storage_bytes,
            created: value.created,
            updated: value.updated,
        }
    }
}

impl AuthTokenRequest for CreateAuthTokenNamespaceOperation {
    async fn apply(
        self,
        state: AuthTokenRaftState<'_>,
        ctx: &coyote_operations::OpContext,
    ) -> CreateNamespaceResponse {
        CreateNamespaceResponse::new(self.apply_real(state.namespace, ctx.timestamp).await)
    }
}
