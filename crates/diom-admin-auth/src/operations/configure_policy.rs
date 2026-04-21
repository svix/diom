use diom_authorization::api::{AccessPolicyId, AccessRule};
use diom_core::{PersistableValue, types::UnixTimestampMs};
use diom_error::Result;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    controller::{AccessPolicyModel, UpsertAccessPolicyInput},
    operations::{AdminAuthRaftState, AdminAuthRequest, ConfigureAccessPolicyResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigureAccessPolicyResponseData {
    pub model: AccessPolicyModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ConfigureAccessPolicyOperation {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
}

impl ConfigureAccessPolicyOperation {
    pub fn new(id: AccessPolicyId, description: String, rules: Vec<AccessRule>) -> Self {
        Self {
            id,
            description,
            rules,
        }
    }

    async fn apply_real(
        self,
        state: &State,
        now: UnixTimestampMs,
    ) -> Result<ConfigureAccessPolicyResponseData> {
        let model = state
            .controller
            .upsert_policy(UpsertAccessPolicyInput {
                id: self.id,
                description: self.description,
                rules: self.rules,
                now,
            })
            .await?;
        Ok(ConfigureAccessPolicyResponseData { model })
    }
}

impl AdminAuthRequest for ConfigureAccessPolicyOperation {
    async fn apply(
        self,
        state: AdminAuthRaftState<'_>,
        ctx: &OpContext,
    ) -> ConfigureAccessPolicyResponse {
        ConfigureAccessPolicyResponse::new(self.apply_real(state.state, ctx.timestamp).await)
    }
}
