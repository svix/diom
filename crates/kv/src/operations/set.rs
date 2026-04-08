use crate::{
    KvNamespace, State,
    kvcontroller::{KvModelIn, OperationBehavior},
    operations::KvRaftState,
};

use super::{KvRequest, SetResponse};
use diom_core::types::{DurationMs, EntityKey};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
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
    pub(crate) key: EntityKey,
    value: Vec<u8>,
    version: Option<u64>,
    ttl: Option<DurationMs>,
    behavior: OperationBehavior,
    #[serde(default)]
    use_postgres: bool,
}

impl SetOperation {
    pub fn new(
        namespace: KvNamespace,
        key: EntityKey,
        value: Vec<u8>,
        ttl: Option<DurationMs>,
        behavior: OperationBehavior,
        version: Option<u64>,
        use_postgres: bool,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            value,
            version,
            ttl,
            behavior,
            use_postgres,
        }
    }
}

impl SetOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<SetResponseData> {
        if self.use_postgres {
            crate::pg::pg_set(&self.key, &self.value).await?;
            return Ok(SetResponseData {
                success: true,
                version: ctx.log_index,
            });
        }

        let now = ctx.timestamp;
        let expiry = self
            .ttl
            .map(|ttl| now + ttl)
            .tap_some(|v| debug_assert!(*v >= Timestamp::UNIX_EPOCH));

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
            success: result.success,
            version: result.version,
        })
    }
}

impl KvRequest for SetOperation {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse::new(self.apply_real(state.state, ctx).await)
    }
}
