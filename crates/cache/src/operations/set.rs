use super::{CacheRaftState, CacheRequest, SetResponse};
use crate::{CacheModel, CacheNamespace};
use coyote_kv::kvcontroller::OperationBehavior;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use fjall_utils::StorageType;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    storage_type: StorageType,
    pub(crate) key: String,
    model: CacheModel,
}

impl SetOperation {
    pub fn new(namespace: CacheNamespace, key: String, model: CacheModel) -> Self {
        Self {
            namespace_id: namespace.id,
            storage_type: namespace.storage_type,
            key,
            model,
        }
    }
}

impl SetOperation {
    fn apply_real(self, state: &CacheRaftState<'_>, now: Timestamp) -> Result<()> {
        state.state.controller(self.storage_type).set(
            self.namespace_id,
            &self.key,
            self.model.value,
            self.model.expiry,
            OperationBehavior::Upsert,
            now,
        )?;
        Ok(())
    }
}

impl CacheRequest for SetOperation {
    fn apply(self, state: CacheRaftState<'_>, now: Timestamp) -> SetResponse {
        SetResponse(self.apply_real(&state, now))
    }
}
