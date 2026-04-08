use std::borrow::Cow;

use crate::{
    KvNamespace, State,
    kvcontroller::{KvModelIn, OperationBehavior},
    operations::KvRaftState,
};

use super::{KvRequest, SetResponse};
use diom_core::types::{DurationMs, EntityKey, Yoke};
use diom_error::Result;
use diom_id::NamespaceId;
use diom_operations::OpContext;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use tap::TapOptional;
use yoke::Yokeable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResponseData {
    pub success: bool,
    pub version: u64,
}

#[derive(Clone, Debug)]
pub struct SetOperation(pub Yoke<SetOperationInner<'static>>);

impl<'de> Deserialize<'de> for SetOperation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // need a type-erased backing cart now I guess
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Yokeable)]
pub struct SetOperationInner<'a> {
    namespace_id: NamespaceId,
    pub(crate) key: EntityKey,
    value: Cow<'a, [u8]>,
    version: Option<u64>,
    ttl: Option<DurationMs>,
    behavior: OperationBehavior,
}

impl<'a> SetOperationInner<'a> {
    pub fn new(
        namespace: KvNamespace,
        key: EntityKey,
        value: &'a [u8],
        ttl: Option<DurationMs>,
        behavior: OperationBehavior,
        version: Option<u64>,
    ) -> Self {
        Self {
            namespace_id: namespace.id,
            key,
            value: value.into(),
            version,
            ttl,
            behavior,
        }
    }
}

impl SetOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<SetResponseData> {
        let data = self.0.get();
        let now = ctx.timestamp;
        let expiry = data
            .ttl
            .map(|ttl| now + ttl)
            .tap_some(|v| debug_assert!(*v >= Timestamp::UNIX_EPOCH));

        let model = KvModelIn {
            value: data.value,
            expiry,
            version: data.version,
        };

        let result = state
            .controller()
            .set(
                data.namespace_id,
                data.key,
                model,
                data.behavior,
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

impl KvRequest for SetOperation<'static> {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> SetResponse {
        SetResponse::new(self.apply_real(state.state, ctx).await)
    }
}
