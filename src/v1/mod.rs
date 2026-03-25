// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::ApiRouter;

use crate::{
    AppState,
    core::{auth::authorization, otel_spans::request_metrics_middleware},
};

pub mod endpoints;
pub mod utils;

pub fn router(state: Option<AppState>) -> ApiRouter<AppState> {
    let mut ret: ApiRouter<AppState> = ApiRouter::new();

    if let Some(state) = &state {
        ret = ret.layer(axum::middleware::from_fn_with_state(
            state.clone(),
            request_metrics_middleware,
        ));
    }

    let unauthenticated_router: ApiRouter<AppState> =
        ApiRouter::new().merge(endpoints::health::router());

    let mut authenticated_router: ApiRouter<AppState> = ApiRouter::new()
        .merge(endpoints::admin::router())
        .merge(endpoints::auth_token::router())
        .merge(endpoints::cache::router())
        .merge(endpoints::kv::router())
        .merge(endpoints::rate_limit::router())
        .merge(endpoints::idempotency::router())
        .merge(endpoints::msgs::router());

    if let Some(state) = state {
        authenticated_router =
            authenticated_router.layer(axum::middleware::from_fn_with_state(state, authorization));
    }

    ret = ret
        .merge(authenticated_router)
        .merge(unauthenticated_router);

    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        let dev_router: ApiRouter<AppState> = development::router().into();
        return ret.merge(dev_router);
    }

    ret
}

#[cfg(debug_assertions)]
mod development {
    use axum::{Router, extract::FromRequestParts, routing::get};
    use coyote_proto::MsgPackOrJson;
    use http::request::Parts;
    use serde::Serialize;

    use crate::{
        AppState,
        error::{Error, Result},
    };

    struct EchoData {
        pub headers: String,
    }

    #[derive(Serialize)]
    struct EmptyResponse {}

    impl<S> FromRequestParts<S> for EchoData
    where
        S: Send + Sync,
    {
        type Rejection = Error;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
            let headers = format!("{:?}", parts.headers);
            Ok(EchoData { headers })
        }
    }

    async fn echo(data: EchoData, body: String) -> Result<MsgPackOrJson<EmptyResponse>> {
        tracing::info!(">>> Echo");
        tracing::info!("{}", data.headers);
        tracing::info!("{}", body);
        tracing::info!("<<< Echo");
        Ok(MsgPackOrJson(EmptyResponse {}))
    }

    pub(super) fn router() -> Router<AppState> {
        Router::new().route("/development/echo", get(echo).post(echo))
    }
}
