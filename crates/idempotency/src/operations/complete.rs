use std::time::Duration;

use super::{CompleteResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::{IdempotencyNamespace, IdempotencyState};
use diom_kv::kvcontroller::OperationBehavior;
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) response: Vec<u8>,
    pub(crate) ttl_seconds: u64,
    now: Timestamp,
}

impl CompleteOperation {
    pub fn new(
        namespace: IdempotencyNamespace,
        key: String,
        response: Vec<u8>,
        ttl_seconds: u64,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            response,
            ttl_seconds,
            now: Timestamp::now(),
        }
    }
}

impl CompleteOperation {
    fn apply_real(self, state: &IdempotencyRaftState<'_>) -> Result<()> {
        let expiry = self.now + Duration::from_secs(self.ttl_seconds);
        state.state.controller(StorageType::Persistent).set(
            self.namespace_id,
            &self.key,
            IdempotencyState::Completed {
                response: self.response,
            }
            .into(),
            Some(expiry),
            OperationBehavior::Upsert,
            self.now,
        )?;

        Ok(())
    }
}

impl IdempotencyRequest for CompleteOperation {
    fn apply(self, state: IdempotencyRaftState<'_>) -> CompleteResponse {
        CompleteResponse(self.apply_real(&state))
    }
}
