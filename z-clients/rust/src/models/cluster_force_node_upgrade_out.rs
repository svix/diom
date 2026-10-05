// this file is @generated
use serde::{Deserialize, Serialize};

use super::server_state::ServerState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterForceNodeUpgradeOut {
    pub initial_node_state: ServerState,

    pub final_node_state: ServerState,
}

impl ClusterForceNodeUpgradeOut {
    pub fn new(initial_node_state: ServerState, final_node_state: ServerState) -> Self {
        Self {
            initial_node_state,
            final_node_state,
        }
    }
}
