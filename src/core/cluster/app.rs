#![expect(clippy::disallowed_types)] // we can't use MsgPackOrJson because these endpoints are not OpenAPI-based

use std::{collections::BTreeMap, sync::Arc};

use axum::{
    Extension, Json,
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::TypedHeader;
use coyote_proto::MsgPack;
use headers::{Authorization, authorization::Bearer};
use http::StatusCode;
use openraft::{
    ChangeMembers,
    raft::{AppendEntriesRequest, InstallSnapshotRequest, VoteRequest},
};
use serde::Serialize;
use tap::{Pipe, TapFallible};

use super::{Node, NodeId, handle::RaftState, network::detect_address, proto::*, raft::TypeConfig};
use crate::{AppState, Configuration};

#[derive(Clone)]
struct RaftSecret(Arc<Vec<u8>>);

impl RaftSecret {
    fn new(secret: &str) -> Self {
        Self(Arc::new(secret.as_bytes().to_owned()))
    }
}

async fn authenticate(
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    Extension(RaftSecret(secret)): Extension<RaftSecret>,
    request: Request,
    next: Next,
) -> Response {
    let Some(TypedHeader(Authorization(bearer))) = bearer else {
        tracing::error!("got request without bearer token; rejecting");
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !constant_time_eq::constant_time_eq(&secret, bearer.token().as_bytes()) {
        tracing::error!("got invalid bearer token for raft operation");
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

pub fn router(cfg: &Configuration) -> axum::Router<AppState> {
    let mut authenticated = axum::Router::new()
        .route("/repl/raft/append_entries", post(append_entries))
        .route("/repl/raft/vote", post(vote))
        .route("/repl/raft/stream-snapshot", post(stream_snapshot))
        .route(
            "/repl/raft/handle-forwarded-write",
            post(handle_forwarded_write),
        )
        .route("/repl/raft/last-id", get(last_id))
        .route("/repl/raft/admin/add-learner", post(add_learner))
        .route("/repl/raft/admin/upgrade-learner", post(upgrade_learner))
        .route(
            "/repl/raft/admin/change-membership",
            post(change_membership),
        )
        .route("/repl/raft/admin/initialize", post(initialize));

    if let Some(secret) = &cfg.cluster.secret {
        authenticated = authenticated
            .layer(axum::middleware::from_fn(authenticate))
            .layer(Extension(RaftSecret::new(secret)));
    }

    let unauthenticated = axum::Router::new()
        .route("/repl/raft/admin/metrics", get(metrics))
        .route("/repl/discover", get(discover))
        .route("/repl/raft/admin/force-snapshot", post(force_snapshot)) // TODO: should this be unauth?
        .route("/repl/health", get(health));

    authenticated.merge(unauthenticated)
}

// Helpers

#[derive(Debug, Serialize)]
struct BasicError {
    error_message: String,
}

impl<T: ToString> From<T> for BasicError {
    fn from(value: T) -> Self {
        Self {
            error_message: value.to_string(),
        }
    }
}

fn internal_error(s: impl ToString) -> Response {
    let body = BasicError::from(s);
    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
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
    Err: ToString,
{
    match result {
        Ok(ok) => (StatusCode::OK, Json(ok)).into_response(),
        Err(e) => {
            let error = BasicError::from(e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
        }
    }
}

// Standard functions

#[tracing::instrument(skip_all)]
async fn append_entries(
    Extension(state): Extension<RaftState>,
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
    Extension(state): Extension<RaftState>,
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
    Extension(state): Extension<RaftState>,
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

#[tracing::instrument(skip_all)]
async fn handle_forwarded_write(
    Extension(state): Extension<RaftState>,
    MsgPack(mut req): MsgPack<ForwardedWriteRequest>,
) -> Result<MsgPack<ForwardedWriteResponse>, crate::Error> {
    // reset the timestamp in case the forwarding node was out of sync
    req.request.timestamp = state.state_machine.now();

    // intentionally do not use state.client_write because we don't want an infinite recursion
    // of forwardings
    let response =
        state.raft.client_write(req.request).await.map_err(|e| {
            crate::Error::internal(format!("Unable to execute forwarded write: {e:?}"))
        })?;
    let response = ForwardedWriteResponse {
        log_id: response.log_id,
        response: response.data,
    };
    Ok(MsgPack(response))
}

// Administrative functions

async fn discover(
    State(app_state): State<AppState>,
    Extension(raft_state): Extension<RaftState>,
) -> MsgPack<DiscoverResponse> {
    let cluster_name = app_state.cfg.cluster.name.clone();
    let cluster_id = raft_state.state_machine.cluster_id().await;
    let cluster = raft_state
        .raft
        .with_raft_state(move |state| DiscoverClusterResponse {
            last_committed_log_id: state.committed,
            cluster_name,
            state: state.server_state,
            cluster_id,
            known_peers: state
                .membership_state
                .committed()
                .nodes()
                .filter_map(|(nid, n)| {
                    if let Ok(peer) = n.clone().try_into() {
                        Some((*nid, peer))
                    } else {
                        tracing::warn!(node_id = ?nid, node = ?n, "could not construct peer address for node");
                        None
                    }
                })
                .collect(),
        })
        .await
        .tap_err(|err| tracing::warn!(?err, "failed to find local cluster state"))
        .ok();
    let response = DiscoverResponse {
        node_id: raft_state.node_id,
        cluster,
    };
    MsgPack(response)
}

async fn metrics(Extension(state): Extension<RaftState>) -> impl IntoResponse {
    let metrics = state.raft.metrics().borrow().clone();

    Json(metrics)
}

async fn add_learner(
    Extension(state): Extension<RaftState>,
    MsgPack(request): MsgPack<AddLearnerRequest>,
) -> impl IntoResponse {
    tracing::debug!(address=?request.address, "adding a learner");
    let node = Node::from(request.address);
    rpc_response(state.raft.add_learner(request.node_id, node, true).await)
}

async fn upgrade_learner(
    Extension(raft_state): Extension<RaftState>,
    MsgPack(request): MsgPack<UpgradeLearnerRequest>,
) -> impl IntoResponse {
    tracing::debug!(node_id=?request.node_id, "upgrading learner to follower");
    let request = ChangeMembers::AddVoterIds([request.node_id].into_iter().collect());
    rpc_response(raft_state.raft.change_membership(request, true).await)
}

async fn change_membership(
    Extension(state): Extension<RaftState>,
    Json(request): Json<ChangeMembershipRequest>,
) -> impl IntoResponse {
    state
        .raft
        .change_membership(request.desired_node_ids.clone(), false)
        .await
        .pipe(admin_response)
}

async fn initialize(
    State(app_state): State<AppState>,
    Extension(state): Extension<RaftState>,
) -> impl IntoResponse {
    let addr = match detect_address(&app_state.cfg) {
        Ok(a) => a,
        Err(_e) => return internal_error("could not find any valid addresses"),
    };
    let my_node = Node::new(addr);
    let nodes = [(state.node_id, my_node)]
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    super::raft::initialize_cluster(&state.raft, nodes)
        .await
        .pipe(admin_response)
}

async fn force_snapshot(Extension(state): Extension<RaftState>) -> impl IntoResponse {
    state.trigger_snapshot().await.pipe(admin_response)
}

async fn last_id(Extension(state): Extension<RaftState>) -> impl IntoResponse {
    state
        .raft
        .with_raft_state(move |s| LastIdResponse {
            last_committed_log_id: s.committed,
        })
        .await
        .pipe(rpc_response)
}

async fn health(Extension(state): Extension<RaftState>) -> impl IntoResponse {
    let leader = state.raft.current_leader().await;
    let me = state.node_id;
    let cluster_id = state.state_machine.cluster_id().await;
    let wall_time = jiff::Timestamp::now();
    let monotonic_time = state.state_machine.time.last();
    state
        .raft
        .with_raft_state(move |s| {
            let response = HealthResponse {
                node_id: me,
                last_committed_log_index: s.committed.map(|l| l.index),
                server_state: s.server_state,
                cluster_id,
                leader,
                wall_time,
                monotonic_time,
            };
            let status = if s.server_state.is_leader()
                || s.server_state.is_follower()
                || s.server_state.is_candidate()
            {
                StatusCode::OK
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(response)).into_response()
        })
        .await
        .map_err(|e| internal_error(e).into_response())
}
