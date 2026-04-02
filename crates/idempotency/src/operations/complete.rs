use super::{CompleteResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::IdempotencyNamespace;
use diom_core::types::DurationMs;
use diom_error::Result;
use diom_id::NamespaceId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    pub(crate) response: Vec<u8>,
    #[serde(rename = "ttl_ms")]
    pub(crate) ttl: DurationMs,
}

impl CompleteOperation {
    pub fn new(
        namespace: IdempotencyNamespace,
        key: String,
        response: Vec<u8>,
        ttl: DurationMs,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            response,
            ttl,
        }
    }
}

impl CompleteOperation {
    async fn apply_real(
        self,
        state: &IdempotencyRaftState<'_>,
        now: Timestamp,
        log_index: u64,
    ) -> Result<()> {
        state
            .state
            .controller()
            .complete(
                self.namespace_id,
                self.key,
                self.response,
                self.ttl,
                now,
                log_index,
            )
            .await?;

        Ok(())
    }
}

impl IdempotencyRequest for CompleteOperation {
    async fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &diom_operations::OpContext,
    ) -> CompleteResponse {
        CompleteResponse::new(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
