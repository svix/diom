use super::msgpack_or_json::MsgPackOrJson;
use crate::ErrorBody;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use futures_util::stream::StreamExt;
use std::time::Duration;

const MAX_BODY_READ_TIME: Duration = Duration::from_secs(10);

async fn read_body_forever(body: axum::body::Body, hard_body_limit: usize) {
    let mut stream = body.into_data_stream();
    let mut remaining = hard_body_limit;
    while let Some(Ok(data)) = stream.next().await {
        remaining = remaining.saturating_sub(data.len());
        if remaining == 0 {
            break;
        }
    }
}

/// Handler to ensure that large requests (but not huge) get a graceful 413
///
/// The axum BodyLimit middleware (and tower-http's RequestBodyLimitLayer) work
/// by actually shutting down the read of the stream when the limit is hit; this causes
/// clients to get an EPIPE, and (depending on whether the end-of-stream packet
/// from the server is received before or after the last bytes are sent), can prevent
/// clients from actually seeing the 413.
///
/// This middleware *only* looks at Content-Length, and emits a 413 without doing anything to the
/// body. This can be tricked by HTTP/1.1 chunked-encoding or HTTP/1.0 connection-close or a whole
/// variety of HTTP/2 things, so it's important to also use one of those other middlewares,
/// configured for a higher value.
pub async fn limit_requests_body_gracefully(
    State((body_limit, hard_body_limit)): State<(usize, usize)>,
    request: Request,
    next: Next,
) -> Response {
    if let Some(content_length) = request
        .headers()
        .get(http::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        && content_length > body_limit
    {
        tokio::task::spawn(async move {
            // consume up to hard_body_limit bytes and discard them; do this in
            // a background task so that we can send the response right away
            let body = request.into_body();
            tokio::time::timeout(MAX_BODY_READ_TIME, read_body_forever(body, hard_body_limit)).await
        });
        return (
            http::StatusCode::PAYLOAD_TOO_LARGE,
            MsgPackOrJson(ErrorBody::invalid_input(
                "payload-too-large",
                "Request payload is too large.",
            )),
        )
            .into_response();
    }

    next.run(request).await
}
