// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use aide::axum::ApiRouter;

use crate::AppState;

pub mod admin;
pub mod cache;
pub mod health;
pub mod idempotency;
pub mod kv;
pub mod msgs;
pub mod queue;
pub mod rate_limit;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(admin::router())
        .merge(health::router())
        .merge(cache::router())
        .merge(kv::router())
        .merge(rate_limit::router())
        .merge(idempotency::router())
        .merge(queue::router())
        .merge(msgs::router())
}
