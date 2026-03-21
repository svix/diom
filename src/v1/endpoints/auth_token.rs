// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::num::NonZeroU64;

use aide::axum::{ApiRouter, routing::post_with};
use axum::{Extension, extract::State};
use diom_auth_token::{
    AuthTokenNamespace,
    controller::AuthTokenModel,
    entities::{TokenHashed, TokenPlaintext},
    operations::{
        CreateAuthTokenNamespaceOperation, CreateAuthTokenOperation, DeleteAuthTokenOperation,
        DeleteResponseData, ExpireAuthTokenOperation, RotateAuthTokenOperation,
        UpdateAuthTokenOperation,
    },
};
use diom_core::types::{DurationMs, Metadata};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::{AuthTokenId, Public};
use diom_namespace::entities::{NamespaceName, StorageType};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::cluster::RaftState,
    error::Result,
    v1::utils::{ListResponse, openapi_tag},
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenOut {
    pub id: Public<AuthTokenId>,
    pub name: String,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub expiry: Option<Timestamp>,
    pub metadata: Metadata,
    pub owner_id: String,
    pub scopes: Vec<String>,
    /// Whether this token is currently enabled.
    pub enabled: bool,
}

impl From<AuthTokenModel> for AuthTokenOut {
    fn from(m: AuthTokenModel) -> Self {
        Self {
            id: m.id.public(),
            name: m.name,
            expiry: m.expiry,
            metadata: m.metadata,
            owner_id: m.owner_id,
            scopes: m.scopes,
            enabled: m.enabled,
            created: m.created,
            updated: m.updated,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenCreateIn {
    pub namespace: Option<String>,
    pub name: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    pub suffix: Option<String>,
    /// Milliseconds from now until the token expires.
    pub expiry_millis: Option<DurationMs>,
    #[serde(default)]
    pub metadata: Metadata,
    pub owner_id: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Whether the token is enabled. Defaults to `true`.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

fn default_prefix() -> String {
    "sk".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenCreateOut {
    pub id: Public<AuthTokenId>,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub token: String,
}

/// Create Auth Token
#[aide_annotate(op_id = "v1.auth_token.create")]
async fn auth_token_create(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenCreateIn>,
) -> Result<MsgPackOrJson<AuthTokenCreateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let token = TokenPlaintext::generate(&data.prefix, data.suffix.as_deref())?;
    let operation = CreateAuthTokenOperation::new(
        namespace,
        data.name,
        token.hash(),
        data.expiry_millis.map(|ms| repl.time.now() + ms),
        data.metadata,
        data.owner_id,
        data.scopes,
        data.enabled,
        repl.time.now(),
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;

    let ret = AuthTokenCreateOut {
        id: resp.model.id.public(),
        token: token.expose_plaintext_dangerously(),
        created: resp.model.created,
        updated: resp.model.updated,
    };
    Ok(MsgPackOrJson(ret))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenExpireIn {
    pub namespace: Option<String>,
    pub id: Public<AuthTokenId>,
    /// Milliseconds from now until the token expires. `None` means expire immediately.
    pub expiry_millis: Option<DurationMs>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenExpireOut {}

/// Expire Auth Token
#[aide_annotate(op_id = "v1.auth_token.expire")]
async fn auth_token_expire(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenExpireIn>,
) -> Result<MsgPackOrJson<AuthTokenExpireOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let expiry = data.expiry_millis.map(|ms| repl.time.now() + ms);
    let operation = ExpireAuthTokenOperation::new(namespace, data.id.into_inner(), expiry);
    let _ = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenExpireOut {}))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenDeleteIn {
    pub namespace: Option<String>,
    pub id: Public<AuthTokenId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenDeleteOut {
    pub success: bool,
}

impl From<DeleteResponseData> for AuthTokenDeleteOut {
    fn from(data: DeleteResponseData) -> Self {
        Self {
            success: data.success,
        }
    }
}

/// Delete Auth Token
#[aide_annotate(op_id = "v1.auth_token.delete")]
async fn auth_token_delete(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenDeleteIn>,
) -> Result<MsgPackOrJson<AuthTokenDeleteOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = DeleteAuthTokenOperation::new(namespace, data.id.into_inner());
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(resp.into()))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenVerifyIn {
    pub namespace: Option<String>,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenVerifyOut {
    pub token: Option<AuthTokenOut>,
}

/// Verify Auth Token
#[aide_annotate(op_id = "v1.auth_token.verify")]
async fn auth_token_verify(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenVerifyIn>,
) -> Result<MsgPackOrJson<AuthTokenVerifyOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let token_hashed = TokenHashed::from(data.token.as_str());
    let auth_token_state = diom_auth_token::State::init(state.do_not_use_dbs.clone())?;
    let token = auth_token_state
        .controller
        .fetch_non_expired(namespace.id, &token_hashed, repl.time.now())
        .await?;

    // FIXME: actually do something if expired, failed for other reasons, etc.

    Ok(MsgPackOrJson(AuthTokenVerifyOut {
        token: token.map(|x| x.into()),
    }))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenListIn {
    pub namespace: Option<String>,
    pub owner_id: String,
}

pub type AuthTokenListOut = ListResponse<AuthTokenOut>;

/// List Auth Tokens
#[aide_annotate(op_id = "v1.auth_token.list")]
async fn auth_token_list(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenListIn>,
) -> Result<MsgPackOrJson<AuthTokenListOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let auth_token_state = diom_auth_token::State::init(state.do_not_use_dbs.clone())?;
    let models = auth_token_state
        .controller
        .list_by_owner(namespace.id, &data.owner_id)
        .await?;

    let data = models.into_iter().map(AuthTokenOut::from).collect();

    Ok(MsgPackOrJson(ListResponse {
        data,
        iterator: None,
        prev_iterator: None,
        done: true,
    }))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenUpdateIn {
    pub namespace: Option<String>,
    pub id: Public<AuthTokenId>,
    pub name: Option<String>,
    pub expiry_millis: Option<DurationMs>,
    pub metadata: Option<Metadata>,
    pub scopes: Option<Vec<String>>,
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenUpdateOut {}

/// Update Auth Token
#[aide_annotate(op_id = "v1.auth_token.update")]
async fn auth_token_update(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenUpdateIn>,
) -> Result<MsgPackOrJson<AuthTokenUpdateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let operation = UpdateAuthTokenOperation::new(
        namespace,
        data.id.into_inner(),
        data.name,
        data.expiry_millis.map(|ms| repl.time.now() + ms),
        data.metadata,
        data.scopes,
        data.enabled,
    );
    let _resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenUpdateOut {}))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenRotateIn {
    pub namespace: Option<String>,
    pub id: Public<AuthTokenId>,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    pub suffix: Option<String>,
    /// Milliseconds from now until the old token expires. `None` means expire immediately.
    pub expiry_millis: Option<DurationMs>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenRotateOut {
    pub id: Public<AuthTokenId>,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub token: String,
}

/// Rotate Auth Token
#[aide_annotate(op_id = "v1.auth_token.rotate")]
async fn auth_token_rotate(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenRotateIn>,
) -> Result<MsgPackOrJson<AuthTokenRotateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_deref())?
        .ok_or_not_found()?;

    let token = TokenPlaintext::generate(&data.prefix, data.suffix.as_deref())?;
    let old_expiry = data.expiry_millis.map(|ms| repl.time.now() + ms);
    let operation = RotateAuthTokenOperation::new(
        namespace,
        data.id.into_inner(),
        token.hash(),
        old_expiry,
        repl.time.now(),
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    let model = resp.model.ok_or_not_found()?;

    Ok(MsgPackOrJson(AuthTokenRotateOut {
        id: model.id.public(),
        token: token.expose_plaintext_dangerously(),
        created: model.created,
        updated: model.updated,
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenGetNamespaceIn {
    pub name: NamespaceName,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenGetNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct AuthTokenCreateNamespaceIn {
    pub name: NamespaceName,
    #[serde(default)]
    pub storage_type: StorageType,
    pub max_storage_bytes: Option<NonZeroU64>,
}

impl From<AuthTokenCreateNamespaceIn> for CreateAuthTokenNamespaceOperation {
    fn from(v: AuthTokenCreateNamespaceIn) -> Self {
        CreateAuthTokenNamespaceOperation::new(v.name, v.storage_type, v.max_storage_bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenCreateNamespaceOut {
    pub name: NamespaceName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_bytes: Option<NonZeroU64>,
    pub storage_type: StorageType,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create Auth Token namespace
#[aide_annotate(op_id = "v1.auth_token.namespace.create")]
async fn auth_token_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenCreateNamespaceIn>,
) -> Result<MsgPackOrJson<AuthTokenCreateNamespaceOut>> {
    let operation = CreateAuthTokenNamespaceOperation::new(
        data.name,
        data.storage_type,
        data.max_storage_bytes,
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenCreateNamespaceOut {
        name: resp.name,
        max_storage_bytes: resp.max_storage_bytes,
        storage_type: resp.storage_type,
        created: resp.created,
        updated: resp.updated,
    }))
}

/// Get Auth Token namespace
#[aide_annotate(op_id = "v1.auth_token.namespace.get")]
async fn auth_token_get_namespace(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenGetNamespaceIn>,
) -> Result<MsgPackOrJson<AuthTokenGetNamespaceOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace_admin(&data.name)?
        .ok_or_not_found()?;

    Ok(MsgPackOrJson(AuthTokenGetNamespaceOut {
        name: namespace.name,
        max_storage_bytes: namespace.max_storage_bytes,
        storage_type: namespace.storage_type,
        created: namespace.created,
        updated: namespace.updated,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Auth Tokens");

    ApiRouter::new()
        .api_route_with(
            "/auth-token/create",
            post_with(auth_token_create, auth_token_create_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/expire",
            post_with(auth_token_expire, auth_token_expire_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/delete",
            post_with(auth_token_delete, auth_token_delete_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/verify",
            post_with(auth_token_verify, auth_token_verify_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/list",
            post_with(auth_token_list, auth_token_list_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/update",
            post_with(auth_token_update, auth_token_update_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/rotate",
            post_with(auth_token_rotate, auth_token_rotate_operation),
            &tag,
        )
        .api_route_with(
            "/auth-token/namespace/create",
            post_with(
                auth_token_create_namespace,
                auth_token_create_namespace_operation,
            ),
            &tag,
        )
        .api_route_with(
            "/auth-token/namespace/get",
            post_with(auth_token_get_namespace, auth_token_get_namespace_operation),
            &tag,
        )
}
