use diom_authorization::AccessPolicyId;
use diom_error::Result;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    operations::{AdminAuthRaftState, AdminAuthRequest, DeleteAccessPolicyResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccessPolicyResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccessPolicyOperation {
    pub id: AccessPolicyId,
}

impl DeleteAccessPolicyOperation {
    pub fn new(id: AccessPolicyId) -> Self {
        Self { id }
    }

    async fn apply_real(self, state: &State) -> Result<DeleteAccessPolicyResponseData> {
        let success = state.controller.delete_policy(&self.id).await?;
        Ok(DeleteAccessPolicyResponseData { success })
    }
}

impl AdminAuthRequest for DeleteAccessPolicyOperation {
    async fn apply(
        self,
        state: AdminAuthRaftState<'_>,
        _ctx: &OpContext,
    ) -> DeleteAccessPolicyResponse {
        DeleteAccessPolicyResponse::new(self.apply_real(state.state).await)
    }
}
