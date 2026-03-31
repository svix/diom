// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use aide::axum::{ApiRouter, routing::post_with};
use axum::extract::{Extension, State};
use diom_admin_auth::{
    State as AdminAuthState,
    controller::RoleModel,
    operations::{DeleteRoleOperation, UpsertRoleOperation},
};
use diom_authorization::{AccessPolicyId, AccessRule, RoleId};
use diom_derive::aide_annotate;
use diom_error::{OptionExt, ResultExt};
use diom_proto::MsgPackOrJson;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    core::cluster::RaftState,
    error::Result,
    v1::utils::{ListResponse, ListResponseItem, Pagination, openapi_tag},
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRoleOut {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
    pub created: Timestamp,
    pub updated: Timestamp,
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

// Upsert

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminRoleUpsertIn {
    pub id: RoleId,
    pub description: String,
    #[serde(default)]
    pub rules: Vec<AccessRule>,
    #[serde(default)]
    pub policies: Vec<AccessPolicyId>,
    #[serde(default)]
    pub context: HashMap<String, String>,
}

admin_request_input!(AdminRoleUpsertIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdminRoleUpsertOut {
    pub id: RoleId,
    pub created: Timestamp,
    pub updated: Timestamp,
}

/// Create or update a role
#[aide_annotate(op_id = "v1.admin.auth-role.upsert")]
async fn role_upsert(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleUpsertIn>,
) -> Result<MsgPackOrJson<AdminRoleUpsertOut>> {
    let operation = UpsertRoleOperation::new(
        data.id,
        data.description,
        data.rules,
        data.policies,
        data.context,
    );
    let resp = repl.client_write(operation).await.or_internal_error()?.0?;
    Ok(MsgPackOrJson(AdminRoleUpsertOut {
        id: resp.model.id,
        created: resp.model.created,
        updated: resp.model.updated,
    }))
}

// Delete

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminRoleDeleteIn {
    pub id: RoleId,
}

admin_request_input!(AdminRoleDeleteIn);

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

#[derive(Clone, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminRoleGetIn {
    pub id: RoleId,
}

admin_request_input!(AdminRoleGetIn);

/// Get a role by ID
#[aide_annotate(op_id = "v1.admin.auth-role.get")]
async fn role_get(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleGetIn>,
) -> Result<MsgPackOrJson<AdminRoleOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = AdminAuthState::init(state.do_not_use_dbs.clone())?;
    let model = admin_auth_state
        .controller
        .get_role(&data.id)
        .await?
        .ok_or_not_found()?;
    Ok(MsgPackOrJson(model.into()))
}

// List

#[derive(Clone, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AdminRoleListIn {
    #[serde(flatten)]
    pub pagination: Pagination<RoleId>,
}

admin_request_input!(AdminRoleListIn);

pub type AdminRoleListOut = ListResponse<AdminRoleOut>;

/// List all roles
#[aide_annotate(op_id = "v1.admin.auth-role.list")]
async fn role_list(
    State(state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<AdminRoleListIn>,
) -> Result<MsgPackOrJson<AdminRoleListOut>> {
    repl.wait_linearizable().await.or_internal_error()?;
    let admin_auth_state = AdminAuthState::init(state.do_not_use_dbs.clone())?;
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
            role_upsert_path,
            post_with(role_upsert, role_upsert_operation),
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
