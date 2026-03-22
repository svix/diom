use crate::{
    KvNamespace, State,
    kvcontroller::{KvModelIn, OperationBehavior},
    operations::KvRaftState,
};

use super::{KvRequest, SetResponse};
use diom_core::types::{DurationMs, EntityKey};
use diom_id::NamespaceId;
use diom_operations::{OpContext, Result};
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tap::TapOptional;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResponseData {
    pub success: bool,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: EntityKey,
    model: KvModelIn,
    behavior: OperationBehavior,
}

impl SetOperation {
    pub fn new(
        namespace: KvNamespace,
        key: EntityKey,
        value: Vec<u8>,
        ttl: Option<DurationMs>,
        behavior: OperationBehavior,
        version: Option<u64>,
        now: Timestamp,
    ) -> Self {
        let expiry = ttl
            .map(|ttl| now + ttl)
            .tap_some(|v| debug_assert!(*v >= Timestamp::UNIX_EPOCH));
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            model: KvModelIn {
                value,
                expiry,
                version,
            },
            behavior,
        }
    }
}

impl SetOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<SetResponseData> {
        let result = state
            .controller(self.storage_type)
            .set_batch(
                ctx.batch.clone(),
                self.namespace_id,
                self.key,
                self.model,
                self.behavior,
                ctx.timestamp,
                ctx.log_index,
            )
            .await?;
        Ok(SetResponseData {
            success: result.success,
            version: result.version,
        })
    }
}

impl KvRequest for SetOperation {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse(self.apply_real(state.state, ctx).await)
    }
}
