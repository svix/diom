use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult};
use coyote_core::types::DurationMs;
use coyote_error::Result;
use coyote_id::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    #[serde(rename = "ttl_ms")]
    pub(crate) ttl: DurationMs,
}

impl TryStartOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String, ttl: DurationMs) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            ttl,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartResponseData {
    pub result: IdempotencyStartResult,
}

impl TryStartOperation {
    async fn apply_real(
        self,
        state: &IdempotencyRaftState<'_>,
        now: Timestamp,
        log_index: u64,
    ) -> Result<TryStartResponseData> {
        let result = state
            .state
            .controller()
            .try_start(self.namespace_id, self.key, self.ttl, now, log_index)
            .await?;

        Ok(TryStartResponseData { result })
    }
}

impl IdempotencyRequest for TryStartOperation {
    async fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &coyote_operations::OpContext,
    ) -> TryStartResponse {
        TryStartResponse::new(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
