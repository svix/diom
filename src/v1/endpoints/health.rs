// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::{ApiRouter, routing::get};
use coyote_proto::MsgPackOrJson;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AppState, error::Result, v1::utils::openapi_tag};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PingOut {
    pub ok: bool,
}

async fn ping() -> Result<MsgPackOrJson<PingOut>> {
    Ok(MsgPackOrJson(PingOut { ok: true }))
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Health");

    ApiRouter::new().api_route_with("/health/ping", get(ping).head(ping), &tag)
}
