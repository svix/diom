use super::{CacheRaftState, CacheRequest, SetResponse};
use crate::CacheNamespace;
use diom_core::types::{ByteString, DurationMs};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    namespace_id: NamespaceId,
    pub(crate) key: String,
    ttl: DurationMs,
    value: ByteString,
}

impl SetOperation {
    pub fn new(namespace: CacheNamespace, key: String, ttl: DurationMs, value: ByteString) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            ttl,
            value,
        }
    }
}

impl SetOperation {
    async fn apply_real(self, state: &CacheRaftState<'_>, now: Timestamp) -> Result<()> {
        let expiry = now + self.ttl;
        debug_assert!(expiry >= Timestamp::UNIX_EPOCH);

        state
            .state
            .controller()
            .set(self.namespace_id, self.key, self.value, expiry)
            .await
    }
}

impl CacheRequest for SetOperation {
    async fn apply(self, state: CacheRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse::new(self.apply_real(&state, ctx.timestamp).await)
    }
}
