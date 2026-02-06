use crate::core::cluster::state_machine::ClusterId;

use super::state_machine::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InternalRequest {
    SetClusterId(ClusterId),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InternalResponse {
    Ok,
}

impl InternalRequest {
    pub(super) async fn apply(self, state_machine: &mut Store) -> anyhow::Result<InternalResponse> {
        match self {
            Self::SetClusterId(uuid) => state_machine.set_cluster_id(uuid).await?,
        }
        Ok(InternalResponse::Ok)
    }
}
