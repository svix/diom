// this file is @generated
use serde::{Deserialize, Serialize};

use super::server_state::ServerState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClusterForceNodeDowngradeOut {
    pub initial_node_state: ServerState,

    pub final_node_state: ServerState,
}

impl ClusterForceNodeDowngradeOut {
    pub fn new(initial_node_state: ServerState, final_node_state: ServerState) -> Self {
        Self {
            initial_node_state,
            final_node_state,
        }
    }
}
