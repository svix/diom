use serde::{Deserialize, Serialize};

use crate::{
    AuthTokenNamespace, State,
    operations::{AuthTokenRaftState, AuthTokenRequest, DeleteResponse},
};
use coyote_id::{AuthTokenId, NamespaceId};
use coyote_operations::{OpContext, Result};
use fjall_utils::StorageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAuthTokenOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub id: AuthTokenId,
}

impl DeleteAuthTokenOperation {
    pub fn new(namespace: AuthTokenNamespace, id: AuthTokenId) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
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
        DeleteResponse(self.apply_real(state.state).await)
    }
}
