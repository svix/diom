use std::time::Duration;

use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyStartResult, IdempotencyState};
use diom_kv::kvcontroller::OperationBehavior;
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    pub(crate) ttl_seconds: u64,
    now: Timestamp,
}

impl TryStartOperation {
    pub fn new(namespace_id: NamespaceId, key: String, ttl_seconds: u64) -> Self {
        Self {
            namespace_id,
            key,
            ttl_seconds,
            now: Timestamp::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartResponseData {
    pub result: IdempotencyStartResult,
}

impl TryStartOperation {
    fn apply_real(self, state: &IdempotencyRaftState<'_>) -> Result<TryStartResponseData> {
        let now = self.now;
        let expiry = now + Duration::from_secs(self.ttl_seconds);

        match state
            .state
            .controller
            .fetch(self.namespace_id, &self.key, now)?
        {
            None => {
                state.state.controller.set(
                    self.namespace_id,
                    &self.key,
                    IdempotencyState::InProgress.into(),
                    Some(expiry),
                    OperationBehavior::Insert,
                    now,
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
    fn apply(self, state: IdempotencyRaftState<'_>) -> TryStartResponse {
        TryStartResponse(self.apply_real(&state))
    }
}
