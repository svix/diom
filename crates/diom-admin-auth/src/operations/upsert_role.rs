use std::collections::HashMap;

use diom_authorization::{AccessPolicyId, AccessRule, RoleId};
use diom_error::Result;
use diom_operations::OpContext;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    controller::{RoleModel, UpsertRoleInput},
    operations::{AdminAuthRaftState, AdminAuthRequest, UpsertRoleResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRoleResponseData {
    pub model: RoleModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRoleOperation {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
}

impl UpsertRoleOperation {
    pub fn new(
        id: RoleId,
        description: String,
        rules: Vec<AccessRule>,
        policies: Vec<AccessPolicyId>,
        context: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            description,
            rules,
            policies,
            context,
        }
    }

    async fn apply_real(self, state: &State, now: Timestamp) -> Result<UpsertRoleResponseData> {
        let model = state
            .controller
            .upsert_role(UpsertRoleInput {
                id: self.id,
                description: self.description,
                rules: self.rules,
                policies: self.policies,
                context: self.context,
                now,
            })
            .await?;
        Ok(UpsertRoleResponseData { model })
    }
}

impl AdminAuthRequest for UpsertRoleOperation {
    async fn apply(self, state: AdminAuthRaftState<'_>, ctx: &OpContext) -> UpsertRoleResponse {
        UpsertRoleResponse::new(self.apply_real(state.state, ctx.timestamp).await)
    }
}
