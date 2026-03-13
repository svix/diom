use super::{AbortResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::IdempotencyNamespace;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
}

impl AbortOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
        }
    }
}

impl AbortOperation {
    fn apply_real(self, state: &IdempotencyRaftState<'_>) -> Result<()> {
        state
            .state
            .controller(self.storage_type)
            .delete(self.namespace_id, &self.key)?;

        Ok(())
    }
}

impl IdempotencyRequest for AbortOperation {
    fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        _ctx: &coyote_operations::OpContext,
    ) -> AbortResponse {
        AbortResponse(self.apply_real(&state))
    }
}
