// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Json, extract::State};
use diom_derive::aide_annotate;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use validator::Validate;

use crate::{
    AppState,
    core::types::EntityKey,
    error::Result,
    v1::{
        modules::kv::{Kv2Model, OperationBehavior},
        utils::{ValidatedJson, openapi_tag},
    },
};

// Re-export types that are used in AppState
pub use crate::v1::modules::kv::Kv2Store as KvStoreType;
pub use crate::v1::modules::kv::worker;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetIn {
    #[validate(nested)]
    pub key: Arc<EntityKey>,
    // FIXME: validate all fields
    /// Time to live in milliseconds
    pub expire_in: u64,

    // FIXME: do we want it here? I think we probably want separate commands for insert, upsert,
    // and update? Or does it get weird?
    #[serde(default)]
    pub behavior: OperationBehavior,

    // FIXME: what to do with TTL? Does it get updated on a set, not?
    pub value: Vec<u8>,
}

impl KvSetIn {
    fn into_model(self) -> Kv2Model {
        let expires_at = Timestamp::now() + Duration::from_millis(self.expire_in);

        Kv2Model {
            expires_at: Some(expires_at),
            value: self.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    #[validate(nested)]
    pub key: Arc<EntityKey>,

    /// Time of expiry
    pub expires_at: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl KvGetOut {
    fn from_model(key: EntityKey, model: Kv2Model) -> Self {
        Self {
            key: Arc::new(key),
            expires_at: model.expires_at,
            value: model.value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteIn {
    #[validate(nested)]
    pub key: Arc<EntityKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteOut {
    pub deleted: bool,
}

/// KV Set
#[aide_annotate(op_id = "v1.kv.set")]
async fn kv_set(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvSetIn>,
) -> Result<Json<KvSetOut>> {
    let key = data.key.clone();
    let behavior = data.behavior.clone();
    let model = data.into_model();

    kv_store
        .set(&key, &model, behavior)
        .map_err(|e| crate::error::Error::generic(e))?;

    let ret = KvSetOut {};
    Ok(Json(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvGetIn>,
) -> Result<Json<KvGetOut>> {
    let model = kv_store
        .get(&data.key)
        .map_err(|e| crate::error::Error::generic(e))?
        .ok_or_else(|| crate::error::Error::http(crate::error::HttpError::not_found(None, None)))?;
    let ret = KvGetOut::from_model(data.key, model);
    Ok(Json(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvDeleteIn>,
) -> Result<Json<KvDeleteOut>> {
    let deleted = kv_store.delete(&data.key).is_ok();
    let ret = KvDeleteOut { deleted };
    Ok(Json(ret))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route_with("/kv/set", post_with(kv_set, kv_set_operation), &tag)
        .api_route_with("/kv/get", post_with(kv_get, kv_get_operation), &tag)
        .api_route_with("/kv/delete", post_with(kv_del, kv_del_operation), &tag)
}
