use super::{LimitResponse, RateLimitRaftState, RateLimitRequest};
use crate::{RateLimitNamespace, TokenBucket};
use diom_core::{PersistableValue, types::DurationMs};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct LimitOperation {
    namespace_id: NamespaceId,
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
    #[serde(rename = "retry_after_ms")]
    pub retry_after: Option<DurationMs>,
}

impl LimitOperation {
    async fn apply_real(
        self,
        state: &RateLimitRaftState<'_>,
        now: Timestamp,
    ) -> Result<LimitResponseData> {
        let (allowed, remaining, retry_after) = state
            .state
            .controller()
            .limit(now, self.namespace_id, self.key, self.tokens, self.method)
            .await?;
        Ok(LimitResponseData {
            allowed,
            remaining,
            retry_after,
        })
    }
}

impl RateLimitRequest for LimitOperation {
    async fn apply(self, state: RateLimitRaftState<'_>, ctx: &OpContext) -> LimitResponse {
        LimitResponse::new(self.apply_real(&state, ctx.timestamp).await)
    }
}
