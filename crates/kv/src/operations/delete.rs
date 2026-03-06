use super::{DeleteResponse, KvRequest};
use crate::{KvNamespace, State, operations::KvRaftState};
use coyote_core::types::EntityKey;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

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
    fn apply_real(self, state: &State) -> Result<()> {
        state
            .controller(self.storage_type)
            .delete(self.namespace_id, &self.key)?;
        Ok(())
    }
}

impl KvRequest for DeleteOperation {
    fn apply(self, state: KvRaftState<'_>, _timestamp: jiff::Timestamp) -> DeleteResponse {
        DeleteResponse(self.apply_real(state.state))
    }
}
