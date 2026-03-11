use std::time::Duration;

use super::{LimitResponse, RateLimiterRaftState, RateLimiterRequest};
use crate::{RateLimitConfig, RateLimitNamespace, RateLimitStatus};
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) tokens: u64,
    pub(crate) method: RateLimitConfig,
}

impl LimitOperation {
    pub fn new(
        namespace: RateLimitNamespace,
        key: String,
        tokens: u64,
        method: RateLimitConfig,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            tokens,
            method,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitResponseData {
    pub status: RateLimitStatus,
    pub remaining: u64,
    pub retry_after: Option<Duration>,
}

impl LimitOperation {
    fn apply_real(
        self,
        state: &RateLimiterRaftState<'_>,
        now: Timestamp,
    ) -> Result<LimitResponseData> {
        let (status, remaining, retry_after) = state.state.limit(
            now,
            self.namespace_id,
            self.storage_type,
            &self.key,
            self.tokens,
            self.method,
        )?;
        Ok(LimitResponseData {
            status,
            remaining,
            retry_after,
        })
    }
}

impl RateLimiterRequest for LimitOperation {
    fn apply(self, state: RateLimiterRaftState<'_>, now: Timestamp) -> LimitResponse {
        LimitResponse(self.apply_real(&state, now))
    }
}
