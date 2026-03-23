use super::{CacheRaftState, CacheRequest, SetResponse};
use crate::{CacheModel, CacheNamespace};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_kv::kvcontroller::{KvModelIn, OperationBehavior};
use diom_operations::OpContext;
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
    async fn apply_real(
        self,
        state: &CacheRaftState<'_>,
        now: Timestamp,
        log_index: u64,
    ) -> Result<()> {
        state
            .state
            .controller(self.storage_type)
            .set(
                self.namespace_id,
                self.key,
                KvModelIn {
                    value: self.model.value,
                    expiry: self.model.expiry,
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

impl CacheRequest for SetOperation {
    async fn apply(self, state: CacheRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse::new(self.apply_real(&state, ctx.timestamp, ctx.log_index).await)
    }
}
