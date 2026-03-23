use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult, IdempotencyState};
use diom_core::types::DurationS;
use diom_error::Result;
use diom_id::NamespaceId;
use diom_kv::kvcontroller::{KvModelIn, OperationBehavior};
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) ttl_seconds: DurationS,
}

impl TryStartOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String, ttl_seconds: DurationS) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            ttl_seconds,
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
        let expiry = now + self.ttl_seconds;

        let controller = state.state.controller(self.storage_type);

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
        ctx: &diom_operations::OpContext,
    ) -> TryStartResponse {
        TryStartResponse::new(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
