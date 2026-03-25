use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    operations::{AuthTokenRaftState, AuthTokenRequest, DeleteResponse},
};
use diom_error::Result;
use diom_id::{AuthTokenId, NamespaceId};
use diom_operations::OpContext;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthTokenOperation {
    namespace_id: NamespaceId,
    pub id: AuthTokenId,
}

impl DeleteAuthTokenOperation {
    pub fn new(namespace: AuthTokenNamespace, id: AuthTokenId) -> Self {
        Self {
            namespace_id: namespace.id,
            id,
        }
    }

    async fn apply_real(self, state: &State) -> Result<DeleteResponseData> {
        let success = state.controller.delete(self.namespace_id, self.id).await?;
        Ok(DeleteResponseData { success })
    }
}

impl AuthTokenRequest for DeleteAuthTokenOperation {
    async fn apply(self, state: AuthTokenRaftState<'_>, _ctx: &OpContext) -> DeleteResponse {
        DeleteResponse::new(self.apply_real(state.state).await)
    }
}
