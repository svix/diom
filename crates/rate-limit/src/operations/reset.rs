use super::{RateLimitRaftState, RateLimitRequest, ResetResponse};
use crate::{RateLimitNamespace, TokenBucket};
use coyote_id::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) method: TokenBucket,
}

impl ResetOperation {
    pub fn new(namespace: RateLimitNamespace, key: String, method: TokenBucket) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            method,
        }
    }
}

impl ResetOperation {
    async fn apply_real(self, state: &RateLimitRaftState<'_>) -> Result<()> {
        state
            .state
            .controller(self.storage_type)
            .reset(self.namespace_id, self.key)
            .await?;
        Ok(())
    }
}

impl RateLimitRequest for ResetOperation {
    async fn apply(
        self,
        state: RateLimitRaftState<'_>,
        _ctx: &coyote_operations::OpContext,
    ) -> ResetResponse {
        ResetResponse(self.apply_real(&state).await)
    }
}
