// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::{Extension, extract::State};
use coyote_derive::aide_annotate;
use coyote_error::{Error, ResultExt};
use coyote_proto::MsgPackOrJson;
use futures_util::StreamExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState, RaftState,
    core::cluster::{ClusterId, Node, NodeId},
    error::Result,
    v1::utils::openapi_tag,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, JsonSchema)]
pub enum ServerState {
    Leader,
    Follower,
    Learner,
    Candidate,
    Shutdown,
    Unknown,
}

impl From<openraft::ServerState> for ServerState {
    fn from(value: openraft::ServerState) -> Self {
        match value {
            openraft::ServerState::Leader => Self::Leader,
            openraft::ServerState::Follower => Self::Follower,
            openraft::ServerState::Learner => Self::Learner,
            openraft::ServerState::Candidate => Self::Candidate,
            openraft::ServerState::Shutdown => Self::Shutdown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct NodeStatusOut {
    /// A unique ID representing this node.
    ///
    /// This will never change unless the node is erased and reset
    node_id: NodeId,
    /// The advertised inter-server (cluster) address of this node.
    address: String,
    /// The last known state of this node
    state: ServerState,
    /// The index of the last log applied on this node
    last_committed_log_index: Option<u64>,
    /// The raft term of the last committed leadership
    last_committed_term: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ClusterStatusOut {
    /// The unique ID of this cluster.
    ///
    /// This value is populated on cluster initialization and will never change.
    pub cluster_id: Option<ClusterId>,
    /// The name of this cluster (as defined in the config)
    ///
    /// This value is not replicated and should only be used for debugging.
    pub cluster_name: Option<String>,
    /// The unique ID of the node servicing this request
    pub this_node_id: NodeId,
    /// The cluster state of the node servicing this request
    pub this_node_state: ServerState,
    /// The timestamp of the last transaction committed on this node
    pub this_node_last_committed_timestamp: jiff::Timestamp,
    /// The last snapshot taken on this node
    pub this_node_last_snapshot_id: Option<String>,
    /// A list of all nodes known to be in the cluster
    pub nodes: Vec<NodeStatusOut>,
}

#[derive(Debug)]
struct PartialNodeStatus {
    node_id: NodeId,
    node: Node,
    state: ServerState,
}

/// Get information about the current cluster
#[aide_annotate(op_id = "v1.admin.cluster.status")]
async fn cluster_status(
    State(app_state): State<AppState>,
    Extension(repl): Extension<RaftState>,
) -> Result<MsgPackOrJson<ClusterStatusOut>> {
    // TODO: move all of this out of the endpoint at some point
    let leader_id = repl.raft.current_leader().await;
    let (this_node_state, this_last_committed_log_index, pnodes) = repl
        .raft
        .with_raft_state(move |s| {
            let this_node_state = s.server_state.into();
            let committed = s.committed;
            let members = s.membership_state.effective().membership();
            let voters = members.voter_ids().collect::<BTreeSet<NodeId>>();
            let learners = members.voter_ids().collect::<BTreeSet<NodeId>>();
            let nodes = members
                .nodes()
                .map(|(node_id, node)| {
                    let node_id = *node_id;
                    let node = node.clone();
                    let state = if leader_id == Some(node_id) {
                        ServerState::Leader
                    } else if voters.contains(&node_id) {
                        ServerState::Follower
                    } else if learners.contains(&node_id) {
                        ServerState::Learner
                    } else {
                        ServerState::Unknown
                    };
                    PartialNodeStatus {
                        node_id,
                        node,
                        state,
                    }
                })
                .collect::<Vec<_>>();
            (this_node_state, committed, nodes)
        })
        .await
        .or_internal_error()?;
    let cluster_id = repl.state_machine.cluster_id().await;
    let cluster_name = cluster_id
        .is_some()
        .then(|| app_state.cfg.cluster.name.to_owned());
    let this_node_id = repl.node_id;

    let this_node_last_committed_timestamp = repl.time.now();
    let this_node_last_snapshot_id = repl.state_machine.last_snapshot_id().await;

    let nodes = futures_util::stream::iter(pnodes)
        .map(|peer| {
            let repl = repl.clone();
            async move {
                let last_log = if peer.node_id == this_node_id {
                    this_last_committed_log_index
                } else {
                    repl.get_peer_last_committed_log(peer.node_id, &peer.node)
                        .await
                        .ok()
                        .flatten()
                };
                NodeStatusOut {
                    node_id: peer.node_id,
                    address: peer.node.to_string(),
                    state: peer.state,
                    last_committed_log_index: last_log.map(|l| l.index),
                    last_committed_term: last_log.map(|l| l.leader_id.term),
                }
            }
        })
        .buffer_unordered(5)
        .collect()
        .await;

    Ok(MsgPackOrJson(ClusterStatusOut {
        cluster_id,
        cluster_name,
        this_node_id,
        this_node_state,
        this_node_last_committed_timestamp,
        this_node_last_snapshot_id,
        nodes,
    }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, JsonSchema)]
struct ClusterInitializeIn {}

admin_request_input!(ClusterInitializeIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct ClusterInitializeOut {
    cluster_id: ClusterId,
}

/// Initialize this node as the leader of a new cluster
///
/// This operation may only be performed against a node which has not been
/// initialized and is not currently a member of a cluster.
#[aide_annotate(op_id = "v1.admin.cluster.initialize")]
async fn cluster_initialize(
    State(app_state): State<AppState>,
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(_data): MsgPackOrJson<ClusterInitializeIn>,
) -> Result<MsgPackOrJson<ClusterInitializeOut>> {
    let node_id = repl.node_id;
    let addr = crate::core::cluster::network::detect_address(&app_state.cfg, node_id)
        .await
        .map_err(|err| {
            Error::internal(format!("Could not discover local node address: {err:?}"))
        })?;
    let my_node = Node::from(addr);
    let nodes = [(node_id, my_node)]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
    let cluster_id = crate::core::cluster::raft::initialize_cluster(&repl.raft, nodes)
        .await
        .or_internal_error()?;
    Ok(MsgPackOrJson(ClusterInitializeOut { cluster_id }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, JsonSchema)]
struct ClusterRemoveNodeIn {
    node_id: NodeId,
}

admin_request_input!(ClusterRemoveNodeIn);

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct ClusterRemoveNodeOut {
    node_id: NodeId,
}

/// Remove a node from the cluster.
///
/// This operation executes immediately and the node must be wiped and reset
/// before it can safely be added to the cluster.
#[aide_annotate(op_id = "v1.admin.cluster.remove-node")]
async fn cluster_remove_node(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(data): MsgPackOrJson<ClusterRemoveNodeIn>,
) -> Result<MsgPackOrJson<ClusterRemoveNodeOut>> {
    let node_id = data.node_id;

    repl.remove_node(node_id).await?;

    Ok(MsgPackOrJson(ClusterRemoveNodeOut { node_id }))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, JsonSchema)]
struct ClusterForceSnapshotIn {}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct ClusterForceSnapshotOut {
    snapshot_time: jiff::Timestamp,
    snapshot_log_index: u64,
}

/// Force the cluster to take a snapshot immediately
#[aide_annotate(op_id = "v1.admin.cluster.force-snapshot")]
async fn cluster_force_snapshot(
    Extension(repl): Extension<RaftState>,
    MsgPackOrJson(_data): MsgPackOrJson<ClusterForceSnapshotIn>,
) -> Result<MsgPackOrJson<ClusterForceSnapshotOut>> {
    let Some((snapshot_time, log_id)) = repl.trigger_snapshot().await.or_internal_error()? else {
        return Err(Error::bad_request(
            "snapshot_unavailable",
            "a snapshot cannot be taken at this time",
        ));
    };

    Ok(MsgPackOrJson(ClusterForceSnapshotOut {
        snapshot_time,
        snapshot_log_index: log_id.index,
    }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Admin");

    ApiRouter::new()
        .api_route_with(
            cluster_status_path,
            get_with(cluster_status, cluster_status_operation),
            &tag,
        )
        .api_route_with(
            cluster_initialize_path,
            post_with(cluster_initialize, cluster_initialize_operation),
            &tag,
        )
        .api_route_with(
            cluster_remove_node_path,
            post_with(cluster_remove_node, cluster_remove_node_operation),
            &tag,
        )
        .api_route_with(
            cluster_force_snapshot_path,
            post_with(cluster_force_snapshot, cluster_force_snapshot_operation),
            &tag,
        )
}
