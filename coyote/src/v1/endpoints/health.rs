// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::error::Result;
use aide::axum::{routing::get, ApiRouter};
use axum::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{v1::utils::openapi_tag, AppState};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PingOut {
    pub ok: bool,
}

async fn ping() -> Result<Json<PingOut>> {
    Ok(Json(PingOut { ok: true }))
}

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Health");

    ApiRouter::new().api_route("/health/ping", get(ping).head(ping))
}
