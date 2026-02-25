//! This module contains custom protocol extensions for the interserver protocol

use std::collections::{BTreeMap, BTreeSet};

use openraft::{LogId, ServerState};
use serde::{Deserialize, Serialize};

use super::handle::{Request, Response};
use crate::{cfg::PeerAddr, core::cluster::state_machine::ClusterId};

use super::NodeId;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct DiscoverClusterResponse {
    pub cluster_name: String,
    pub cluster_id: Option<ClusterId>,
    pub known_peers: BTreeMap<NodeId, PeerAddr>,
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
    pub address: PeerAddr,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardedWriteRequest {
    pub source_node_id: NodeId,
    pub request: Request,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardedWriteResponse {
    pub log_id: LogId<NodeId>,
    pub response: Response,
}
