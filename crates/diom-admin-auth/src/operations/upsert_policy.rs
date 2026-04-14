use diom_authorization::{AccessPolicyId, AccessRule};
use diom_core::PersistableValue;
use diom_error::Result;
use diom_operations::OpContext;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    controller::{AccessPolicyModel, UpsertAccessPolicyInput},
    operations::{AdminAuthRaftState, AdminAuthRequest, UpsertAccessPolicyResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertAccessPolicyResponseData {
    pub model: AccessPolicyModel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct UpsertAccessPolicyOperation {
    pub id: AccessPolicyId,
    pub description: String,
    pub rules: Vec<AccessRule>,
}

impl UpsertAccessPolicyOperation {
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
        now: Timestamp,
    ) -> Result<UpsertAccessPolicyResponseData> {
        let model = state
            .controller
            .upsert_policy(UpsertAccessPolicyInput {
                id: self.id,
                description: self.description,
                rules: self.rules,
                now,
            })
            .await?;
        Ok(UpsertAccessPolicyResponseData { model })
    }
}

impl AdminAuthRequest for UpsertAccessPolicyOperation {
    async fn apply(
        self,
        state: AdminAuthRaftState<'_>,
        ctx: &OpContext,
    ) -> UpsertAccessPolicyResponse {
        UpsertAccessPolicyResponse::new(self.apply_real(state.state, ctx.timestamp).await)
    }
}
