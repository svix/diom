// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{num::NonZeroU64, time::Duration};

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use coyote_cache::{
    CacheModel,
    operations::{DeleteOperation, SetOperation},
};
use coyote_configgroup::entities::{EvictionPolicy, StorageType};
use coyote_derive::aide_annotate;
use coyote_error::{Error, HttpError, ResultExt};
use coyote_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::{cluster::RaftState, types::EntityKey},
    error::Result,
    v1::utils::openapi_tag,
};

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key", "value"]))]
pub struct CacheSetIn {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time to live in milliseconds
    pub ttl: u64,

    pub value: Vec<u8>,
}

impl CacheSetIn {
    fn into_model(self) -> CacheModel {
        let expiry = Timestamp::now() + Duration::from_millis(self.ttl);

        CacheModel {
            expiry: Some(expiry),
            value: self.value,
        }
    }
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheSetOut {}

#[derive(Clone, Debug, Deserialize, Validate, JsonSchema)]
#[schemars(extend("x-positional" = ["key"]))]
pub struct CacheGetIn {
    #[validate(nested)]
    pub key: EntityKey,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct CacheGetOut {
    #[validate(nested)]
    pub key: EntityKey,

    /// Time of expiry
    pub expiry: Option<Timestamp>,

    pub value: Vec<u8>,
}

impl CacheGetOut {
    fn from_model(key: EntityKey, model: CacheModel) -> Self {
        Self {
            key,
            expiry: model.expiry,
            value: model.value,
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheSetIn>,
) -> Result<MsgPackOrJson<CacheSetOut>> {
    let key_str = data.key.to_string();
    let operation = SetOperation::new(key_str, data.into_model());
    repl.client_write(operation).await.map_err_generic()?.0?;
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
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheDeleteIn>,
) -> Result<MsgPackOrJson<CacheDeleteOut>> {
    let key_str = data.key.to_string();
    let operation = DeleteOperation::new(key_str);
    repl.client_write(operation).await.map_err_generic()?.0?;
    Ok(MsgPackOrJson(CacheDeleteOut { deleted: true }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetGroupIn {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct CacheGetGroupOut {
    pub name: String,
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub eviction_policy: EvictionPolicy,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Get cache group
#[aide_annotate(op_id = "v1.cache.get_group")]
async fn cache_get_group(
    State(state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<CacheGetGroupIn>,
) -> Result<MsgPackOrJson<CacheGetGroupOut>> {
    let group = state
        .configgroup_state
        .fetch_cache_group(&data.name)?
        .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

    Ok(MsgPackOrJson(CacheGetGroupOut {
        name: group.name,
        max_storage_bytes: group.max_storage_bytes,
        storage_type: group.storage_type,
        eviction_policy: group.config.eviction_policy,
        created_at: group.created_at,
        updated_at: group.updated_at,
    }))
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
            "/cache/get-group",
            post_with(cache_get_group, cache_get_group_operation),
            &tag,
        )
        .api_route_with(
            "/cache/delete",
            post_with(cache_del, cache_del_operation),
            &tag,
        )
}
