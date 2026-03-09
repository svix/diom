use std::time::Duration;

use crate::{KvNamespace, State, kvcontroller::OperationBehavior, operations::KvRaftState};

use super::{KvRequest, SetResponse};
use coyote_core::types::EntityKey;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: EntityKey,
    expiry: Option<Timestamp>,
    value: Vec<u8>,
    behavior: OperationBehavior,
    now: Timestamp,
}

impl SetOperation {
    pub fn new(
        namespace: KvNamespace,
        key: EntityKey,
        value: Vec<u8>,
        ttl: Option<u64>,
        behavior: OperationBehavior,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            expiry: ttl.map(|ttl| Timestamp::now() + Duration::from_millis(ttl)),
            value,
            behavior,
            now: Timestamp::now(),
        }
    }
}

impl SetOperation {
    fn apply_real(self, state: &State) -> Result<()> {
        state.controller(self.storage_type).set(
            self.namespace_id,
            &self.key,
            self.value,
            self.expiry,
            self.behavior,
            self.now,
        )?;
        Ok(())
    }
}

impl KvRequest for SetOperation {
    fn apply(self, state: KvRaftState<'_>) -> SetResponse {
        SetResponse(self.apply_real(state.state))
    }
}
