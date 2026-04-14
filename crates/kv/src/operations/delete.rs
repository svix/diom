use super::{DeleteResponse, KvRequest};
use crate::{KvNamespace, State, operations::KvRaftState};
use diom_core::{PersistableValue, types::EntityKey};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct DeleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: EntityKey,
    version: Option<u64>,
}

impl DeleteOperation {
    pub fn new(namespace: KvNamespace, key: EntityKey, version: Option<u64>) -> Self {
        Self {
            key,
            namespace_id: namespace.id,
            version,
        }
    }
}

impl DeleteOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<DeleteResponseData> {
        let success = state
            .controller()
            .delete(self.namespace_id, self.key, self.version, ctx.timestamp)
            .await?;
        Ok(DeleteResponseData { success })
    }
}

impl KvRequest for DeleteOperation {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> DeleteResponse {
        DeleteResponse::new(self.apply_real(state.state, ctx).await)
    }
}
