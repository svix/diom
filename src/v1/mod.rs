// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::ApiRouter;
use tower_http::trace::TraceLayer;

use crate::{
    AppState,
    core::otel_spans::{
        AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator, request_metrics_middleware,
    },
};

pub mod endpoints;
pub mod utils;

pub fn router(state: Option<AppState>) -> ApiRouter<AppState> {
    let mut ret: ApiRouter<AppState> = ApiRouter::new().merge(endpoints::router());

    if let Some(state) = state {
        ret = ret.layer(axum::middleware::from_fn_with_state(
            state,
            request_metrics_middleware,
        ));
    }

    let ret = ret.layer(
        TraceLayer::new_for_http()
            .make_span_with(AxumOtelSpanCreator)
            .on_response(AxumOtelOnResponse)
            .on_failure(AxumOtelOnFailure),
    );

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
