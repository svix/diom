use std::collections::HashMap;

use diom_authorization::api::{AccessPolicyId, AccessRule, RoleId};
use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_operations::{OpContext, VERSIONS};
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
        last_configured: Option<UnixTimestampMs>,
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
                last_configured,
            })
            .await?;
        Ok(ConfigureRoleResponseData { model })
    }
}

impl AdminAuthRequest for ConfigureRoleOperation {
    async fn apply(self, state: AdminAuthRaftState<'_>, ctx: &OpContext) -> ConfigureRoleResponse {
        // Gate the new column on the committed feature version, read deterministically from the
        // apply context. Until every node has advanced to feature version VERSIONS[1] this stays None.
        let last_configured = (ctx.feature_version >= VERSIONS[1]).then_some(ctx.timestamp);
        ConfigureRoleResponse::new(
            self.apply_real(state.state, ctx.timestamp, last_configured)
                .await,
        )
    }
}
