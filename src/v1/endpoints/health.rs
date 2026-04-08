// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#[cfg(debug_assertions)]
use aide::axum::routing::post;
use aide::axum::{
    ApiRouter,
    routing::{get_with, post_with},
};
use coyote_derive::aide_annotate;
use diom_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AppState, error::Result, v1::utils::openapi_tag};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PingOut {
    pub ok: bool,
}

/// Verify the server is up and running.
#[aide_annotate(op_id = "v1.health.ping")]
async fn ping() -> Result<MsgPackOrJson<PingOut>> {
    Ok(MsgPackOrJson(PingOut { ok: true }))
}

/// Intentionally return an error
#[aide_annotate(op_id = "v1.health.error")]
async fn error() -> Result<()> {
    Err(coyote_error::Error::internal(
        "despite appearances, I am not an error",
    ))
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
        .api_route_with(error_path, post_with(error, error_operation), &tag);

    #[cfg(debug_assertions)]
    let router = router.route("/health/panic", post(panic));

    router
}
