use super::{RateLimitRaftState, RateLimitRequest, ResetResponse};
use crate::{RateLimitNamespace, TokenBucket};
use diom_core::PersistableValue;
use diom_error::Result;
use diom_id::NamespaceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ResetOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    pub(crate) method: TokenBucket,
}

impl ResetOperation {
    pub fn new(namespace: RateLimitNamespace, key: String, method: TokenBucket) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            method,
        }
    }
}

impl ResetOperation {
    async fn apply_real(self, state: &RateLimitRaftState<'_>) -> Result<()> {
        state
            .state
            .controller()
            .reset(self.namespace_id, self.key)
            .await?;
        Ok(())
    }
}

impl RateLimitRequest for ResetOperation {
    async fn apply(
        self,
        state: RateLimitRaftState<'_>,
        _ctx: &diom_operations::OpContext,
    ) -> ResetResponse {
        ResetResponse::new(self.apply_real(&state).await)
    }
}
