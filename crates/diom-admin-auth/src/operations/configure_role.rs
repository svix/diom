use std::collections::HashMap;

use diom_authorization::{AccessPolicyId, AccessRule, RoleId};
use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    controller::{RoleModel, UpsertRoleInput},
    operations::{AdminAuthRaftState, AdminAuthRequest, ConfigureRoleResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureRoleResponseData {
    pub model: RoleModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureRoleOperation {
    pub id: RoleId,
    pub description: String,
    pub rules: Vec<AccessRule>,
    pub policies: Vec<AccessPolicyId>,
    pub context: HashMap<String, String>,
}

impl ConfigureRoleOperation {
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

    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<ConfigureRoleResponseData> {
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
        Ok(ConfigureRoleResponseData { model })
    }
}

impl AdminAuthRequest for ConfigureRoleOperation {
    async fn apply(self, state: AdminAuthRaftState<'_>, ctx: &OpContext) -> ConfigureRoleResponse {
        ConfigureRoleResponse::new(self.apply_real(state.state, ctx.timestamp).await)
    }
}
