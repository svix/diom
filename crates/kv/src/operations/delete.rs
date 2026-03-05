use super::{DeleteResponse, KvRequest};
use crate::{State, operations::KvRaftState};
use diom_core::types::EntityKey;
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: EntityKey,
}

impl DeleteOperation {
    pub fn new(namespace_id: NamespaceId, key: EntityKey) -> Self {
        Self { key, namespace_id }
    }
}

impl DeleteOperation {
    fn apply_real(self, state: &State) -> Result<()> {
        state.controller.delete(self.namespace_id, &self.key)?;
        Ok(())
    }
}

impl KvRequest for DeleteOperation {
    fn apply(self, state: KvRaftState<'_>) -> DeleteResponse {
        DeleteResponse(self.apply_real(state.state))
    }
}
