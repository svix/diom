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
}

impl From<CreateAuthTokenNamespaceOperation> for CreateNamespace<AuthTokenConfig> {
    fn from(value: CreateAuthTokenNamespaceOperation) -> Self {
        CreateNamespace::new(value.name, AuthTokenConfig {})
    }
}

impl CreateAuthTokenNamespaceOperation {
    pub fn new(name: String) -> Self {
        Self { name }
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
    pub created: Timestamp,
    pub updated: Timestamp,
}

impl From<CreateNamespaceOutput<AuthTokenConfig>> for CreateAuthTokenNamespaceResponseData {
    fn from(value: CreateNamespaceOutput<AuthTokenConfig>) -> Self {
        Self {
            name: value.name,
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
