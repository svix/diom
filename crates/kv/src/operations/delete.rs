use super::{DeleteResponse, KvRequest};
use crate::{KvNamespace, State, operations::KvRaftState};
use diom_core::types::EntityKey;
use diom_id::NamespaceId;
use diom_operations::Result;
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
        _ctx: &diom_operations::OpContext,
    ) -> DeleteResponse {
        DeleteResponse(self.apply_real(state.state).await)
    }
}
