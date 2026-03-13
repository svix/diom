use std::time::Duration;

use super::{LimitResponse, RateLimitRaftState, RateLimitRequest};
use crate::{RateLimitNamespace, TokenBucket};
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
    pub(crate) method: TokenBucket,
}

impl LimitOperation {
    pub fn new(
        namespace: RateLimitNamespace,
        key: String,
        tokens: u64,
        method: TokenBucket,
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
    pub allowed: bool,
    pub remaining: u64,
    pub retry_after: Option<Duration>,
}

impl LimitOperation {
    fn apply_real(
        self,
        state: &RateLimitRaftState<'_>,
        now: Timestamp,
    ) -> Result<LimitResponseData> {
        let (allowed, remaining, retry_after) = state.state.controller(self.storage_type).limit(
            now,
            self.namespace_id,
            &self.key,
            self.tokens,
            self.method,
        )?;
        Ok(LimitResponseData {
            allowed,
            remaining,
            retry_after,
        })
    }
}

impl RateLimitRequest for LimitOperation {
    fn apply(self, state: RateLimitRaftState<'_>, now: Timestamp) -> LimitResponse {
        LimitResponse(self.apply_real(&state, now))
    }
}
