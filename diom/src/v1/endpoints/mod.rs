// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use aide::axum::ApiRouter;

use crate::AppState;

pub mod health;
pub mod kv;
pub mod cache;
pub mod rate_limiter;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(health::router())
        .merge(cache::router())
        .merge(kv::router())
        .merge(rate_limiter::router())
}
