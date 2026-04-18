use std::{fmt, sync::Arc};

use headers::ContentType;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct Error(Arc<ErrorImpl>);

impl Error {
    fn new(op_id: &'static str, kind: ErrorKind) -> Self {
        Self(Arc::new(ErrorImpl { op_id, kind }))
    }

    pub(crate) fn network(
        op_id: &'static str,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::new(op_id, ErrorKind::Network(NetworkError(Box::new(err) as _)))
    }

    fn client(
        op_id: &'static str,
        http_status: http::StatusCode,
        body: Option<ClientErrorBody>,
    ) -> Self {
        let kind = ErrorKind::Client(ClientError::new(http_status, body));
        Self::new(op_id, kind)
    }

    fn server(
        op_id: &'static str,
        http_status: http::StatusCode,
        body: Option<StandardHttpError>,
    ) -> Self {
        let kind = ErrorKind::Server(ServerError::new(http_status, body));
        Self::new(op_id, kind)
    }

    pub(crate) fn timeout(op_id: &'static str) -> Error {
        Self::new(op_id, ErrorKind::Timeout(TimeoutError))
    }

    pub(crate) fn other(
        op_id: &'static str,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::new(op_id, ErrorKind::Other(GenericError(Box::new(err) as _)))
    }

    /// Returns the ID of the operation that was attempted when this error occurred.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        self.0.op_id
    }

    #[must_use]
    pub fn is_network(&self) -> bool {
        matches!(self.0.kind, ErrorKind::Network(_))
    }

    #[must_use]
    pub fn is_client(&self) -> bool {
        matches!(self.0.kind, ErrorKind::Client(_))
    }

    #[must_use]
    pub fn is_server(&self) -> bool {
        matches!(self.0.kind, ErrorKind::Server(_))
    }

    #[must_use]
    pub fn is_timeout(&self) -> bool {
        matches!(self.0.kind, ErrorKind::Timeout(_))
    }

    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self.0.kind, ErrorKind::Other(_))
    }

    pub fn is_retryable(&self) -> bool {
        match self.0.kind {
            ErrorKind::Network(_) | ErrorKind::Server(_) | ErrorKind::Timeout(_) => true,
            ErrorKind::Client(_) | ErrorKind::Other(_) => false,
        }
    }

    pub(crate) async fn from_response(
        op_id: &'static str,
        status_code: http::StatusCode,
        body: Incoming,
        content_type: ContentType,
    ) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                let mime: headers::Mime = content_type.into();
                if status_code == http::StatusCode::UNPROCESSABLE_ENTITY {
                    let body = deserialize_body(status_code, &mime, &bytes)
                        .map(ClientErrorBody::Validation);
                    Self::client(op_id, status_code, body)
                } else if status_code.is_client_error() {
                    let body =
                        deserialize_body(status_code, &mime, &bytes).map(ClientErrorBody::Standard);
                    Self::client(op_id, status_code, body)
                } else {
                    let body = deserialize_body(status_code, &mime, &bytes);
                    Self::server(op_id, status_code, body)
                }
            }
            Err(e) => Self::network(op_id, e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0.kind {
            // don't print inner errors that are returned from Error::source
            ErrorKind::Network(_) => write!(f, "network error"),
            ErrorKind::Client(e) => write!(f, "client error: {e}"),
            ErrorKind::Server(e) => write!(f, "server error: {e}"),
            ErrorKind::Timeout(_) => write!(f, "timeout"),
            ErrorKind::Other(_) => write!(f, "other"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ErrorImpl { op_id, kind } = &*self.0;
        f.debug_struct("Error")
            .field("operation_id", op_id)
            .field("kind", kind)
            .finish()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0.kind {
            ErrorKind::Network(e) => Some(&*e.0),
            ErrorKind::Other(e) => Some(&*e.0),
            _ => None,
        }
    }
}

struct ErrorImpl {
    op_id: &'static str,
    kind: ErrorKind,
}

/// The error type returned from the Diom API
#[derive(Debug)]
pub enum ErrorKind {
    /// Could not make the intended request and fully receive the response.
    Network(NetworkError),
    /// The server indicated that the request was invalid.
    Client(ClientError),
    /// Unexpected server-side error.
    Server(ServerError),
    /// The configured request timeout was hit.
    Timeout(TimeoutError),
    /// Some other error that could not be classified.
    Other(GenericError),
}

#[derive(Debug)]
pub struct NetworkError(Box<dyn std::error::Error + Send + Sync + 'static>);

#[derive(Debug)]
pub struct ClientError(Box<ClientErrorInner>);

impl ClientError {
    fn new(http_status: http::StatusCode, body: Option<ClientErrorBody>) -> Self {
        Self(Box::new(ClientErrorInner { http_status, body }))
    }

    /// Stable identifier for the specific error condition that was triggered.
    pub fn code(&self) -> Option<&str> {
        self.0.body.as_ref().and_then(|e| match e {
            ClientErrorBody::Standard(e) => Some(e.code.as_str()),
            ClientErrorBody::Validation(_) => None,
        })
    }

    /// Get a human-readable error message, if any.
    ///
    /// This corresponds to the `detail` field of the error, if it's a string.
    pub fn message(&self) -> Option<&str> {
        self.0.body.as_ref().and_then(|e| match e {
            ClientErrorBody::Standard(e) => Some(e.detail.as_str()),
            ClientErrorBody::Validation(_) => None,
        })
    }

    /// If `self` is a validation error, return the error details.
    pub fn as_validation(&self) -> Option<&[ValidationError]> {
        self.0.body.as_ref().and_then(|e| match e {
            ClientErrorBody::Validation(e) => Some(e.detail.as_slice()),
            ClientErrorBody::Standard(_) => None,
        })
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.0.http_status)?;
        match &self.0.body {
            Some(ClientErrorBody::Standard(e)) => {
                write!(f, " (code='{}', detail='{}')", e.code, e.detail)
            }
            Some(ClientErrorBody::Validation(e)) => {
                write!(f, " (detail={:?})", e.detail)
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug)]
struct ClientErrorInner {
    http_status: http::StatusCode,
    body: Option<ClientErrorBody>,
}

#[derive(Debug)]
enum ClientErrorBody {
    Standard(StandardHttpError),
    Validation(HttpValidationError),
}

#[derive(Debug)]
pub struct ServerError {
    http_status: http::StatusCode,
    body: Option<StandardHttpError>,
}

impl ServerError {
    fn new(http_status: http::StatusCode, body: Option<StandardHttpError>) -> Self {
        Self { http_status, body }
    }

    /// Get the HTTP status associated with this error.
    pub fn http_status(&self) -> http::StatusCode {
        self.http_status
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.http_status)?;
        match &self.body {
            Some(e) => {
                write!(f, " (code='{}', detail='{}')", e.code, e.detail)
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct TimeoutError;

#[derive(Debug)]
pub struct GenericError(Box<dyn std::error::Error + Send + Sync + 'static>);

fn deserialize_body<'b, 'a, T>(
    status_code: http::StatusCode,
    mime: &headers::Mime,
    bytes: &'b [u8],
) -> Option<T>
where
    T: Deserialize<'a>,
    'b: 'a,
{
    let payload = if mime.subtype() == "json" {
        serde_json::from_slice(bytes).ok()
    } else if mime.essence_str() == "application/msgpack" {
        rmp_serde::from_slice(bytes).ok()
    } else {
        None
    };
    if payload.is_none() {
        let as_str = String::from_utf8_lossy(bytes);
        tracing::warn!(?status_code, mime_type = ?mime, response = %as_str, "unparsable error");
    }
    payload
}

#[derive(Debug, Deserialize)]
struct StandardHttpError {
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
struct HttpValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}
