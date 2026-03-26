// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

pub mod policy;
pub mod role;
pub mod token;

use aide::axum::ApiRouter;

use crate::AppState;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .merge(token::router())
        .merge(role::router())
        .merge(policy::router())
}
