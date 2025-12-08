// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use diom_derive::aide_annotate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

use crate::{
    core::types::EntityKey,
    error::Result,
    v1::{
        modules::kv::{KvModel, OperationBehavior},
        utils::{openapi_tag, ValidatedJson},
    },
    AppState,
};

// Re-export types that are used in AppState
pub use crate::v1::modules::kv::worker;
pub use crate::v1::modules::kv::KvStore as KvStoreType;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetIn {
    #[validate]
    pub key: Arc<EntityKey>,
    // FIXME: validate all fields
    /// Time to live in milliseconds
    pub expire_in: u64,

    // FIXME: do we want it here? I think we probably want separate commands for insert, upsert,
    // and update? Or does it get weird?
    #[serde(default)]
    pub behavior: OperationBehavior,

    // FIXME: what to do with TTL? Does it get updated on a set, not?

    // FIXME: change to Bytes
    pub value: String,
}

impl KvSetIn {
    fn into_model(self) -> KvModel {
        let KvSetIn {
            key,
            expire_in,
            value,
            behavior: _,
        } = self;

        let expires_at = Utc::now() + chrono::Duration::milliseconds(expire_in as i64);

        KvModel {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetIn {
    #[validate]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvGetOut {
    #[validate]
    pub key: Arc<EntityKey>,

    /// Time of expiry
    pub expires_at: DateTime<Utc>,

    // FIXME: change to Bytes
    pub value: String,
}

impl From<KvModel> for KvGetOut {
    fn from(model: KvModel) -> Self {
        let KvModel {
            key,
            expires_at,
            value,
        } = model;

        Self {
            key,
            expires_at,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvDeleteIn {
    #[validate]
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

    kv_store.set(key, model, behavior)?;

    let ret = KvSetOut {};
    Ok(Json(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvGetIn>,
) -> Result<Json<KvGetOut>> {
    let model = kv_store.get(&data.key)?;
    let ret: KvGetOut = model.into();
    Ok(Json(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    State(AppState { kv_store, .. }): State<AppState>,
    ValidatedJson(data): ValidatedJson<KvDeleteIn>,
) -> Result<Json<KvDeleteOut>> {
    let deleted = kv_store.delete(&data.key);
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
