// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use aide::axum::ApiRouter;

use crate::AppState;

pub mod health;
pub mod kv;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(health::router())
        .merge(kv::router())
}
