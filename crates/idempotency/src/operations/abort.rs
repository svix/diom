use super::{AbortResponse, IdempotencyRaftState, IdempotencyRequest};
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
}

impl AbortOperation {
    pub fn new(namespace_id: NamespaceId, key: String) -> Self {
        Self { namespace_id, key }
    }
}

impl AbortOperation {
    fn apply_real(self, state: &IdempotencyRaftState<'_>) -> Result<()> {
        state
            .state
            .controller(StorageType::Persistent)
            .delete(self.namespace_id, &self.key)?;

        Ok(())
    }
}

impl IdempotencyRequest for AbortOperation {
    fn apply(self, state: IdempotencyRaftState<'_>) -> AbortResponse {
        AbortResponse(self.apply_real(&state))
    }
}
