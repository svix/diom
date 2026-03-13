use super::{RateLimiterRaftState, RateLimiterRequest, ResetResponse};
use crate::{RateLimitNamespace, TokenBucket};
use coyote_namespace::entities::NamespaceId;
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
    fn apply_real(self, state: &RateLimiterRaftState<'_>) -> Result<()> {
        state
            .state
            .reset(self.namespace_id, self.storage_type, &self.key)?;
        Ok(())
    }
}

impl RateLimiterRequest for ResetOperation {
    fn apply(self, state: RateLimiterRaftState<'_>, _now: jiff::Timestamp) -> ResetResponse {
        ResetResponse(self.apply_real(&state))
    }
}
