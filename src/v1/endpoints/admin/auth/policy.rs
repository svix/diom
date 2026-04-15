use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::Extension;
use diom_admin_auth::{
    controller::AccessPolicyModel,
    operations::{ConfigureAccessPolicyOperation, DeleteAccessPolicyOperation},
};
use diom_authorization::{AccessPolicyId, AccessRule, RequestedOperation};
use diom_core::types::UnixTimestampMs;
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::Module;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::validate_access_rule_list;
use crate::{
    AppState,
    core::cluster::RaftState,
    error::Result,
    v1::utils::{ListResponse, ListResponseItem, Pagination, openapi_tag},
};

fn admin_access_policy_access_metadata<'a>(
    id: Option<&'a AccessPolicyId>,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::AdminAccessPolicy,
        namespace: None,
        key: id.map(|AccessPolicyId(id)| id.as_str()),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                admin_access_policy_access_metadata(Some(&self.id), $action)
            }
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAccessPolicyOut {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl ListResponseItem for AdminAccessPolicyOut {
    fn id(&self) -> String {
        self.id.as_str().to_owned()
    }
}

impl From<AccessPolicyModel> for AdminAccessPolicyOut {
    fn from(model: AccessPolicyModel) -> Self {
        Self {
            id: model.id,
            description: model.description,
            rules: model.rules,
            created: model.created,
            updated: model.updated,
        }
    }
}

// Configure

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAccessPolicyConfigureIn {
    pub id: AccessPolicyId,
    pub description: String,
    #[serde(default)]
    #[validate(custom(function = "validate_access_rule_list"))]
    pub rules: Vec<AccessRule>,
}

request_input!(AdminAccessPolicyConfigureIn, "configure");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAccessPolicyConfigureOut {
    pub id: AccessPolicyId,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Create or update an access policy
#[aide_annotate(op_id = "v1.admin.auth-policy.configure")]
async fn access_policy_configure(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAccessPolicyConfigureIn>,
) -> Result<MsgPackOrJson<AdminAccessPolicyConfigureOut>> {
    let operation = ConfigureAccessPolicyOperation::new(data.id, data.description, data.rules);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AdminAccessPolicyConfigureOut {
        id: resp.model.id,
        created: resp.model.created,
        updated: resp.model.updated,
    }))
}

// Delete

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAccessPolicyDeleteIn {
    pub id: AccessPolicyId,
}

request_input!(AdminAccessPolicyDeleteIn, "delete");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminAccessPolicyDeleteOut {
    pub success: bool,
}

/// Delete an access policy
#[aide_annotate(op_id = "v1.admin.auth-policy.delete")]
async fn access_policy_delete(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAccessPolicyDeleteIn>,
) -> Result<MsgPackOrJson<AdminAccessPolicyDeleteOut>> {
    let operation = DeleteAccessPolicyOperation::new(data.id);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AdminAccessPolicyDeleteOut {
        success: resp.success,
    }))
}

// Get

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAccessPolicyGetIn {
    pub id: AccessPolicyId,
}

request_input!(AdminAccessPolicyGetIn, "get");

/// Get an access policy by ID
#[aide_annotate(op_id = "v1.admin.auth-policy.get")]
async fn access_policy_get(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAccessPolicyGetIn>,
) -> Result<MsgPackOrJson<AdminAccessPolicyOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = repl.state_machine.admin_auth_store().await;
    let model = admin_auth_state
        .controller
        .get_policy(&data.id)
        .await?
        .ok_or_not_found()?;
    Ok(MsgPackOrJson(model.into()))
}

// List

#[derive(Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminAccessPolicyListIn {
    #[serde(flatten)]
    pub pagination: Pagination<AccessPolicyId>,
}

impl RequestInput for AdminAccessPolicyListIn {
    fn access_metadata(&self) -> AccessMetadata<'_> {
        admin_access_policy_access_metadata(None, "list")
    }
}

pub type AdminAccessPolicyListOut = ListResponse<AdminAccessPolicyOut>;

/// List all access policies
#[aide_annotate(op_id = "v1.admin.auth-policy.list")]
async fn access_policy_list(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminAccessPolicyListIn>,
) -> Result<MsgPackOrJson<AdminAccessPolicyListOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = repl.state_machine.admin_auth_store().await;
    let limit = data.pagination.limit.into();
    let iterator = data.pagination.iterator;
    let models = admin_auth_state
        .controller
        .list_policies(limit + 1, iterator.clone())
        .await?;
    let items = models.into_iter().map(Into::into).collect();
    Ok(MsgPackOrJson(ListResponse::create(
        items,
        limit,
        iterator.map(|AccessPolicyId(i)| i),
    )))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Admin");

    ApiRouter::new()
        .api_route_with(
            access_policy_configure_path,
            post_with(access_policy_configure, access_policy_configure_operation),
            &tag,
        )
        .api_route_with(
            access_policy_delete_path,
            post_with(access_policy_delete, access_policy_delete_operation),
            &tag,
        )
        .api_route_with(
            access_policy_get_path,
            post_with(access_policy_get, access_policy_get_operation),
            &tag,
        )
        .api_route_with(
            access_policy_list_path,
            post_with(access_policy_list, access_policy_list_operation),
            &tag,
        )
}
