use aide::{
    axum::{ApiRouter, routing::post_with},
    transform::TransformOperation,
};
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
use diom_authorization::RequestedOperation;
use diom_core::types::{DurationMs, Metadata, UnixTimestampMs};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::{AuthTokenId, Module, Public};
use diom_namespace::entities::NamespaceName;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::cluster::RaftState,
    error::Result,
    v1::utils::{ListResponse, ListResponseItem, Pagination, openapi_tag},
};

fn auth_token_access_metadata<'a>(
    ns: Option<&'a NamespaceName>,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::AuthToken,
        namespace: ns.map(|n| n.as_str()),
        key: None,
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                auth_token_access_metadata(self.namespace.as_ref(), $action)
            }
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenOut {
    pub id: Public<AuthTokenId>,
    pub name: String,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
    pub expiry: Option<UnixTimestampMs>,
    pub metadata: Metadata,
    pub owner_id: String,
    pub scopes: Vec<String>,
    /// Whether this token is currently enabled.
    pub enabled: bool,
}

impl ListResponseItem for AuthTokenOut {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl From<AuthTokenModel> for AuthTokenOut {
    fn from(m: AuthTokenModel) -> Self {
        Self {
            id: m.id.public(),
            name: m.name,
            expiry: m.expiry.map(Into::into),
            metadata: m.metadata,
            owner_id: m.owner_id,
            scopes: m.scopes,
            enabled: m.enabled,
            created: m.created.into(),
            updated: m.updated.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenCreateIn {
    pub namespace: Option<NamespaceName>,
    pub name: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    pub suffix: Option<String>,
    /// Milliseconds from now until the token expires.
    #[serde(rename = "expiry_ms")]
    pub expiry: Option<DurationMs>,
    #[serde(default)]
    pub metadata: Metadata,
    pub owner_id: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    /// Whether the token is enabled. Defaults to `true`.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

request_input!(AuthTokenCreateIn, "create");

fn default_true() -> bool {
    true
}

pub fn default_prefix() -> String {
    "sk".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenCreateOut {
    pub id: Public<AuthTokenId>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
    pub token: String,
}

/// Create Auth Token
#[aide_annotate(op_id = "v1.auth-token.create")]
async fn auth_token_create(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenCreateIn>,
) -> Result<MsgPackOrJson<AuthTokenCreateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let token = TokenPlaintext::generate(&data.prefix, data.suffix.as_deref())?;
    let operation = CreateAuthTokenOperation::new(
        namespace,
        data.name,
        token.hash(),
        data.expiry.map(|ms| repl.time.now() + ms),
        data.metadata,
        data.owner_id,
        data.scopes,
        data.enabled,
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;

    let ret = AuthTokenCreateOut {
        id: resp.model.id.public(),
        token: token.expose_plaintext_dangerously(),
        created: resp.model.created.into(),
        updated: resp.model.updated.into(),
    };
    Ok(MsgPackOrJson(ret))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenExpireIn {
    pub namespace: Option<NamespaceName>,
    pub id: Public<AuthTokenId>,
    /// Milliseconds from now until the token expires. `None` means expire immediately.
    #[serde(rename = "expiry_ms")]
    pub expiry: Option<DurationMs>,
}

request_input!(AuthTokenExpireIn, "expire");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenExpireOut {}

/// Expire Auth Token
#[aide_annotate(op_id = "v1.auth-token.expire")]
async fn auth_token_expire(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenExpireIn>,
) -> Result<MsgPackOrJson<AuthTokenExpireOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let expiry = data.expiry.map(|ms| repl.time.now() + ms);
    let operation = ExpireAuthTokenOperation::new(namespace, data.id.into_inner(), expiry);
    let _ = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenExpireOut {}))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenDeleteIn {
    pub namespace: Option<NamespaceName>,
    pub id: Public<AuthTokenId>,
}

request_input!(AuthTokenDeleteIn, "delete");

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
#[aide_annotate(op_id = "v1.auth-token.delete")]
async fn auth_token_delete(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenDeleteIn>,
) -> Result<MsgPackOrJson<AuthTokenDeleteOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = DeleteAuthTokenOperation::new(namespace, data.id.into_inner());
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(resp.into()))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenVerifyIn {
    pub namespace: Option<NamespaceName>,
    pub token: String,
}

request_input!(AuthTokenVerifyIn, "verify");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenVerifyOut {
    pub token: Option<AuthTokenOut>,
}

/// Verify Auth Token
#[aide_annotate(op_id = "v1.auth-token.verify")]
async fn auth_token_verify(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenVerifyIn>,
) -> Result<MsgPackOrJson<AuthTokenVerifyOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let token_hashed = TokenHashed::from(data.token.as_str());
    let auth_token_state = repl.state_machine.auth_token_store().await;
    let token = auth_token_state
        .controller
        .fetch_non_expired(namespace.id, &token_hashed, repl.time.now())
        .await?;

    // FIXME: actually do something if expired, failed for other reasons, etc.

    Ok(MsgPackOrJson(AuthTokenVerifyOut {
        token: token.map(|x| x.into()),
    }))
}

#[derive(Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenListIn {
    pub namespace: Option<NamespaceName>,
    pub owner_id: String,
    #[serde(flatten)]
    pub pagination: Pagination<Public<AuthTokenId>>,
}

request_input!(AuthTokenListIn, "list");

pub type AuthTokenListOut = ListResponse<AuthTokenOut>;

/// List Auth Tokens
#[aide_annotate(op_id = "v1.auth-token.list")]
async fn auth_token_list(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenListIn>,
) -> Result<MsgPackOrJson<AuthTokenListOut>> {
    repl.wait_linearizable().await.or_internal_error()?;

    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let limit = data.pagination.limit.0 as usize;
    let iterator = data.pagination.iterator.map(|id| id.into_inner());

    let auth_token_state = repl.state_machine.auth_token_store().await;
    let models = auth_token_state
        .controller
        .list_by_owner(namespace.id, &data.owner_id, limit + 1, iterator)
        .await?;

    let items = models.into_iter().map(AuthTokenOut::from).collect();

    Ok(MsgPackOrJson(ListResponse::create(
        items,
        limit,
        iterator.map(|x| x.public().to_string()),
    )))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenUpdateIn {
    pub namespace: Option<NamespaceName>,
    pub id: Public<AuthTokenId>,
    pub name: Option<String>,
    #[serde(rename = "expiry_ms")]
    pub expiry: Option<DurationMs>,
    pub metadata: Option<Metadata>,
    pub scopes: Option<Vec<String>>,
    pub enabled: Option<bool>,
}

request_input!(AuthTokenUpdateIn, "update");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenUpdateOut {}

/// Update Auth Token
#[aide_annotate(op_id = "v1.auth-token.update")]
async fn auth_token_update(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenUpdateIn>,
) -> Result<MsgPackOrJson<AuthTokenUpdateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let operation = UpdateAuthTokenOperation::new(
        namespace,
        data.id.into_inner(),
        data.name,
        data.expiry.map(|ms| repl.time.now() + ms),
        data.metadata,
        data.scopes,
        data.enabled,
    );
    let _resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenUpdateOut {}))
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthTokenRotateIn {
    pub namespace: Option<NamespaceName>,
    pub id: Public<AuthTokenId>,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    pub suffix: Option<String>,
    /// Milliseconds from now until the old token expires. `None` means expire immediately.
    #[serde(rename = "expiry_ms")]
    pub expiry: Option<DurationMs>,
}

request_input!(AuthTokenRotateIn, "rotate");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthTokenRotateOut {
    pub id: Public<AuthTokenId>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
    pub token: String,
}

/// Rotate Auth Token
#[aide_annotate(op_id = "v1.auth-token.rotate")]
async fn auth_token_rotate(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenRotateIn>,
) -> Result<MsgPackOrJson<AuthTokenRotateOut>> {
    let namespace: AuthTokenNamespace = state
        .namespace_state
        .fetch_namespace(data.namespace.as_ref())?
        .ok_or_not_found()?;

    let token = TokenPlaintext::generate(&data.prefix, data.suffix.as_deref())?;
    let old_expiry = data.expiry.map(|ms| repl.time.now() + ms);
    let operation =
        RotateAuthTokenOperation::new(namespace, data.id.into_inner(), token.hash(), old_expiry);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    let model = resp.model.ok_or_not_found()?;

    Ok(MsgPackOrJson(AuthTokenRotateOut {
        id: model.id.public(),
        token: token.expose_plaintext_dangerously(),
        created: model.created.into(),
        updated: model.updated.into(),
    }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenGetNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(AuthTokenGetNamespaceIn, "get");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenGetNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
pub(crate) struct AuthTokenCreateNamespaceIn {
    pub name: NamespaceName,
}

namespace_request_input!(AuthTokenCreateNamespaceIn, "create");

impl From<AuthTokenCreateNamespaceIn> for CreateAuthTokenNamespaceOperation {
    fn from(v: AuthTokenCreateNamespaceIn) -> Self {
        CreateAuthTokenNamespaceOperation::new(v.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate, JsonSchema)]
struct AuthTokenCreateNamespaceOut {
    pub name: NamespaceName,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Create Auth Token namespace
#[aide_annotate(op_id = "v1.auth-token.namespace.create")]
async fn auth_token_create_namespace(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AuthTokenCreateNamespaceIn>,
) -> Result<MsgPackOrJson<AuthTokenCreateNamespaceOut>> {
    let operation = CreateAuthTokenNamespaceOperation::new(data.name);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AuthTokenCreateNamespaceOut {
        name: resp.name,
        created: resp.created.into(),
        updated: resp.updated.into(),
    }))
}

/// Get Auth Token namespace
#[aide_annotate(op_id = "v1.auth-token.namespace.get")]
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
        created: namespace.created.into(),
        updated: namespace.updated.into(),
    }))
}

pub fn router() -> ApiRouter<AppState> {
    fn internal(
        transform: impl FnOnce(TransformOperation<'_>) -> TransformOperation<'_>,
    ) -> impl FnOnce(TransformOperation<'_>) -> TransformOperation<'_> {
        move |op| {
            let mut op = transform(op);
            op.inner_mut()
                .extensions
                .insert("x-internal".to_owned(), true.into());
            op
        }
    }

    let tag = openapi_tag("Auth Tokens");

    ApiRouter::new()
        .api_route_with(
            auth_token_create_path,
            post_with(auth_token_create, internal(auth_token_create_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_expire_path,
            post_with(auth_token_expire, internal(auth_token_expire_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_delete_path,
            post_with(auth_token_delete, internal(auth_token_delete_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_verify_path,
            post_with(auth_token_verify, internal(auth_token_verify_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_list_path,
            post_with(auth_token_list, internal(auth_token_list_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_update_path,
            post_with(auth_token_update, internal(auth_token_update_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_rotate_path,
            post_with(auth_token_rotate, internal(auth_token_rotate_operation)),
            &tag,
        )
        .api_route_with(
            auth_token_create_namespace_path,
            post_with(
                auth_token_create_namespace,
                internal(auth_token_create_namespace_operation),
            ),
            &tag,
        )
        .api_route_with(
            auth_token_get_namespace_path,
            post_with(
                auth_token_get_namespace,
                internal(auth_token_get_namespace_operation),
            ),
            &tag,
        )
}
