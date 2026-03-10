// SPDX-FileCopyrightText: © 2022 Diom Authors
// SPDX-License-Identifier: MIT

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
// FIXME: Can't use MsgPackOrJson as that would create
// dependency cycle between diom-error and diom-proto
#[expect(clippy::disallowed_types)]
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use tokio::task::JoinError;

mod option_ext;
mod result_ext;
mod validation;

pub use self::{
    option_ext::OptionExt,
    result_ext::ResultExt,
    validation::{ValidationErrorBody, ValidationErrorItem, validation_error, validation_errors},
};

/// A short-hand version of a [`std::result::Result`] that defaults to Diom'es [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned from the Diom API
#[derive(Debug)]
pub struct Error(ErrorType);

impl Error {
    #[track_caller]
    pub fn generic(s: impl fmt::Display) -> Self {
        Self(ErrorType::Generic {
            message: s.to_string(),
            trace: vec![Location::caller()],
        })
    }

    pub fn not_found(detail: impl Into<Option<String>>) -> Self {
        Self(ErrorType::NotFound(StandardErrorBody::new(
            "not_found",
            detail
                .into()
                .unwrap_or_else(|| "Entity not found".to_owned()),
        )))
    }

    pub fn bad_request(code: &'static str, detail: impl fmt::Display) -> Self {
        Self(ErrorType::BadRequest(StandardErrorBody::new(code, detail)))
    }

    pub fn invalid_user_input(detail: impl fmt::Display) -> Self {
        // We'll probably change _how_ invalid user input is displayed later on,
        // but having a universal error function to capture user errors is ideal
        Self::bad_request("invalid_input", detail)
    }

    pub fn validation(detail: Vec<ValidationErrorItem>) -> Self {
        Self(ErrorType::Validation(ValidationErrorBody::new(detail)))
    }

    pub fn operation(code: StatusCode, detail: Option<String>) -> Self {
        Self(ErrorType::Operation { code, detail })
    }

    #[track_caller]
    pub fn trace(mut self) -> Self {
        if let ErrorType::Generic { trace, .. } = &mut self.0 {
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
        match self.0 {
            ErrorType::BadRequest(body) => {
                tracing::debug!(error = %body, "bad request");
                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            ErrorType::NotFound(body) => {
                tracing::debug!(error = %body, "not found");
                (StatusCode::NOT_FOUND, Json(body)).into_response()
            }
            ErrorType::Validation(body) => {
                tracing::debug!(error = %body, "validation error");
                (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
            }
            ErrorType::Operation {
                code,
                detail: Some(detail),
            } => (code, detail).into_response(),
            ErrorType::Operation { code, detail: _ } => code.into_response(),
            ErrorType::Generic { trace, message } => {
                tracing::error!(
                    location = ?trace.into_iter().map(ToString::to_string).collect::<Vec<_>>(),
                    message,
                    "generic error",
                );
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response()
            }
        }
    }
}

impl OperationOutput for Error {
    type Inner = Self;
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
    /// A generic error
    Generic {
        message: String,
        trace: Vec<&'static Location<'static>>,
    },
    /// Bad user input
    BadRequest(StandardErrorBody),
    /// Entity not found
    NotFound(StandardErrorBody),
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
            Self::Generic { message, .. } => message.fmt(f),
            Self::BadRequest(s) => write!(f, "bad_request {s}"),
            Self::NotFound(s) => write!(f, "not_found {s}"),
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

#[derive(Debug, Clone, Serialize)]
pub struct StandardErrorBody {
    code: &'static str,
    detail: String,
}

impl StandardErrorBody {
    pub fn new(code: &'static str, detail: impl fmt::Display) -> Self {
        Self {
            code,
            detail: detail.to_string(),
        }
    }
}

impl fmt::Display for StandardErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { code, detail } = self;
        write!(f, "code={code:?} detail={detail:?}")
    }
}

impl From<fjall::Error> for Error {
    #[track_caller]
    fn from(e: fjall::Error) -> Self {
        Self::generic(format!("{e:?}"))
    }
}

impl From<JoinError> for Error {
    #[track_caller]
    fn from(e: JoinError) -> Self {
        Self::generic(format!("{e:?}"))
    }
}
