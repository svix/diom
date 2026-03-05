use super::{CacheRaftState, CacheRequest, DeleteResponse};
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
}

impl DeleteOperation {
    pub fn new(namespace_id: NamespaceId, key: String) -> Self {
        Self { namespace_id, key }
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
