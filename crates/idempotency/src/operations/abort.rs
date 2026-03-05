use super::{AbortResponse, IdempotencyRaftState, IdempotencyRequest};
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
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
            .controller
            .delete(self.namespace_id, &self.key)?;

        Ok(())
    }
}

impl IdempotencyRequest for AbortOperation {
    fn apply(self, state: IdempotencyRaftState<'_>) -> AbortResponse {
        AbortResponse(self.apply_real(&state))
    }
}
