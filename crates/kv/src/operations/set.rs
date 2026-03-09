use std::time::Duration;

use crate::{KvNamespace, State, kvcontroller::OperationBehavior, operations::KvRaftState};

use super::{KvRequest, SetResponse};
use diom_core::types::EntityKey;
use diom_namespace::entities::NamespaceId;
use diom_operations::Result;
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
        }
    }
}

impl SetOperation {
    fn apply_real(self, state: &State, now: Timestamp) -> Result<()> {
        state.controller(self.storage_type).set(
            self.namespace_id,
            &self.key,
            self.value,
            self.expiry,
            self.behavior,
            now,
        )?;
        Ok(())
    }
}

impl KvRequest for SetOperation {
    fn apply(self, state: KvRaftState<'_>, timestamp: Timestamp) -> SetResponse {
        SetResponse(self.apply_real(state.state, timestamp))
    }
}
