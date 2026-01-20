// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::ApiRouter;

use crate::{AppState, v1::utils::openapi_tag};

pub fn router() -> ApiRouter<AppState> {
    let _tag = openapi_tag("Stream");

    ApiRouter::new()
}
