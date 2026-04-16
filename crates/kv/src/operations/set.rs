use crate::{
    KvNamespace, State,
    kvcontroller::{KvModelIn, OperationBehavior},
    operations::KvRaftState,
};

use super::{KvRequest, SetResponse};
use diom_core::{
    PersistableValue,
    types::{ByteString, DurationMs, EntityKey},
};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResponseData {
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    pub(crate) key: EntityKey,
    value: ByteString,
    version: Option<u64>,
    ttl: Option<DurationMs>,
    behavior: OperationBehavior,
}

impl SetOperation {
    pub fn new(
        namespace: KvNamespace,
        key: EntityKey,
        value: ByteString,
        ttl: Option<DurationMs>,
        behavior: OperationBehavior,
        version: Option<u64>,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            value,
            version,
            ttl,
            behavior,
        }
    }
}

impl SetOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<SetResponseData> {
        let now = ctx.timestamp;
        let expiry = self.ttl.map(|ttl| now + ttl);

        let model = KvModelIn {
            value: self.value,
            expiry,
            version: self.version,
        };

        let result = state
            .controller()
            .set(
                self.namespace_id,
                self.key,
                model,
                self.behavior,
                ctx.timestamp,
                ctx.log_index,
            )
            .await?;
        Ok(SetResponseData {
            version: result.version,
        })
    }
}

impl KvRequest for SetOperation {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse::new(self.apply_real(state.state, ctx).await)
    }
}
