use super::{AbortResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::IdempotencyNamespace;
use diom_core::PersistableValue;
use diom_error::Result;
use diom_id::NamespaceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct AbortOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
}

impl AbortOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
        }
    }
}

impl AbortOperation {
    async fn apply_real(
        self,
        state: &IdempotencyRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> Result<()> {
        state
            .state
            .controller()
            .delete(self.namespace_id, self.key, None, ctx.timestamp)
            .await?;

        Ok(())
    }
}

impl IdempotencyRequest for AbortOperation {
    async fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> AbortResponse {
        AbortResponse::new(self.apply_real(&state, ctx).await)
    }
}
