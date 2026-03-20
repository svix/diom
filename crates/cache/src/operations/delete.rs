use super::{CacheRaftState, CacheRequest, DeleteResponse};
use crate::CacheNamespace;
use diom_id::NamespaceId;
use diom_operations::{OpContext, Result};
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
    pub(crate) key: String,
}

impl DeleteOperation {
    pub fn new(namespace: CacheNamespace, key: String) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
        }
    }
}

impl DeleteOperation {
    async fn apply_real(self, state: &CacheRaftState<'_>) -> Result<DeleteResponseData> {
        let success = state
            .state
            .controller(self.storage_type)
            .delete(self.namespace_id, self.key)
            .await?;
        Ok(DeleteResponseData { success })
    }
}

impl CacheRequest for DeleteOperation {
    async fn apply(self, state: CacheRaftState<'_>, _ctx: &OpContext) -> DeleteResponse {
        DeleteResponse(self.apply_real(&state).await)
    }
}
