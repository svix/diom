use std::collections::BTreeMap;

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use coyote_proto::MsgPack;
use http::{StatusCode, Uri};
use openraft::{
    ChangeMembers,
    raft::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest},
};
use serde::Serialize;
use tap::{Pipe, TapFallible};

use super::Node;
use super::NodeId;
use super::network::detect_address;
use super::proto::*;
use super::raft::TypeConfig;
use crate::AppState;

pub fn router() -> axum::Router<AppState> {
    // TODO: implement snapshot methods
    axum::Router::new()
        .route("/repl/discover", get(discover))
        .route("/repl/raft/append_entries", post(append_entries))
        .route("/repl/raft/vote", post(vote))
        .route("/repl/raft/stream-snapshot", post(stream_snapshot))
        .route("/repl/raft/admin/metrics", get(metrics))
        .route("/repl/raft/admin/add-learner", post(add_learner))
        .route("/repl/raft/admin/upgrade-learner", post(upgrade_learner))
        .route(
            "/repl/raft/admin/change-membership",
            post(change_membership),
        )
        .route("/repl/raft/admin/initialize", post(initialize))
        .route("/repl/health", get(health))
}

// Helpers

fn internal_error(s: impl ToString) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, s.to_string()).into_response()
}

fn rpc_response<Ok, Err>(result: Result<Ok, Err>) -> Response
where
    Ok: Serialize,
    Err: Serialize,
{
    match result {
        Ok(ok) => (StatusCode::OK, MsgPack(ok)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, MsgPack(e)).into_response(),
    }
}

fn admin_response<Ok, Err>(result: Result<Ok, Err>) -> Response
where
    Ok: Serialize,
    Err: Serialize,
{
    match result {
        Ok(ok) => (StatusCode::OK, Json(ok)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e)).into_response(),
    }
}

// Standard functions

#[tracing::instrument(skip_all)]
#[axum::debug_handler]
async fn append_entries(
    State(state): State<AppState>,
    MsgPack(body): MsgPack<AppendEntriesRequest<TypeConfig>>,
) -> impl IntoResponse {
    tracing::debug!(
        num_entries=body.entries.len(),
        leader_commit=?body.leader_commit,
        "appending some entries to the log");
    state.raft.append_entries(body).await.pipe(rpc_response)
}

#[tracing::instrument(skip_all)]
async fn vote(
    State(state): State<AppState>,
    MsgPack(body): MsgPack<VoteRequest<NodeId>>,
) -> impl IntoResponse {
    tracing::debug!(
        vote=?body.vote,
        "recording a vote",
    );
    state.raft.vote(body).await.pipe(rpc_response)
}

#[tracing::instrument(skip_all)]
async fn stream_snapshot(
    State(state): State<AppState>,
    MsgPack(req): MsgPack<InstallSnapshotRequest<TypeConfig>>,
) -> impl IntoResponse {
    let _num_bytes = req.data.len();
    tracing::debug!(
        num_bytes = req.data.len(),
        vote = ?req.vote,
        done = req.done,
        "streaming part of a snapshot"
    );
    state.raft.install_snapshot(req).await.pipe(rpc_response)
}

// Administrative functions

async fn discover(State(app_state): State<AppState>) -> Json<DiscoverResponse> {
    let cluster_name = app_state.cfg.cluster.name.clone();
    let cluster = app_state
        .raft
        .with_raft_state(move |state| DiscoverClusterResponse {
            last_committed_log_id: state.committed,
            cluster_name,
            state: state.server_state,
            known_peers: state
                .membership_state
                .committed()
                .nodes()
                .map(|(nid, n)| (*nid, n.addr.parse().unwrap()))
                .collect(),
        })
        .await
        .tap_err(|err| tracing::warn!(?err, "failed to find local cluster state"))
        .ok();
    let response = DiscoverResponse {
        node_id: app_state.node_id,
        cluster,
    };
    Json(response)
}

async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.raft.metrics().borrow().clone();

    Json(metrics)
}

async fn add_learner(
    State(state): State<AppState>,
    Json(request): Json<AddLearnerRequest>,
) -> impl IntoResponse {
    let url = format!("http://{}/repl/raft/vote", request.address);
    let Ok(Some(addr)) = Uri::try_from(url).map(|v| v.authority().map(|a| a.to_string())) else {
        return internal_error("invalid address");
    };
    let node = Node { addr };
    admin_response(state.raft.add_learner(request.node_id, node, true).await)
}

async fn upgrade_learner(
    State(state): State<AppState>,
    Json(request): Json<UpgradeLearnerRequest>,
) -> impl IntoResponse {
    let request = ChangeMembers::AddVoterIds([request.node_id].into_iter().collect());
    admin_response(state.raft.change_membership(request, true).await)
}

async fn change_membership(
    State(state): State<AppState>,
    Json(request): Json<ChangeMembershipRequest>,
) -> impl IntoResponse {
    state
        .raft
        .change_membership(request.desired_node_ids.clone(), false)
        .await
        .pipe(admin_response)
}

async fn initialize(State(state): State<AppState>) -> impl IntoResponse {
    let addr = match detect_address(&state.cfg) {
        Ok(a) => a,
        Err(_e) => return internal_error("could not find any valid addresses"),
    }
    .to_string();
    let my_node = Node { addr };
    let nodes = [(state.node_id, my_node)]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    state.raft.initialize(nodes).await.pipe(admin_response)
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
