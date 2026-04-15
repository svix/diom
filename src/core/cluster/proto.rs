//! This module contains custom protocol extensions for the interserver protocol

use std::collections::{BTreeMap, BTreeSet};

use diom_core::types::UnixTimestampMs;
use openraft::ServerState;
use serde::{Deserialize, Serialize};

use crate::cfg::PeerAddr;

use super::{
    ClusterId, LogId, NodeId,
    handle::{RequestWithContext, Response},
};

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct GetNodeIdResponse {
    pub node_id: NodeId,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct DiscoverClusterResponse {
    pub cluster_name: String,
    pub cluster_id: Option<ClusterId>,
    pub known_peers: BTreeMap<NodeId, PeerAddr>,
    pub state: ServerState,
    pub last_committed_log_id: Option<LogId>,
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
pub(super) struct AddLearnerResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UpgradeLearnerRequest {
    pub node_id: NodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UpgradeLearnerResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RemoveNodeRequest {
    pub node_id: NodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RemoveNodeResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GoAwayRequest {
    pub cluster_id: ClusterId,
    pub node_id: NodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GoAwayResponse {}

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
    pub wall_time: UnixTimestampMs,
    pub monotonic_time: UnixTimestampMs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardedWriteRequest {
    pub source_node_id: NodeId,
    pub request: RequestWithContext,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardedWriteResponse {
    pub log_id: LogId,
    pub response: Response,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastIdRequest {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastIdResponse {
    pub last_committed_log_id: Option<LogId>,
}
