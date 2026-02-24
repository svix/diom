use crate::core::cluster::state_machine::ClusterId;

use super::{raft::NodeId, state_machine::Store};
use openraft::LogId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InternalRequest {
    SetClusterId(ClusterId),
    RecordLogTimestamp(jiff::Timestamp),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InternalResponse {
    Ok,
}

impl InternalRequest {
    pub(super) async fn apply(
        self,
        state_machine: &mut Store,
        log_id: LogId<NodeId>,
    ) -> anyhow::Result<InternalResponse> {
        match self {
            Self::SetClusterId(uuid) => state_machine.set_cluster_id(uuid).await?,
            Self::RecordLogTimestamp(timestamp) => {
                state_machine
                    .logs
                    .record_log_timestamp(timestamp, log_id.index)
                    .await?
            }
        }
        Ok(InternalResponse::Ok)
    }
}
