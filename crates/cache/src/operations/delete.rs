use super::{CacheRaftState, CacheRequest, DeleteResponse};
use crate::CacheNamespace;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

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
    fn apply_real(self, state: &CacheRaftState<'_>) -> Result<()> {
        state
            .state
            .controller(StorageType::Persistent)
            .delete(self.namespace_id, &self.key)?;
        Ok(())
    }
}

impl CacheRequest for DeleteOperation {
    fn apply(self, state: CacheRaftState<'_>) -> DeleteResponse {
        DeleteResponse(self.apply_real(&state))
    }
}
