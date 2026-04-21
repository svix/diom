use diom_authorization::api::RoleId;
use diom_core::PersistableValue;
use diom_error::Result;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

use crate::{
    State,
    operations::{AdminAuthRaftState, AdminAuthRequest, DeleteRoleResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleResponseData {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct DeleteRoleOperation {
    pub id: RoleId,
}

impl DeleteRoleOperation {
    pub fn new(id: RoleId) -> Self {
        Self { id }
    }

    async fn apply_real(self, state: &State) -> Result<DeleteRoleResponseData> {
        let success = state.controller.delete_role(&self.id).await?;
        Ok(DeleteRoleResponseData { success })
    }
}

impl AdminAuthRequest for DeleteRoleOperation {
    async fn apply(self, state: AdminAuthRaftState<'_>, _ctx: &OpContext) -> DeleteRoleResponse {
        DeleteRoleResponse::new(self.apply_real(state.state).await)
    }
}
