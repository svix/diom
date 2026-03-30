use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult, IdempotencyState};
use coyote_core::types::DurationMs;
use coyote_error::Result;
use coyote_id::NamespaceId;
use coyote_kv::kvcontroller::{KvModelIn, OperationBehavior};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    pub(crate) ttl_ms: DurationMs,
}

impl TryStartOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String, ttl_ms: DurationMs) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            ttl_ms,
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
        let expiry = now + self.ttl_ms;

        let controller = state.state.controller();

        match controller
            .fetch(self.namespace_id, self.key.clone(), now)
            .await?
        {
            None => {
                controller
                    .set(
                        self.namespace_id,
                        self.key,
                        KvModelIn {
                            value: IdempotencyState::InProgress.into(),
                            expiry: Some(expiry),
                            version: None,
                        },
                        OperationBehavior::Insert,
                        now,
                        log_index,
                    )
                    .await?;
                Ok(TryStartResponseData {
                    result: IdempotencyStartResult::Started,
                })
            }
            Some(kv_model) => {
                let idem_state: IdempotencyState = kv_model.value.into();
                let result = match idem_state {
                    IdempotencyState::InProgress => IdempotencyStartResult::Locked,
                    IdempotencyState::Completed { response } => {
                        IdempotencyStartResult::Completed { response }
                    }
                };
                Ok(TryStartResponseData { result })
            }
        }
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
