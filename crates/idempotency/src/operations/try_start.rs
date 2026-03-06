use std::time::Duration;

use super::{IdempotencyRaftState, IdempotencyRequest, TryStartResponse};
use crate::{IdempotencyNamespace, IdempotencyStartResult, IdempotencyState};
use diom_kv::kvcontroller::OperationBehavior;
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStartOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) ttl_seconds: u64,
    now: Timestamp,
}

impl TryStartOperation {
    pub fn new(namespace: IdempotencyNamespace, key: String, ttl_seconds: u64) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
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

        let controller = state.state.controller(StorageType::Persistent);

        match controller.fetch(self.namespace_id, &self.key, now)? {
            None => {
                controller.set(
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
