use axum::{body::Body, extract::Request as HttpRequest, response::Response as HttpResponse};
use bytes::Bytes;
use http::{HeaderValue, StatusCode, header};
use http_body_util::BodyExt;
use serde::{Serialize, de::DeserializeOwned};
use tap::Tap;
use tokio::sync::{mpsc, oneshot};

use super::msgpack_client::APPLICATION_MSGPACK;

/// API client for interacting with the internal "loopback" API server.
#[derive(Clone)]
pub struct InternalClient {
    sender: mpsc::Sender<InternalRequest>,
}

impl InternalClient {
    pub fn new(sender: mpsc::Sender<InternalRequest>) -> Self {
        Self { sender }
    }

    pub fn useless_instance_for_tests() -> Self {
        let (internal_req_tx, _internal_req_rx) = mpsc::channel(1);
        Self {
            sender: internal_req_tx,
        }
    }

    pub async fn post<T: Serialize, U: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U, InternalRequestError> {
        let body = rmp_serde::to_vec_named(body)?;
        let resp_body = self.post_impl(path, body).await?;
        Ok(rmp_serde::decode::from_slice(&resp_body)?)
    }

    async fn post_impl(&self, path: &str, body: Vec<u8>) -> Result<Bytes, InternalRequestError> {
        let uri = path.parse::<http::Uri>()?;
        let http_req = http::Request::new(Body::from(body)).tap_mut(|req| {
            *req.method_mut() = http::Method::POST;
            *req.uri_mut() = uri;
            req.headers_mut()
                .insert(header::CONTENT_TYPE, APPLICATION_MSGPACK);
        });

        let (response_tx, response_rx) = oneshot::channel();
        let internal_req = InternalRequest {
            inner: http_req,
            response_tx,
        };

        self.sender
            .send(internal_req)
            .await
            .map_err(|_| InternalRequestError::ShuttingDown)?;

        let mut resp = response_rx
            .await
            .map_err(|_| InternalRequestError::ShuttingDown)?;

        let status = resp.status();
        if !status.is_success() {
            return Err(InternalRequestError::ErrorResponse { status });
        }

        let content_type = resp.headers_mut().remove(header::CONTENT_TYPE);
        if content_type != Some(APPLICATION_MSGPACK) {
            return Err(InternalRequestError::ContentType(content_type));
        }

        let resp_body = resp
            .into_body()
            .collect()
            .await
            .map_err(InternalRequestError::BodyRead)?
            .to_bytes();
        Ok(resp_body)
    }
}

pub struct InternalRequest {
    pub inner: HttpRequest,
    pub response_tx: oneshot::Sender<HttpResponse>,
}

#[derive(Debug, thiserror::Error)]
pub enum InternalRequestError {
    #[error("invalid request path: {0}")]
    InvalidPath(#[from] http::uri::InvalidUri),
    #[error("failed to serialize request body: {0}")]
    RequestSerialization(#[from] rmp_serde::encode::Error),
    #[error("received error response with status code {status}")]
    ErrorResponse {
        status: StatusCode,
        // FIXME: Attach the error code / message from body
    },
    #[error("unexpected response content-type `{0:?}`")]
    ContentType(Option<HeaderValue>),
    #[error("failed to read response body: {0}")]
    BodyRead(axum::Error),
    #[error("failed to deserialize response body: {0}")]
    ResponseDeserialization(#[from] rmp_serde::decode::Error),
    #[error("server is shutting down")]
    ShuttingDown,
}
