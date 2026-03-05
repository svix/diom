// SPDX-FileCopyrightText: © 2022 Diom Authors
// SPDX-License-Identifier: MIT

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
// FIXME: Can't use MsgPackOrJson as that would create
// dependency cycle between diom-error and diom-proto
#[expect(clippy::disallowed_types)]
use axum::{
    Json,
    extract::rejection::{ExtensionRejection, PathRejection},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use tokio::task::JoinError;

mod result_ext;
mod validation;

pub use self::{
    result_ext::ResultExt,
    validation::{ValidationErrorItem, ValidationHttpError, validation_error, validation_errors},
};

/// A short-hand version of a [`std::result::Result`] that defaults to Diom'es [Error].
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned from the Diom API
#[derive(Debug)]
pub struct Error {
    // the file name and line number of the error. Used for debugging non Http errors
    pub trace: Vec<&'static Location<'static>>,
    pub typ: ErrorType,
}

impl Error {
    #[track_caller]
    fn new(typ: ErrorType) -> Self {
        let trace = vec![Location::caller()];
        Self { trace, typ }
    }

    #[track_caller]
    pub fn generic(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Generic(s.to_string()))
    }

    #[track_caller]
    pub fn invalid_user_input(s: impl fmt::Display) -> Self {
        // We'll probably change _how_ invalid user input is displayed later on,
        // but having a universal error function to capture user errors is ideal
        Self::new(ErrorType::Http(HttpError {
            status: StatusCode::BAD_REQUEST,
            body: HttpErrorBody::Standard(StandardHttpError {
                code: "invalid_input".to_owned(),
                detail: s.to_string(),
            }),
        }))
    }

    #[track_caller]
    pub fn http(h: HttpError) -> Self {
        Self {
            trace: Vec::with_capacity(0), // no debugging necessary
            typ: ErrorType::Http(h),
        }
    }

    #[track_caller]
    pub fn operation(code: StatusCode, detail: Option<String>) -> Self {
        Self::new(ErrorType::Operation { code, detail })
    }

    #[track_caller]
    pub fn trace(mut self) -> Self {
        self.trace.push(Location::caller());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.typ.fmt(f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let stringified: Vec<String> = self.trace.into_iter().map(ToString::to_string).collect();
        match self.typ {
            ErrorType::Http(s) => {
                tracing::debug!("{:?}, location: {:?}", &s, stringified);
                s.into_response()
            }
            ErrorType::Operation {
                code,
                detail: Some(detail),
            } => (code, detail).into_response(),
            ErrorType::Operation { code, detail: _ } => code.into_response(),
            s => {
                tracing::error!("type: {:?}, location: {:?}", s, stringified);
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

impl From<ExtensionRejection> for Error {
    #[track_caller]
    fn from(value: ExtensionRejection) -> Self {
        Error::generic(value)
    }
}

impl From<PathRejection> for Error {
    #[track_caller]
    fn from(value: PathRejection) -> Self {
        Error::generic(value)
    }
}

#[derive(Debug)]
pub enum ErrorType {
    /// A generic error
    Generic(String),
    /// Any kind of HttpError
    Http(HttpError),
    /// An error from an Operation application
    Operation {
        code: StatusCode,
        detail: Option<String>,
    },
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) => s.fmt(f),
            Self::Http(s) => s.fmt(f),
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

impl From<HttpError> for ErrorType {
    fn from(e: HttpError) -> Self {
        Self::Http(e)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StandardHttpError {
    code: String,
    detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpErrorBody {
    Standard(StandardHttpError),
    Validation(ValidationHttpError),
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: StatusCode,
    body: HttpErrorBody,
}

impl HttpError {
    fn new_standard(status: StatusCode, code: String, detail: String) -> Self {
        Self {
            status,
            body: HttpErrorBody::Standard(StandardHttpError { code, detail }),
        }
    }

    pub fn bad_request(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::BAD_REQUEST,
            code.unwrap_or_else(|| "generic_error".to_owned()),
            detail.unwrap_or_else(|| "Generic error".to_owned()),
        )
    }

    pub fn not_found(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::NOT_FOUND,
            code.unwrap_or_else(|| "not_found".to_owned()),
            detail.unwrap_or_else(|| "Entity not found".to_owned()),
        )
    }

    pub fn unauthorized(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::UNAUTHORIZED,
            code.unwrap_or_else(|| "authentication_failed".to_owned()),
            detail.unwrap_or_else(|| "Incorrect authentication credentials.".to_owned()),
        )
    }

    pub fn permission_denied(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::FORBIDDEN,
            code.unwrap_or_else(|| "insufficient access".to_owned()),
            detail.unwrap_or_else(|| "Insufficient access for the given operation.".to_owned()),
        )
    }

    pub fn conflict(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::CONFLICT,
            code.unwrap_or_else(|| "conflict".to_owned()),
            detail.unwrap_or_else(|| "A conflict has occurred".to_owned()),
        )
    }

    pub fn unprocessable_entity(detail: Vec<ValidationErrorItem>) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            body: HttpErrorBody::Validation(ValidationHttpError { detail }),
        }
    }

    pub fn internal_server_error(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::INTERNAL_SERVER_ERROR,
            code.unwrap_or_else(|| "server_error".to_owned()),
            detail.unwrap_or_else(|| "Internal Server Error".to_owned()),
        )
    }

    pub fn too_large(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::PAYLOAD_TOO_LARGE,
            code.unwrap_or_else(|| "payload_too_large".to_owned()),
            detail.unwrap_or_else(|| "Request payload is too large.".to_owned()),
        )
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error::http(err)
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.body {
            HttpErrorBody::Standard(StandardHttpError { code, detail }) => write!(
                f,
                "status={} code=\"{code}\" detail=\"{detail}\"",
                self.status
            ),

            HttpErrorBody::Validation(ValidationHttpError { detail }) => {
                write!(
                    f,
                    "status={} detail={}",
                    self.status,
                    serde_json::to_string(&detail)
                        .unwrap_or_else(|e| format!("\"unserializable error for {e}\""))
                )
            }
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl From<ErrorType> for Error {
    fn from(typ: ErrorType) -> Self {
        Self { trace: vec![], typ }
    }
}

impl From<fjall::Error> for Error {
    #[track_caller]
    fn from(e: fjall::Error) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Generic(format!("{e:?}")),
        }
    }
}

impl From<JoinError> for Error {
    #[track_caller]
    fn from(e: JoinError) -> Self {
        Self {
            trace: vec![Location::caller()],
            typ: ErrorType::Generic(format!("{e:?}")),
        }
    }
}
