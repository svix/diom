// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::{Extension, State};
use diom_authorization::RoleId;
use diom_core::types::{DurationMs, Metadata};
use diom_derive::aide_annotate;
use diom_error::{Error, ResultExt};
use diom_id::{AuthTokenId, Public};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::auth::Permissions,
    error::Result,
    v1::{
        endpoints::auth_token::{
            AuthTokenCreateIn, AuthTokenCreateOut, AuthTokenDeleteIn, AuthTokenDeleteOut,
            AuthTokenExpireIn, AuthTokenExpireOut, AuthTokenListIn, AuthTokenListOut,
            AuthTokenRotateIn, AuthTokenRotateOut, AuthTokenUpdateIn, AuthTokenUpdateOut,
            default_prefix,
        },
        utils::{ListResponse, Pagination, openapi_tag},
    },
};

use crate::core::INTERNAL_NAMESPACE;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenOut {
    pub id: Public<AuthTokenId>,
    pub name: String,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub expiry: Option<Timestamp>,
    pub role: String,
    /// Whether this token is currently enabled.
    pub enabled: bool,
}

// Create

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenCreateIn {
    pub name: String,
    pub role: String,
    /// Milliseconds from now until the token expires.
    pub expiry_ms: Option<DurationMs>,
    /// Whether the token is enabled. Defaults to `true`.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

admin_request_input!(AdminAuthTokenCreateIn);

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenCreateOut {
    pub id: Public<AuthTokenId>,
    pub token: String,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create an auth token
#[aide_annotate(op_id = "v1.admin.auth-token.create")]
async fn auth_token_create(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenCreateIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenCreateOut>> {
    let mut metadata = HashMap::new();
    metadata.insert("role".to_string(), data.role);

    let out: AuthTokenCreateOut = app_state
        .internal_call(
            "v1.auth-token.create",
            &AuthTokenCreateIn {
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                name: data.name,
                prefix: default_prefix(),
                suffix: None,
                expiry_ms: data.expiry_ms,
                metadata: Metadata(metadata),
                owner_id: RoleId::operator().0,
                scopes: vec![],
                enabled: data.enabled,
            },
        )
        .await
        .or_internal_error()?;

    Ok(MsgPackOrJson(AdminAuthTokenCreateOut {
        id: out.id,
        token: out.token,
        created: out.created,
        updated: out.updated,
    }))
}

// Expire

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenExpireIn {
    pub id: Public<AuthTokenId>,
    /// Milliseconds from now until the token expires. `None` means expire immediately.
    pub expiry_ms: Option<DurationMs>,
}

admin_request_input!(AdminAuthTokenExpireIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenExpireOut {}

/// Expire an auth token
#[aide_annotate(op_id = "v1.admin.auth-token.expire")]
async fn auth_token_expire(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenExpireIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenExpireOut>> {
    let _: AuthTokenExpireOut = app_state
        .internal_call(
            "v1.auth-token.expire",
            &AuthTokenExpireIn {
                id: data.id,
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                expiry_ms: data.expiry_ms,
            },
        )
        .await
        .or_internal_error()?;

    Ok(MsgPackOrJson(AdminAuthTokenExpireOut {}))
}

// Rotate

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenRotateIn {
    pub id: Public<AuthTokenId>,
}

admin_request_input!(AdminAuthTokenRotateIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenRotateOut {
    pub id: Public<AuthTokenId>,
    pub token: String,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Rotate an auth token, invalidating the old one and issuing a new secret
#[aide_annotate(op_id = "v1.admin.auth-token.rotate")]
async fn auth_token_rotate(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenRotateIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenRotateOut>> {
    let out: AuthTokenRotateOut = app_state
        .internal_call(
            "v1.auth-token.rotate",
            &AuthTokenRotateIn {
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                id: data.id,
                prefix: default_prefix(),
                suffix: None,
                expiry_ms: None,
            },
        )
        .await
        .or_internal_error()?;

    Ok(MsgPackOrJson(AdminAuthTokenRotateOut {
        id: out.id,
        token: out.token,
        created: out.created,
        updated: out.updated,
    }))
}

// Delete

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenDeleteIn {
    pub id: Public<AuthTokenId>,
}

admin_request_input!(AdminAuthTokenDeleteIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenDeleteOut {
    pub success: bool,
}

/// Delete an auth token
#[aide_annotate(op_id = "v1.admin.auth-token.delete")]
async fn auth_token_delete(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenDeleteIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenDeleteOut>> {
    let out: AuthTokenDeleteOut = app_state
        .internal_call(
            "v1.auth-token.delete",
            &AuthTokenDeleteIn {
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                id: data.id,
            },
        )
        .await
        .or_internal_error()?;

    Ok(MsgPackOrJson(AdminAuthTokenDeleteOut {
        success: out.success,
    }))
}

// List

#[derive(Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenListIn {
    #[serde(flatten)]
    pub pagination: Pagination<Public<AuthTokenId>>,
}

admin_request_input!(AdminAuthTokenListIn);

pub type AdminAuthTokenListOut = ListResponse<AdminAuthTokenOut>;

/// List auth tokens for a given owner
#[aide_annotate(op_id = "v1.admin.auth-token.list")]
async fn auth_token_list(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenListIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenListOut>> {
    let out: AuthTokenListOut = app_state
        .internal_call(
            "v1.auth-token.list",
            &AuthTokenListIn {
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                owner_id: RoleId::operator().0,
                pagination: data.pagination,
            },
        )
        .await
        .or_internal_error()?;

    // FIXME: pass limits on the response.
    let data = out
        .data
        .into_iter()
        .map(|t| {
            Ok(AdminAuthTokenOut {
                id: t.id,
                name: t.name,
                created: t.created,
                updated: t.updated,
                expiry: t.expiry,
                role: t
                    .metadata
                    .get("role")
                    .ok_or_else(|| Error::internal("Failed fetching role"))?
                    .to_string(),
                enabled: t.enabled,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(MsgPackOrJson(ListResponse {
        data,
        iterator: out.iterator,
        prev_iterator: out.prev_iterator,
        done: out.done,
    }))
}

// Update

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenUpdateIn {
    pub id: Public<AuthTokenId>,
    pub name: Option<String>,
    pub expiry_ms: Option<DurationMs>,
    pub enabled: Option<bool>,
}

admin_request_input!(AdminAuthTokenUpdateIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenUpdateOut {}

/// Update an auth token's properties
#[aide_annotate(op_id = "v1.admin.auth-token.update")]
async fn auth_token_update(
    State(app_state): State<AppState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAuthTokenUpdateIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenUpdateOut>> {
    let _: AuthTokenUpdateOut = app_state
        .internal_call(
            "v1.auth-token.update",
            &AuthTokenUpdateIn {
                namespace: Some(INTERNAL_NAMESPACE.to_owned()),
                id: data.id,
                name: data.name,
                expiry_ms: data.expiry_ms,
                metadata: None,
                scopes: None,
                enabled: data.enabled,
            },
        )
        .await
        .or_internal_error()?;

    Ok(MsgPackOrJson(AdminAuthTokenUpdateOut {}))
}

// Whoami

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAuthTokenWhoamiIn {}

admin_request_input!(AdminAuthTokenWhoamiIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAuthTokenWhoamiOut {
    pub role: RoleId,
}

/// Return the role of the currently authenticated token
#[aide_annotate(op_id = "v1.admin.auth-token.whoami")]
async fn auth_token_whoami(
    Extension(perms): Extension<Permissions>,
    MsgPackOrJson(_data): MsgPackOrJson<AdminAuthTokenWhoamiIn>,
) -> Result<MsgPackOrJson<AdminAuthTokenWhoamiOut>> {
    Ok(MsgPackOrJson(AdminAuthTokenWhoamiOut { role: perms.role }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Admin");

    ApiRouter::new()
        .api_route_with(
            auth_token_create_path,
            post_with(auth_token_create, auth_token_create_operation),
            &tag,
        )
        .api_route_with(
            auth_token_expire_path,
            post_with(auth_token_expire, auth_token_expire_operation),
            &tag,
        )
        .api_route_with(
            auth_token_rotate_path,
            post_with(auth_token_rotate, auth_token_rotate_operation),
            &tag,
        )
        .api_route_with(
            auth_token_delete_path,
            post_with(auth_token_delete, auth_token_delete_operation),
            &tag,
        )
        .api_route_with(
            auth_token_list_path,
            post_with(auth_token_list, auth_token_list_operation),
            &tag,
        )
        .api_route_with(
            auth_token_update_path,
            post_with(auth_token_update, auth_token_update_operation),
            &tag,
        )
        .api_route_with(
            auth_token_whoami_path,
            post_with(auth_token_whoami, auth_token_whoami_operation),
            &tag,
        )
}
