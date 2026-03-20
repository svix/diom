use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult, IdempotencyState};
use coyote_core::types::DurationS;
use coyote_id::NamespaceId;
use coyote_kv::kvcontroller::{KvModelIn, OperationBehavior};
use coyote_operations::Result;
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
    fn apply_real(
        self,
        state: &IdempotencyRaftState<'_>,
        now: Timestamp,
        log_index: u64,
    ) -> Result<TryStartResponseData> {
        let expiry = now + self.ttl_seconds;

        let controller = state.state.controller(self.storage_type);

        match controller.fetch(self.namespace_id, &self.key, now)? {
            None => {
                controller.set(
                    self.namespace_id,
                    &self.key,
                    KvModelIn {
                        value: IdempotencyState::InProgress.into(),
                        expiry: Some(expiry),
                        version: None,
                    },
                    OperationBehavior::Insert,
                    now,
                    log_index,
                )?;
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
    fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        ctx: &coyote_operations::OpContext,
    ) -> TryStartResponse {
        TryStartResponse(self.apply_real(&state, ctx.timestamp, ctx.log_index))
    }
}
