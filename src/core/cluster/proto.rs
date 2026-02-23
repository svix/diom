//! This module contains custom protocol extensions for the interserver protocol

use std::{
    collections::{BTreeMap, BTreeSet},
    net::SocketAddr,
};

use openraft::{LogId, ServerState};
use serde::{Deserialize, Serialize};

use crate::core::cluster::state_machine::ClusterId;

use super::raft::NodeId;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct DiscoverClusterResponse {
    pub cluster_name: String,
    pub cluster_id: Option<ClusterId>,
    pub known_peers: BTreeMap<NodeId, Vec<SocketAddr>>,
    pub state: ServerState,
    pub last_committed_log_id: Option<LogId<NodeId>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct DiscoverResponse {
    pub node_id: NodeId,
    pub cluster: Option<DiscoverClusterResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AddLearnerRequest {
    pub node_id: NodeId,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UpgradeLearnerRequest {
    pub node_id: NodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ChangeMembershipRequest {
    pub desired_node_ids: BTreeSet<NodeId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthResponse {
    pub node_id: NodeId,
    pub last_committed_log_index: Option<u64>,
    pub server_state: ServerState,
    pub leader: Option<NodeId>,
    pub cluster_id: Option<ClusterId>,
}
