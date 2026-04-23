#![warn(clippy::str_to_string)]

use std::{error, fmt, panic::Location};

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

    pub fn entity_not_found(detail: impl Into<Option<String>>) -> Self {
        Self::new(ErrorType::EntityNotFound(StandardErrorBody::new(
            "not_found",
            detail
                .into()
                .unwrap_or_else(|| "Entity not found".to_owned()),
        )))
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
        Self::new(ErrorType::Authentication(StandardErrorBody::new(
            code, detail,
        )))
    }

    pub fn authorization(code: &'static str, detail: impl fmt::Display) -> Self {
        Self::new(ErrorType::Authorization(StandardErrorBody::new(
            code, detail,
        )))
    }

    pub fn operation(code: StatusCode, detail: Option<String>) -> Self {
        Self::new(ErrorType::Operation {
            status: code,
            error_code: None,
            detail,
        })
    }

    pub fn operation_with_code(status: StatusCode, error_code: String, detail: String) -> Self {
        Self::new(ErrorType::Operation {
            status,
            error_code: Some(error_code),
            detail: Some(detail),
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
    pub fn into_parts(self) -> (StatusCode, Option<String>, Option<String>) {
        match *self.0 {
            ErrorType::BadRequest(body) | ErrorType::EntityNotFound(body) => (
                StatusCode::BAD_REQUEST,
                Some(body.code().to_owned()),
                Some(body.detail().to_owned()),
            ),
            ErrorType::Authentication(body) => (
                StatusCode::UNAUTHORIZED,
                Some(body.code().to_owned()),
                Some(body.detail().to_owned()),
            ),
            ErrorType::Authorization(body) => (
                StatusCode::FORBIDDEN,
                Some(body.code().to_owned()),
                Some(body.detail().to_owned()),
            ),
            ErrorType::Operation {
                status,
                error_code,
                detail,
            } => (status, error_code, detail),
            ErrorType::Internal { body, .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Some(body.code().to_owned()),
                Some(body.detail().to_owned()),
            ),
            ErrorType::NotReady { .. } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("NOT_READY".to_owned()),
                None,
            ),
            ErrorType::ShuttingDown => (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("SHUTTING_DOWN".to_owned()),
                None,
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
            ErrorType::BadRequest(body) => {
                tracing::debug!(error = %body, "bad request");
                (StatusCode::BAD_REQUEST, MsgPackOrJson(body)).into_response()
            }
            ErrorType::EntityNotFound(body) => {
                tracing::debug!(error = %body, "entity not found");
                (StatusCode::BAD_REQUEST, MsgPackOrJson(body)).into_response()
            }
            ErrorType::Authentication(body) => {
                tracing::debug!(error = %body, "authentication");
                (StatusCode::UNAUTHORIZED, MsgPackOrJson(body)).into_response()
            }
            ErrorType::Authorization(body) => {
                tracing::debug!(error = %body, "authorization");
                (StatusCode::FORBIDDEN, MsgPackOrJson(body)).into_response()
            }
            ErrorType::Operation {
                status,
                error_code: Some(error_code),
                detail: Some(detail),
            } => (
                status,
                MsgPackOrJson(json!({ "code": error_code, "detail": detail })),
            )
                .into_response(),
            ErrorType::Operation {
                status,
                detail: Some(detail),
                ..
            } => (status, detail).into_response(),
            ErrorType::Operation { status, .. } => status.into_response(),
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
                MsgPackOrJson(json!({"code": "NOT_READY", "detail": message})),
            )
                .into_response(),
            ErrorType::ShuttingDown => (
                StatusCode::SERVICE_UNAVAILABLE,
                MsgPackOrJson(json!({"code": "SHUTTING_DOWN", "detail": "server shutting down"})),
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
    /// An unexpected internal error
    Internal {
        body: StandardErrorBody,
        trace: Vec<&'static Location<'static>>,
    },

    /// Bad user input (to be further refined)
    BadRequest(StandardErrorBody),

    /// Entity not found
    EntityNotFound(StandardErrorBody),

    /// Authentication error
    Authentication(StandardErrorBody),

    /// Authorization error
    Authorization(StandardErrorBody),

    /// An error from an Operation application
    Operation {
        status: StatusCode,
        error_code: Option<String>,
        detail: Option<String>,
    },

    /// The operation cannot proceed because the server is not yet ready
    NotReady { message: String },

    /// The operation cannot proceed because the server is shutting down
    ShuttingDown,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { body, .. } => write!(f, "internal {body}"),
            Self::NotReady { message } => write!(f, "not_ready {message}"),
            Self::BadRequest(s) => write!(f, "bad_request {s}"),
            Self::EntityNotFound(s) => write!(f, "not_found {s}"),
            Self::Authentication(s) => write!(f, "authn {s}"),
            Self::Authorization(s) => write!(f, "authz {s}"),
            Self::ShuttingDown => write!(f, "shutting_down"),
            Self::Operation { detail, status, .. } => {
                if let Some(detail) = detail {
                    detail.fmt(f)
                } else {
                    write!(f, "code {status}")
                }
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
