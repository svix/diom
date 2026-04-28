#![warn(clippy::str_to_string)]

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
use axum::response::{IntoResponse, Response};
use diom_proto::{ErrorBody, MsgPackOrJson};
use hyper::StatusCode;
use tokio::task::JoinError;

mod can_fail_ext;
mod option_ext;
mod result_ext;

pub use self::{can_fail_ext::CanFailExt, option_ext::OptionExt, result_ext::ResultExt};

/// A short-hand version of a [`std::result::Result`] that defaults to Diom'es [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned from the Diom API
#[derive(Debug)]
pub struct Error(Box<ErrorType>);

impl Error {
    pub fn new(error_type: ErrorType) -> Self {
        Self(Box::new(error_type))
    }

    fn operation_error(
        http_status: StatusCode,
        code: &'static str,
        detail: impl fmt::Display,
    ) -> Self {
        Self::new(ErrorType::OperationError {
            http_status,
            body: ErrorBody::operation_error(code, detail),
        })
    }

    fn server_error(
        http_status: StatusCode,
        code: &'static str,
        detail: impl fmt::Display,
    ) -> Self {
        Self::new(ErrorType::ServerError {
            http_status,
            body: ErrorBody::server_error(code, detail),
        })
    }

    #[track_caller]
    pub fn internal(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Internal {
            body: ErrorBody::server_error("internal_error", s),
            trace: vec![Location::caller()],
        })
    }

    /// Create an error value for semantically invalid user input.
    ///
    /// Deserialization should take care of most input parsing.
    /// This constructor should only be used for invariant violations that
    /// could theoretically be put in the `Deserialize` implementation
    /// of the input, but aren't for practical reasons.
    pub fn invalid_data(detail: impl fmt::Display, location: impl Into<Option<String>>) -> Self {
        Self::new(ErrorType::OperationError {
            http_status: StatusCode::UNPROCESSABLE_ENTITY,
            body: ErrorBody::invalid_input("invalid_data", detail).with_location(location),
        })
    }

    /// Create an `operation_error` with the default status code of HTTP 400.
    pub fn bad_request(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::operation_error(StatusCode::BAD_REQUEST, code, detail)
    }

    pub fn conflict(detail: impl fmt::Display) -> Self {
        Self::bad_request("conflict", detail)
    }

    pub fn entity_not_found(entity: &'static str) -> Self {
        Self::bad_request("not_found", format!("{entity} not found"))
    }

    pub fn authentication(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::operation_error(StatusCode::UNAUTHORIZED, code, detail)
    }

    pub fn authorization(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::operation_error(StatusCode::FORBIDDEN, code, detail)
    }

    /// Create an `Error` from a raft-serialized response failure.
    pub fn from_raft(
        http_status: StatusCode,
        type_: Option<diom_proto::ErrorType>,
        code: Option<String>,
        detail: Option<String>,
    ) -> Self {
        let type_ = type_.unwrap_or(diom_proto::ErrorType::ServerError);

        let code = match code {
            Some(c) => c.into(),
            None => "generic".into(),
        };

        let detail = detail.unwrap_or_else(|| {
            tracing::warn!("no error message in error response from raft");
            "unknown error".to_owned()
        });

        Self::new(ErrorType::Remote {
            http_status,
            body: ErrorBody::from_raft(type_, code, detail),
        })
    }

    pub fn not_ready(s: impl fmt::Display) -> Self {
        Self::server_error(StatusCode::SERVICE_UNAVAILABLE, "not_ready", s)
    }

    pub fn shutting_down() -> Self {
        Self::server_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "shutting_down",
            "server shutting down",
        )
    }

    /// Decompose into HTTP status, optional error code, and optional detail message.
    pub fn into_parts(self) -> (StatusCode, ErrorBody) {
        match *self.0 {
            ErrorType::InvalidInput { http_status, body } => {
                tracing::trace!(error = %body, "invalid input");
                (http_status, body)
            }
            ErrorType::OperationError { http_status, body } => {
                tracing::debug!(error = %body, "operation error");
                (http_status, body)
            }
            ErrorType::ServerError { http_status, body } => {
                tracing::debug!(error = %body, "server error");
                (http_status, body)
            }
            ErrorType::Internal { body, trace } => {
                tracing::error!(
                    location = ?trace.iter().map(ToString::to_string).collect::<Vec<_>>(),
                    message = body.detail,
                    "internal error",
                );
                (StatusCode::INTERNAL_SERVER_ERROR, body)
            }
            ErrorType::Remote { http_status, body } => (http_status, body),
        }
    }

    #[track_caller]
    pub fn trace(mut self) -> Self {
        if let ErrorType::Internal { trace, .. } = &mut *self.0 {
            trace.push(Location::caller());
        }
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (http_status, body) = self.into_parts();
        (http_status, MsgPackOrJson(body)).into_response()
    }
}

impl OperationOutput for Error {
    type Inner = Self;

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<aide::openapi::StatusCode>, aide::openapi::Response)> {
        use aide::openapi::StatusCode::Code;

        let standard_error_body_response =
            MsgPackOrJson::<ErrorBody>::operation_response(ctx, operation).unwrap();

        vec![
            (Some(Code(400)), standard_error_body_response.clone()),
            (Some(Code(401)), standard_error_body_response.clone()),
            (Some(Code(403)), standard_error_body_response.clone()),
            (Some(Code(422)), standard_error_body_response),
        ]
    }
}

pub trait Traceable<T> {
    /// Pushes the current [`Location`] onto the error's trace stack
    #[track_caller]
    fn trace(self) -> Result<T>;
}

impl<T> Traceable<T> for Result<T> {
    fn trace(self) -> Result<T> {
        // Using `map_err` would lose `#[track_caller]` information
        match self {
            Err(e) => Err(e.trace()),
            ok => ok,
        }
    }
}

#[derive(Debug)]
pub enum ErrorType {
    /// The request was invalid.
    ///
    /// This error type is to be used for 'stateless' errors that will fail no
    /// matter under which circumstances the same request is retried. Examples:
    ///
    /// - missing `content-type` header
    /// - msgpack decode error
    /// - value outside of supported range
    InvalidInput {
        http_status: StatusCode,
        body: ErrorBody,
    },

    /// The requested operation failed.
    ///
    /// This error type is to be used for 'stateful' errors. Examples:
    ///
    /// - invalid access token
    /// - namespace not found
    /// - any sort of conflict
    OperationError {
        http_status: StatusCode,
        body: ErrorBody,
    },

    /// An 'expected' server error.
    ServerError {
        http_status: StatusCode,
        body: ErrorBody,
    },

    /// An unexpected internal error.
    Internal {
        body: ErrorBody,
        trace: Vec<&'static Location<'static>>,
    },

    /// An error that was forwarded from another node.
    Remote {
        http_status: StatusCode,
        body: ErrorBody,
    },
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { http_status, body } => {
                write!(f, "invalid_input http_status={http_status:?} {body}")
            }
            Self::OperationError { http_status, body } => {
                write!(f, "operation_error http_status={http_status:?} {body}")
            }
            Self::ServerError { http_status, body } => {
                write!(f, "server_error http_status={http_status:?} {body}")
            }
            Self::Internal { body, .. } => write!(f, "internal {body}"),
            Self::Remote { http_status, body } => {
                write!(f, "remote http_status={http_status:?} {body:}")
            }
        }
    }
}

impl From<fjall::Error> for Error {
    #[track_caller]
    fn from(e: fjall::Error) -> Self {
        Self::internal(format!("{e:?}"))
    }
}

impl From<JoinError> for Error {
    #[track_caller]
    fn from(e: JoinError) -> Self {
        Self::internal(format!("{e:?}"))
    }
}
