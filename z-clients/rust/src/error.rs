use std::{fmt, sync::Arc};

use headers::ContentType;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the Diom API
#[derive(Clone, Debug)]
pub enum Error {
    /// Could not make the intended request and fully receive the response.
    Network(NetworkError),
    /// The server indicated that the request was invalid.
    Client(ClientError),
    /// Unexpected server-side error.
    Server(ServerError),
    /// The configured request timeout was hit.
    Timeout,
    /// Some other error that could not be classified.
    Other(GenericError),
}

impl Error {
    pub(crate) fn network(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Network(NetworkError(Arc::new(err) as _))
    }

    pub(crate) fn other(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Other(GenericError(Arc::new(err) as _))
    }

    #[must_use]
    pub fn is_network(&self) -> bool {
        matches!(self, Self::Network(_))
    }

    #[must_use]
    pub fn is_client(&self) -> bool {
        matches!(self, Self::Client(_))
    }

    #[must_use]
    pub fn is_server(&self) -> bool {
        matches!(self, Self::Server(_))
    }

    #[must_use]
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }

    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(_))
    }

    pub(crate) fn is_retryable(&self) -> bool {
        match self {
            Self::Network(_) | Self::Server(_) | Self::Timeout => true,
            Self::Client(_) | Self::Other(_) => false,
        }
    }

    pub(crate) async fn from_response(
        status_code: http::StatusCode,
        body: Incoming,
        content_type: ContentType,
    ) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                let mime: headers::Mime = content_type.into();
                if status_code == http::StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Client(ClientError::new(
                        status_code,
                        deserialize_body(status_code, &mime, &bytes)
                            .map(ClientErrorBody::Validation),
                    ))
                } else if status_code.is_client_error() {
                    Self::Client(ClientError::new(
                        status_code,
                        deserialize_body(status_code, &mime, &bytes).map(ClientErrorBody::Standard),
                    ))
                } else {
                    Self::Server(ServerError::new(
                        status_code,
                        deserialize_body(status_code, &mime, &bytes),
                    ))
                }
            }
            Err(e) => Self::network(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // don't print inner errors that are returned from Error::source
            Self::Network(_) => write!(f, "network error"),
            Self::Client(e) => write!(f, "client error: {e}"),
            Self::Server(e) => write!(f, "server error: {e}"),
            Self::Timeout => write!(f, "timeout"),
            Self::Other(_) => write!(f, "other"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(e) => Some(&*e.0),
            Self::Other(e) => Some(&*e.0),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkError(Arc<dyn std::error::Error + Send + Sync + 'static>);

#[derive(Clone, Debug)]
pub struct ClientError(Arc<ClientErrorInner>);

impl ClientError {
    fn new(http_status: http::StatusCode, body: Option<ClientErrorBody>) -> Self {
        Self(Arc::new(ClientErrorInner { http_status, body }))
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

#[derive(Clone, Debug)]
pub struct ServerError(Arc<ServerErrorInner>);

impl ServerError {
    fn new(http_status: http::StatusCode, body: Option<StandardHttpError>) -> Self {
        Self(Arc::new(ServerErrorInner { http_status, body }))
    }

    /// Get the HTTP status associated with this error.
    pub fn http_status(&self) -> http::StatusCode {
        self.0.http_status
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP {}", self.0.http_status)?;
        match &self.0.body {
            Some(e) => {
                write!(f, " (code='{}', detail='{}')", e.code, e.detail)
            }
            None => Ok(()),
        }
    }
}

#[derive(Debug)]
struct ServerErrorInner {
    http_status: http::StatusCode,
    body: Option<StandardHttpError>,
}

#[derive(Clone, Debug)]
pub struct GenericError(Arc<dyn std::error::Error + Send + Sync + 'static>);

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

#[derive(Clone, Debug, Deserialize)]
struct StandardHttpError {
    pub code: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize)]
struct HttpValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}
