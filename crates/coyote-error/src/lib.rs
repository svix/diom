// SPDX-FileCopyrightText: © 2022 Coyote Authors
// SPDX-License-Identifier: MIT

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
// FIXME: Change to MsgPackOrJson
#[expect(clippy::disallowed_types)]
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use coyote_proto::{MsgPackOrJson, StandardErrorBody, ValidationErrorBody, ValidationErrorItem};
use hyper::StatusCode;
use serde_json::json;
use tokio::task::JoinError;

mod can_fail_ext;
mod option_ext;
mod result_ext;

pub use self::{can_fail_ext::CanFailExt, option_ext::OptionExt, result_ext::ResultExt};

/// A short-hand version of a [`std::result::Result`] that defaults to Coyote'es [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned from the Coyote API
#[derive(Debug)]
pub struct Error(Box<ErrorType>);

impl Error {
    pub fn new(error_type: ErrorType) -> Self {
        Self(Box::new(error_type))
    }

    #[track_caller]
    pub fn internal(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Internal {
            message: s.to_string(),
            trace: vec![Location::caller()],
        })
    }

    pub fn not_found(detail: impl Into<Option<String>>) -> Self {
        Self::new(ErrorType::NotFound(StandardErrorBody::new(
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

    pub fn validation(detail: Vec<ValidationErrorItem>) -> Self {
        Self::new(ErrorType::Validation(ValidationErrorBody::new(detail)))
    }

    pub fn operation(code: StatusCode, detail: Option<String>) -> Self {
        Self::new(ErrorType::Operation { code, detail })
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
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            ErrorType::NotFound(body) => {
                tracing::debug!(error = %body, "entity not found");
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            ErrorType::Validation(body) => {
                tracing::debug!(error = %body, "validation error");
                (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
            }
            ErrorType::Authentication(body) => {
                tracing::debug!(error = %body, "authentication");
                (StatusCode::UNAUTHORIZED, Json(body)).into_response()
            }
            ErrorType::Authorization(body) => {
                tracing::debug!(error = %body, "authorization");
                (StatusCode::FORBIDDEN, Json(body)).into_response()
            }
            ErrorType::Operation {
                code,
                detail: Some(detail),
            } => (code, detail).into_response(),
            ErrorType::Operation { code, detail: _ } => code.into_response(),
            ErrorType::Internal { trace, message } => {
                tracing::error!(
                    location = ?trace.into_iter().map(ToString::to_string).collect::<Vec<_>>(),
                    message,
                    "generic error",
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "internal error"})),
                )
                    .into_response()
            }
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
        let validation_error_body_response =
            MsgPackOrJson::<ValidationErrorBody>::operation_response(ctx, operation).unwrap();

        vec![
            (Some(Code(400)), standard_error_body_response.clone()),
            (Some(Code(401)), standard_error_body_response.clone()),
            (Some(Code(403)), standard_error_body_response),
            (Some(Code(422)), validation_error_body_response),
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
        message: String,
        trace: Vec<&'static Location<'static>>,
    },

    /// Bad user input (to be further refined)
    BadRequest(StandardErrorBody),

    /// Entity not found
    NotFound(StandardErrorBody),

    /// Authentication error
    Authentication(StandardErrorBody),

    /// Authorization error
    Authorization(StandardErrorBody),

    /// An error from validating a request
    Validation(ValidationErrorBody),

    /// An error from an Operation application
    Operation {
        code: StatusCode,
        detail: Option<String>,
    },
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { message, .. } => message.fmt(f),
            Self::BadRequest(s) => write!(f, "bad_request {s}"),
            Self::NotFound(s) => write!(f, "not_found {s}"),
            Self::Authentication(s) => write!(f, "authn {s}"),
            Self::Authorization(s) => write!(f, "authz {s}"),
            Self::Validation(s) => s.fmt(f),
            Self::Operation { detail, code } => {
                if let Some(detail) = detail {
                    detail.fmt(f)
                } else {
                    write!(f, "code {code}")
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
