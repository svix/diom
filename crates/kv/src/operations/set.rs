use crate::{
    KvNamespace, State,
    kvcontroller::{KvModelIn, OperationBehavior},
    operations::KvRaftState,
};

use super::{KvRequest, SetResponse};
use coyote_core::types::{DurationMs, EntityKey};
use coyote_namespace::entities::NamespaceId;
use coyote_operations::{OpContext, Result};
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
    ) -> Self {
        let expiry = ttl
            .map(|ttl| Timestamp::now() + ttl)
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
    fn apply_real(self, state: &State, ctx: &OpContext) -> Result<SetResponseData> {
        let result = state.controller(self.storage_type).set(
            self.namespace_id,
            &self.key,
            self.model,
            self.behavior,
            ctx.timestamp,
            ctx.log_index,
        )?;
        Ok(SetResponseData {
            success: result.success,
            version: result.version,
        })
    }
}

impl KvRequest for SetOperation {
    fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse(self.apply_real(state.state, ctx))
    }
}
