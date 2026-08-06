#[cfg(debug_assertions)]
use aide::axum::routing::post;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use axum::extract::Extension;
use diom_derive::aide_annotate;
use diom_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    core::cluster::RaftState,
    error::{Error, Result, ResultExt},
    v1::utils::openapi_tag,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PingOut {
    pub ok: bool,
}

/// Verify the server is up and running.
///
/// This endpoint only checks the server itself, not the cluster mechanism, and should not be used
/// as a readiness gate.
#[aide_annotate(op_id = "v1.health.ping")]
async fn ping() -> Result<MsgPackOrJson<PingOut>> {
    Ok(MsgPackOrJson(PingOut { ok: true }))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadyOut {
    pub ok: bool,
    pub message: String,
}

/// Verify that this server is ready to serve customer traffic.
#[aide_annotate(op_id = "v1.health.ready")]
async fn ready(Extension(repl): Extension<RaftState>) -> Result<MsgPackOrJson<ReadyOut>> {
    let state = repl.state().await.or_internal_error()?;
    if state.is_leader() || state.is_follower() || state.is_candidate() {
        Ok(MsgPackOrJson(ReadyOut {
            ok: true,
            message: format!("State: {state:?}"),
        }))
    } else {
        Err(Error::not_ready(format!(
            "Cluster is currently in state {state:?}"
        )))
    }
}

/// Intentionally return an error
#[aide_annotate(op_id = "v1.health.error")]
async fn error() -> Result<()> {
    Err(Error::internal("despite appearances, I am not an error"))
}

/// Intentionally panic a thread
#[aide_annotate(op_id = "v1.health.panic")]
#[cfg(debug_assertions)]
async fn panic() -> Result<()> {
    panic!("oh dear")
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Health");

    let router = ApiRouter::new()
        .api_route_with(ping_path, get_with(ping, ping_operation), &tag)
        .api_route_with(ready_path, get_with(ready, ready_operation), &tag)
        .api_route_with(error_path, post_with(error, error_operation), &tag);

    #[cfg(debug_assertions)]
    let router = router.route("/health/panic", post(panic));

    router
}
