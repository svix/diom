use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult, storage::IdempotencyState};
use diom_core::{
    PersistableValue,
    types::{DurationMs, UnixTimestampMs},
};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_kv::kvcontroller::{KvModelIn, OperationBehavior};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    #[serde(rename = "lock_period_ms")]
    pub(crate) lock_period: DurationMs,
}

impl TryStartOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String, lock_period: DurationMs) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            lock_period,
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
        now: UnixTimestampMs,
        log_index: u64,
    ) -> Result<TryStartResponseData> {
        let expiry = now + self.lock_period;

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
                    IdempotencyState::Completed { response, context } => {
                        IdempotencyStartResult::Completed { response, context }
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
        ctx: &diom_operations::OpContext,
    ) -> TryStartResponse {
        TryStartResponse::new(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
