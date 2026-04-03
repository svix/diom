use super::{CacheRaftState, CacheRequest, DeleteResponse};
use crate::CacheNamespace;
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
}

impl DeleteOperation {
    pub fn new(namespace: CacheNamespace, key: String) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
        }
    }
}

impl DeleteOperation {
    async fn apply_real(
        self,
        state: &CacheRaftState<'_>,
        ctx: &OpContext,
    ) -> Result<DeleteResponseData> {
        let success = state
            .state
            .controller()
            .delete(self.namespace_id, self.key, None, ctx.timestamp)
            .await?;
        Ok(DeleteResponseData { success })
    }
}

impl CacheRequest for DeleteOperation {
    async fn apply(self, state: CacheRaftState<'_>, ctx: &OpContext) -> DeleteResponse {
        DeleteResponse::new(self.apply_real(&state, ctx).await)
    }
}
