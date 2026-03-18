use super::{CompleteResponse, IdempotencyRaftState, IdempotencyRequest};
use crate::{IdempotencyNamespace, IdempotencyState};
use coyote_core::types::DurationS;
use coyote_id::NamespaceId;
use coyote_kv::kvcontroller::{KvModelIn, OperationBehavior};
use coyote_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    pub(crate) response: Vec<u8>,
    pub(crate) ttl_seconds: DurationS,
}

impl CompleteOperation {
    pub fn new(
        namespace: IdempotencyNamespace,
        key: String,
        response: Vec<u8>,
        ttl_seconds: DurationS,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            response,
            ttl_seconds,
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
        let expiry = now + self.ttl_seconds;
        state
            .state
            .controller(self.storage_type)
            .set(
                self.namespace_id,
                self.key,
                KvModelIn {
                    value: IdempotencyState::Completed {
                        response: self.response,
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
        ctx: &coyote_operations::OpContext,
    ) -> CompleteResponse {
        CompleteResponse(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
