// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::{
    routing::{get},
    ApiRouter,
};

use crate::{
    v1::utils::{openapi_tag, NoContent},
    AppState,
};

async fn ping() -> NoContent {
    NoContent
}

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Health");

    ApiRouter::new()
        .api_route("/health/ping", get(ping).head(ping))
}
