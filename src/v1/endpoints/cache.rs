// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::State;
use coyote_derive::aide_annotate;
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::types::EntityKey,
    error::Result,
    v1::{modules::cache::CacheModel, utils::openapi_tag},
};

// Re-export types that are used in AppState
pub use crate::v1::modules::cache::CacheStore;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheSetIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time to live in milliseconds
    pub expire_in: u64,

    // FIXME: change to Bytes
    pub value: String,
}

impl CacheSetIn {
    fn into_model(self) -> CacheModel {
        let expires_at = Timestamp::now() + Duration::from_millis(self.expire_in);

        CacheModel {
            expires_at: Some(expires_at),
            value: self.value.into_bytes(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheSetOut {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheGetIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheGetOut {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time of expiry
    pub expires_at: Option<Timestamp>,

    // FIXME: change to Bytes
    pub value: String,
}

impl CacheGetOut {
    fn from_model(key: EntityKey, model: CacheModel) -> Self {
        Self {
            key,
            expires_at: model.expires_at,
            value: String::from_utf8_lossy(&model.value).into_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub struct CacheDeleteOut {
    pub deleted: bool,
}

/// Cache Set
#[aide_annotate(op_id = "v1.cache.set")]
async fn cache_set(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheSetIn>,
) -> Result<MsgPackOrJson<CacheSetOut>> {
    let mut cache_store = state.get_cache_store_by_key(&data.key.0)?;
    let key = data.key.clone();
    cache_store.set(key.as_str(), data.into_model())?;
    Ok(MsgPackOrJson(CacheSetOut {}))
}

/// Cache Get
#[aide_annotate(op_id = "v1.cache.get")]
async fn cache_get(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetIn>,
) -> Result<MsgPackOrJson<CacheGetOut>> {
    let mut cache_store = state.get_cache_store_by_key(&data.key.0)?;
    let model = cache_store
        .get(&data.key)?
        .ok_or_else(|| crate::error::HttpError::not_found(None, None))?;
    Ok(MsgPackOrJson(CacheGetOut::from_model(data.key, model)))
}

/// Cache Delete
#[aide_annotate(op_id = "v1.cache.delete")]
async fn cache_del(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheDeleteIn>,
) -> Result<MsgPackOrJson<CacheDeleteOut>> {
    let mut cache_store = state.get_cache_store_by_key(&data.key.0)?;
    cache_store.delete(&data.key)?;
    Ok(MsgPackOrJson(CacheDeleteOut { deleted: true }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Cache");

    ApiRouter::new()
        .api_route_with(
            "/cache/set",
            post_with(cache_set, cache_set_operation),
            &tag,
        )
        .api_route_with(
            "/cache/get",
            post_with(cache_get, cache_get_operation),
            &tag,
        )
        .api_route_with(
            "/cache/delete",
            post_with(cache_del, cache_del_operation),
            &tag,
        )
}
