#![warn(clippy::str_to_string)]

use std::{borrow::Cow, error, fmt, panic::Location};

use aide::OperationOutput;
use axum::response::{IntoResponse, Response};
use diom_proto::{MsgPackOrJson, StandardErrorBody};
use hyper::StatusCode;
use serde_json::json;
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
            body: StandardErrorBody::new(code, detail),
        })
    }

    #[track_caller]
    pub fn internal(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Internal {
            body: StandardErrorBody::new("internal_error", s),
            trace: vec![Location::caller()],
        })
    }

    pub fn conflict(detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::BadRequest(StandardErrorBody::new(
            "conflict",
            detail.to_string(),
        )))
    }

    pub fn entity_not_found(entity: &'static str) -> Self {
        Self::operation_error(
            StatusCode::BAD_REQUEST,
            "not_found",
            format!("{entity} not found"),
        )
    }

    pub fn bad_request(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::BadRequest(StandardErrorBody::new(code, detail)))
    }

    pub fn invalid_user_input(detail: impl fmt::Display) -> Self {
        // We'll probably change _how_ invalid user input is displayed later on,
        // but having a universal error function to capture user errors is ideal
        Self::bad_request("invalid_input", detail)
    }

    pub fn authentication(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::operation_error(StatusCode::UNAUTHORIZED, code, detail)
    }

    pub fn authorization(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::operation_error(StatusCode::FORBIDDEN, code, detail)
    }

    pub fn from_raft(
        http_status: StatusCode,
        code: Option<String>,
        detail: Option<String>,
    ) -> Self {
        Self::new(ErrorType::Operation {
            http_status,
            code: match code {
                Some(c) => c.into(),
                None => "generic".into(),
            },
            detail: detail.unwrap_or_else(|| {
                tracing::warn!("no error message in OperationError from raft");
                "unknown error".to_owned()
            }),
        })
    }

    pub fn not_ready(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::NotReady {
            message: s.to_string(),
        })
    }

    pub fn shutting_down() -> Self {
        Self::new(ErrorType::ShuttingDown)
    }

    /// Decompose into HTTP status, optional error code, and optional detail message.
    pub fn into_parts(self) -> (StatusCode, String, String) {
        match *self.0 {
            ErrorType::InvalidInput { http_status, body }
            | ErrorType::OperationError { http_status, body } => (
                http_status,
                body.code().to_owned(),
                body.detail().to_owned(),
            ),
            ErrorType::BadRequest(body) => (
                StatusCode::BAD_REQUEST,
                body.code().to_owned(),
                body.detail().to_owned(),
            ),
            ErrorType::Operation {
                http_status,
                code,
                detail,
            } => (http_status, code.into_owned(), detail),
            ErrorType::Internal { body, .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                body.code().to_owned(),
                body.detail().to_owned(),
            ),
            ErrorType::NotReady { message } => (
                StatusCode::SERVICE_UNAVAILABLE,
                "not_ready".to_owned(),
                message,
            ),
            ErrorType::ShuttingDown => (
                StatusCode::SERVICE_UNAVAILABLE,
                "shutting_down".to_owned(),
                "server shutting down".to_owned(),
            ),
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
        match *self.0 {
            ErrorType::InvalidInput { http_status, body } => {
                tracing::trace!(error = %body, "invalid input");
                (http_status, MsgPackOrJson(body)).into_response()
            }
            ErrorType::OperationError { http_status, body } => {
                tracing::debug!(error = %body, "operation error");
                (http_status, MsgPackOrJson(body)).into_response()
            }
            ErrorType::BadRequest(body) => {
                tracing::debug!(error = %body, "bad request");
                (StatusCode::BAD_REQUEST, MsgPackOrJson(body)).into_response()
            }
            ErrorType::Operation {
                http_status,
                code,
                detail,
            } => (
                http_status,
                MsgPackOrJson(json!({ "code": code, "detail": detail })),
            )
                .into_response(),
            ErrorType::Internal { trace, body } => {
                tracing::error!(
                    location = ?trace.into_iter().map(ToString::to_string).collect::<Vec<_>>(),
                    message = body.detail(),
                    "internal error",
                );
                (StatusCode::INTERNAL_SERVER_ERROR, MsgPackOrJson(body)).into_response()
            }
            ErrorType::NotReady { message } => (
                StatusCode::SERVICE_UNAVAILABLE,
                MsgPackOrJson(json!({"code": "not_ready", "detail": message})),
            )
                .into_response(),
            ErrorType::ShuttingDown => (
                StatusCode::SERVICE_UNAVAILABLE,
                MsgPackOrJson(json!({"code": "shutting_down", "detail": "server shutting down"})),
            )
                .into_response(),
        }
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
            MsgPackOrJson::<StandardErrorBody>::operation_response(ctx, operation).unwrap();

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
        body: StandardErrorBody,
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
        body: StandardErrorBody,
    },

    /// An unexpected internal error.
    Internal {
        body: StandardErrorBody,
        trace: Vec<&'static Location<'static>>,
    },

    /// Bad user input (to be further refined)
    BadRequest(StandardErrorBody),

    /// An error from an Operation application
    Operation {
        http_status: StatusCode,
        code: Cow<'static, str>,
        detail: String,
    },

    /// The operation cannot proceed because the server is not yet ready
    NotReady { message: String },

    /// The operation cannot proceed because the server is shutting down
    ShuttingDown,
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
            Self::Internal { body, .. } => write!(f, "internal {body}"),
            Self::NotReady { message } => write!(f, "not_ready {message}"),
            Self::BadRequest(s) => write!(f, "bad_request {s}"),
            Self::ShuttingDown => write!(f, "shutting_down"),
            Self::Operation {
                http_status,
                code,
                detail,
            } => {
                write!(
                    f,
                    "http_status={http_status:?} code={code:?} detail={detail:?}"
                )
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
