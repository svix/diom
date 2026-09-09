//! This module contains custom protocol extensions for the interserver protocol

use std::collections::{BTreeMap, BTreeSet};

use diom_core::types::UnixTimestampMs;
use openraft::ServerState;
use serde::{Deserialize, Serialize};

use crate::cfg::PeerAddr;

use diom_operations::FeatureVersion;

use super::{
    ClusterId, LogId, NodeId,
    handle::{RequestWithContext, Response},
    version::VersionRange,
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
    /// The feature version the cluster has committed to. A joining node must support this version.
    /// Defaults to `0` from a peer that predates the version handshake.
    #[serde(default)]
    pub feature_version: FeatureVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct DiscoverResponse {
    pub node_id: NodeId,
    pub cluster: Option<DiscoverClusterResponse>,
    /// The feature versions the responding node's build supports. The leader polls this from each
    /// voter to decide when to advance the cluster feature version. Defaults to `0..=0` from a peer
    /// that predates the version handshake.
    #[serde(default)]
    pub supported_versions: VersionRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AddLearnerRequest {
    pub node_id: NodeId,
    pub address: PeerAddr,
    /// The feature versions the joining node supports. Defaults to `0..=0` from a peer that predates
    /// the version handshake.
    #[serde(default)]
    pub supported_versions: VersionRange,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AddLearnerResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UpgradeLearnerRequest {
    pub node_id: NodeId,
    /// The feature versions the promoting node supports. Defaults to `0..=0` from a peer that
    /// predates the version handshake.
    #[serde(default)]
    pub supported_versions: VersionRange,
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
    #[serde(default = "default_hop_ttl")]
    pub hop_ttl: usize,
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

const fn default_hop_ttl() -> usize {
    1
}
