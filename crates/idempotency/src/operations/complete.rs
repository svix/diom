use super::{CompleteResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::{IdempotencyNamespace, IdempotencyState};
use diom_core::types::{ByteString, DurationMs, Metadata};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_kv::kvcontroller::{KvModelIn, OperationBehavior};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    pub(crate) response: ByteString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) context: Option<Metadata>,
    #[serde(rename = "ttl_ms")]
    pub(crate) ttl: DurationMs,
}

impl CompleteOperation {
    pub fn new(
        namespace: IdempotencyNamespace,
        key: String,
        response: ByteString,
        context: Option<Metadata>,
        ttl: DurationMs,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            response,
            context,
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
        let expiry = now + self.ttl;
        state
            .state
            .controller()
            .set(
                self.namespace_id,
                self.key,
                KvModelIn {
                    value: IdempotencyState::Completed {
                        response: self.response,
                        context: self.context,
                    }
                    .into(),
                    expiry: Some(expiry),
                    version: None,
                },
                OperationBehavior::Upsert,
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
