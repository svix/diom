// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{sync::Arc, time::Duration};

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_derive::aide_annotate;
use coyote_error::ResultExt;
use coyote_kv::KvStore;
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::{cluster::RaftState, types::EntityKey},
    error::Result,
    v1::{
        modules::kv::{KvModel, OperationBehavior},
        utils::openapi_tag,
    },
};

// Re-export types that are used in AppState
pub use crate::v1::modules::kv::{KvStore as KvStoreType, worker};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct KvSetIn {
    #[validate(nested)]
    pub key: Arc<EntityKey>,

    /// Time to live in milliseconds
    #[validate(range(min = 1))]
    pub ttl: Option<u64>,

    #[serde(default)]
    pub behavior: OperationBehavior,

    pub value: Vec<u8>,
}

impl KvSetIn {
    fn into_model(self) -> KvModel {
        let KvSetIn {
            key: _,
            ttl: expire_in,
            value,
            behavior: _,
        } = self;

        let expiry = expire_in.map(|expire_in| Timestamp::now() + Duration::from_millis(expire_in));

        KvModel { expiry, value }
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
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl KvGetOut {
    fn from_model(key: Arc<EntityKey>, model: KvModel) -> Self {
        Self {
            key,
            expiry: model.expiry,
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvSetIn>,
) -> Result<MsgPackOrJson<KvSetOut>> {
    let key = data.key.0.clone();
    let behavior = data.behavior.clone();
    let model = data.into_model();

    let operation = KvStore::set_operation(key, model, behavior);
    repl.client_write(operation).await.map_err_generic()?.0?;

    let ret = KvSetOut {};
    Ok(MsgPackOrJson(ret))
}

/// KV Get
#[aide_annotate(op_id = "v1.kv.get")]
async fn kv_get(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<KvGetIn>,
) -> Result<MsgPackOrJson<KvGetOut>> {
    let mut kv_store = state.get_kv_store_by_key(&data.key.0)?;

    let model = kv_store
        .get(&data.key.0)
        .map_err(|e| crate::error::Error::generic(e))?;
    let ret = match model {
        Some(m) => KvGetOut::from_model(Arc::new(data.key), m),
        None => {
            return Err(crate::error::Error::http(
                crate::error::HttpError::not_found(None, Some("Key not found".to_string())),
            ));
        }
    };
    Ok(MsgPackOrJson(ret))
}

/// KV Delete
#[aide_annotate(op_id = "v1.kv.delete")]
async fn kv_del(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<KvDeleteIn>,
) -> Result<MsgPackOrJson<KvDeleteOut>> {
    let key = data.key.0.clone();
    let operation = KvStore::delete_operation(key);
    repl.client_write(operation).await.map_err_generic()?.0?;

    let ret = KvDeleteOut { deleted: true };
    Ok(MsgPackOrJson(ret))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Key Value Store");

    ApiRouter::new()
        .api_route_with("/kv/set", post_with(kv_set, kv_set_operation), &tag)
        .api_route_with("/kv/get", post_with(kv_get, kv_get_operation), &tag)
        .api_route_with("/kv/delete", post_with(kv_del, kv_del_operation), &tag)
}
