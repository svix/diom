use std::collections::HashMap;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::Extension;
use diom_admin_auth::{
    controller::RoleModel,
    operations::{ConfigureRoleOperation, DeleteRoleOperation},
};
use diom_authorization::{
    RequestedOperation,
    api::{AccessPolicyId, AccessRule, RoleId},
};
use diom_core::types::UnixTimestampMs;
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_id::Module;
use diom_proto::{AccessMetadata, MsgPackOrJson, RequestInput};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    core::cluster::RaftState,
    error::Result,
    v1::utils::{ListResponse, ListResponseItem, Pagination, openapi_tag},
};

fn admin_role_access_metadata<'a>(
    id: Option<&'a RoleId>,
    action: &'static str,
) -> AccessMetadata<'a> {
    AccessMetadata::RuleProtected(RequestedOperation {
        module: Module::AdminAccessPolicy,
        namespace: None,
        key: id.map(|RoleId(id)| id.as_str()),
        action,
    })
}

macro_rules! request_input {
    ($ty:ty, $action:literal) => {
        impl RequestInput for $ty {
            fn access_metadata(&self) -> AccessMetadata<'_> {
                admin_role_access_metadata(Some(&self.id), $action)
            }
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRoleOut {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

impl ListResponseItem for AdminRoleOut {
    fn id(&self) -> String {
        self.id.as_str().to_owned()
    }
}

impl From<RoleModel> for AdminRoleOut {
    fn from(model: RoleModel) -> Self {
        Self {
            id: model.id,
            description: model.description,
            rules: model.rules,
            policies: model.policies,
            context: model.context,
            created: model.created,
            updated: model.updated,
        }
    }
}

// Configure

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct AdminRoleConfigureIn {
    pub id: RoleId,
    pub description: String,
    #[serde(default)]
    pub rules: Vec<AccessRule>,
    #[serde(default)]
    pub policies: Vec<AccessPolicyId>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

request_input!(AdminRoleConfigureIn, "configure");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRoleConfigureOut {
    pub id: RoleId,
    pub created: UnixTimestampMs,
    pub updated: UnixTimestampMs,
}

/// Create or update a role
#[aide_annotate(op_id = "v1.admin.auth-role.configure")]
async fn role_configure(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleConfigureIn>,
) -> Result<MsgPackOrJson<AdminRoleConfigureOut>> {
    let operation = ConfigureRoleOperation::new(
        data.id,
        data.description,
        data.rules,
        data.policies,
        data.context,
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AdminRoleConfigureOut {
        id: resp.model.id,
        created: resp.model.created,
        updated: resp.model.updated,
    }))
}

// Delete

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct AdminRoleDeleteIn {
    pub id: RoleId,
}

request_input!(AdminRoleDeleteIn, "delete");

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRoleDeleteOut {
    pub success: bool,
}

/// Delete a role
#[aide_annotate(op_id = "v1.admin.auth-role.delete")]
async fn role_delete(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleDeleteIn>,
) -> Result<MsgPackOrJson<AdminRoleDeleteOut>> {
    let operation = DeleteRoleOperation::new(data.id);
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AdminRoleDeleteOut {
        success: resp.success,
    }))
}

// Get

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct AdminRoleGetIn {
    pub id: RoleId,
}

request_input!(AdminRoleGetIn, "get");

/// Get a role by ID
#[aide_annotate(op_id = "v1.admin.auth-role.get")]
async fn role_get(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleGetIn>,
) -> Result<MsgPackOrJson<AdminRoleOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = repl.state_machine.admin_auth_store().await;
    let model = admin_auth_state
        .controller
        .get_role(&data.id)
        .await?
        .ok_or_not_found()?;
    Ok(MsgPackOrJson(model.into()))
}

// List

#[derive(Clone, Deserialize, Serialize, JsonSchema)]
pub struct AdminRoleListIn {
    #[serde(flatten)]
    pub pagination: Pagination<RoleId>,
}

impl RequestInput for AdminRoleListIn {
    fn access_metadata(&self) -> AccessMetadata<'_> {
        admin_role_access_metadata(None, "list")
    }
}

pub type AdminRoleListOut = ListResponse<AdminRoleOut>;

/// List all roles
#[aide_annotate(op_id = "v1.admin.auth-role.list")]
async fn role_list(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleListIn>,
) -> Result<MsgPackOrJson<AdminRoleListOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = repl.state_machine.admin_auth_store().await;
    let limit = data.pagination.limit.into();
    let iterator = data.pagination.iterator;
    let models = admin_auth_state
        .controller
        .list_roles(limit + 1, iterator.clone())
        .await?;
    let items = models.into_iter().map(AdminRoleOut::from).collect();
    Ok(MsgPackOrJson(ListResponse::create(
        items,
        limit,
        iterator.map(|RoleId(i)| i),
    )))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Admin");

    ApiRouter::new()
        .api_route_with(
            role_configure_path,
            post_with(role_configure, role_configure_operation),
            &tag,
        )
        .api_route_with(
            role_delete_path,
            post_with(role_delete, role_delete_operation),
            &tag,
        )
        .api_route_with(role_get_path, post_with(role_get, role_get_operation), &tag)
        .api_route_with(
            role_list_path,
            post_with(role_list, role_list_operation),
            &tag,
        )
}
