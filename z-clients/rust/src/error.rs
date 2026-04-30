use std::{fmt, sync::Arc};

use headers::ContentType;
use http_body_util::BodyExt;
use hyper::body::Incoming;
use serde::Deserialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct Error(Arc<ErrorImpl>);

impl Error {
    fn new(op_id: &'static str, kind: ErrorKind) -> Self {
        Self(Arc::new(ErrorImpl { op_id, kind }))
    }

    pub(crate) fn connection(
        op_id: &'static str,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::new(
            op_id,
            ErrorKind::Connection(ConnectionError(Box::new(err) as _)),
        )
    }

    pub(crate) fn timeout(op_id: &'static str) -> Self {
        Self::connection(op_id, TimeoutElapsed)
    }

    pub(crate) fn other(
        op_id: &'static str,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::new(
            op_id,
            ErrorKind::Other(OtherError {
                http_status: None,
                inner: Box::new(err) as _,
            }),
        )
    }

    /// Returns the ID of the operation that was attempted when this error occurred.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        self.0.op_id
    }

    #[must_use]
    pub fn kind(&self) -> &ErrorKind {
        &self.0.kind
    }

    #[must_use]
    pub fn is_connection(&self) -> bool {
        matches!(self.kind(), ErrorKind::Connection(_))
    }

    #[must_use]
    pub fn is_invalid_input(&self) -> bool {
        matches!(self.kind(), ErrorKind::InvalidInput(_))
    }

    #[must_use]
    pub fn is_operation_error(&self) -> bool {
        matches!(self.kind(), ErrorKind::OperationError(_))
    }

    #[must_use]
    pub fn is_server_error(&self) -> bool {
        matches!(self.kind(), ErrorKind::ServerError(_))
    }

    #[must_use]
    pub fn is_timeout(&self) -> bool {
        matches!(self.kind(), ErrorKind::Connection(c) if c.is_timeout())
    }

    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self.kind(), ErrorKind::Other(_))
    }

    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self.kind() {
            ErrorKind::Connection(_) | ErrorKind::ServerError(_) => true,
            ErrorKind::InvalidInput(_) | ErrorKind::OperationError(_) | ErrorKind::Other(_) => {
                false
            }
        }
    }

    pub(crate) async fn from_response(
        op_id: &'static str,
        http_status: http::StatusCode,
        body: Incoming,
        content_type: ContentType,
    ) -> Self {
        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                let mime: headers::Mime = content_type.into();
                let body = match deserialize_body(http_status, mime, &bytes) {
                    Ok(b) => b,
                    Err(e) => return Self::new(op_id, ErrorKind::Other(e)),
                };

                let res = ErrorResponse {
                    http_status,
                    code: body.code,
                    detail: body.detail,
                    location: body.location,
                };
                let kind = match body.type_ {
                    ErrorType::InvalidInput => ErrorKind::InvalidInput(InvalidInput(res)),
                    ErrorType::OperationError => ErrorKind::OperationError(OperationError(res)),
                    ErrorType::ServerError => ErrorKind::ServerError(ServerError(res)),
                };

                Self::new(op_id, kind)
            }
            Err(e) => Self::connection(op_id, e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0.kind {
            ErrorKind::InvalidInput(e) => write!(f, "invalid input {e}"),
            ErrorKind::OperationError(e) => write!(f, "operation error {e}"),
            ErrorKind::ServerError(e) => write!(f, "server error {e}"),
            // don't print inner errors that are returned from Error::source
            ErrorKind::Connection(_) => write!(f, "connection error"),
            ErrorKind::Other(_) => write!(f, "internal error"),
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
            ErrorKind::Connection(e) => Some(&*e.0),
            ErrorKind::Other(e) => Some(&*e.inner),
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
    Connection(ConnectionError),

    /// The server indicated that the request was invalid.
    InvalidInput(InvalidInput),

    /// The server indicated that the request failed.
    OperationError(OperationError),

    /// Unexpected server-side error.
    ServerError(ServerError),

    /// Some other error that could not be classified.
    Other(OtherError),
}

#[derive(Debug)]
pub struct ConnectionError(Box<dyn std::error::Error + Send + Sync + 'static>);

impl ConnectionError {
    fn is_timeout(&self) -> bool {
        self.0.downcast_ref::<TimeoutElapsed>().is_some()
    }
}

#[derive(Debug)]
#[non_exhaustive]
struct TimeoutElapsed;

impl fmt::Display for TimeoutElapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("timeout elapsed")
    }
}

impl std::error::Error for TimeoutElapsed {}

#[derive(Debug)]
pub struct InvalidInput(ErrorResponse);

impl InvalidInput {
    /// Stable identifier for the specific error condition that was triggered.
    pub fn code(&self) -> &str {
        &self.0.code
    }

    /// Get a human-readable error message, if any.
    ///
    /// This corresponds to the `detail` field of the error, if it's a string.
    pub fn message(&self) -> &str {
        &self.0.detail
    }
}

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct OperationError(ErrorResponse);

impl OperationError {
    /// Stable identifier for the specific error condition that was triggered.
    pub fn code(&self) -> &str {
        &self.0.code
    }

    /// Get a human-readable error message, if any.
    ///
    /// This corresponds to the `detail` field of the error, if it's a string.
    pub fn message(&self) -> &str {
        &self.0.detail
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct ServerError(ErrorResponse);

impl ServerError {
    /// Stable identifier for the specific error condition that was triggered.
    pub fn code(&self) -> &str {
        &self.0.code
    }

    /// Get a human-readable error message, if any.
    ///
    /// This corresponds to the `detail` field of the error, if it's a string.
    pub fn message(&self) -> &str {
        &self.0.detail
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct OtherError {
    #[allow(dead_code)] // good to have for Debug, doesn't need to be used elsewhere
    http_status: Option<http::StatusCode>,
    inner: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl OtherError {
    fn new(
        http_status: http::StatusCode,
        error: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            http_status: Some(http_status),
            inner: error.into(),
        }
    }
}

#[derive(Debug)]
struct UnexpectedMimeType(headers::Mime);

impl fmt::Display for UnexpectedMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unexpected mime type `{}`", self.0)
    }
}

impl std::error::Error for UnexpectedMimeType {}

fn deserialize_body<'b, 'a>(
    status_code: http::StatusCode,
    mime: headers::Mime,
    bytes: &'b [u8],
) -> Result<ErrorBody, OtherError>
where
    'b: 'a,
{
    if mime.subtype() == "json" {
        serde_json::from_slice(bytes).map_err(|e| OtherError::new(status_code, e))
    } else if mime.essence_str() == "application/msgpack" {
        rmp_serde::from_slice(bytes).map_err(|e| OtherError::new(status_code, e))
    } else {
        Err(OtherError::new(status_code, UnexpectedMimeType(mime)))
    }
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    #[serde(rename = "type")]
    type_: ErrorType,
    code: String,
    detail: String,
    location: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ErrorType {
    InvalidInput,
    OperationError,
    ServerError,
}

#[derive(Debug)]
struct ErrorResponse {
    #[allow(dead_code)] // good to have for Debug, doesn't need to be used elsewhere
    http_status: http::StatusCode,
    code: String,
    detail: String,
    location: Option<String>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            http_status: _,
            code,
            detail,
            location,
        } = self;
        write!(f, "code={code:?}")?;
        if let Some(location) = location {
            write!(f, " location={location:?}")?;
        }
        write!(f, " detail={detail:?}")
    }
}
