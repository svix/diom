use super::{DeleteResponse, KvRequest};
use crate::{KvNamespace, State, operations::KvRaftState};
use coyote_core::types::EntityKey;
use coyote_error::Result;
use coyote_id::NamespaceId;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: EntityKey,
}

impl DeleteOperation {
    pub fn new(namespace: KvNamespace, key: EntityKey) -> Self {
        Self {
            key,
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
        }
    }
}

impl DeleteOperation {
    async fn apply_real(self, state: &State) -> Result<DeleteResponseData> {
        let success = state
            .controller(self.storage_type)
            .delete(self.namespace_id, self.key)
            .await?;
        Ok(DeleteResponseData { success })
    }
}

impl KvRequest for DeleteOperation {
    async fn apply(
        self,
        state: KvRaftState<'_>,
        _ctx: &coyote_operations::OpContext,
    ) -> DeleteResponse {
        DeleteResponse::new(self.apply_real(state.state).await)
    }
}
