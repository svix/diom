use super::{CacheRaftState, CacheRequest, SetResponse};
use crate::CacheModel;
use coyote_kv::kvcontroller::OperationBehavior;
use coyote_namespace::entities::NamespaceId;
use coyote_operations::Result;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    model: CacheModel,
    now: Timestamp,
}

impl SetOperation {
    pub fn new(namespace_id: NamespaceId, key: String, model: CacheModel) -> Self {
        Self {
            namespace_id,
            key,
            model,
            now: Timestamp::now(),
        }
    }
}

impl SetOperation {
    fn apply_real(self, state: &CacheRaftState<'_>) -> Result<()> {
        state.state.controller.set(
            self.namespace_id,
            &self.key,
            self.model.value,
            self.model.expiry,
            OperationBehavior::Upsert,
            self.now,
        )?;
        Ok(())
    }
}

impl CacheRequest for SetOperation {
    fn apply(self, state: CacheRaftState<'_>) -> SetResponse {
        SetResponse(self.apply_real(&state))
    }
}
