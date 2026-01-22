// SPDX-FileCopyrightText: © 2022 Diom Authors
// SPDX-License-Identifier: MIT

use std::{error, fmt, panic::Location};

use aide::OperationOutput;
use axum::{
    Json,
    extract::rejection::{ExtensionRejection, PathRejection},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use tokio::task::JoinError;

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
    pub fn validation(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Validation(s.to_string()))
    }

    #[track_caller]
    pub fn http(h: HttpError) -> Self {
        Self {
            trace: Vec::with_capacity(0), // no debugging necessary
            typ: ErrorType::Http(h),
        }
    }

    #[track_caller]
    pub fn timeout(s: impl fmt::Display) -> Self {
        Self::new(ErrorType::Timeout(s.to_string()))
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
    /// Database error
    Validation(String),
    /// Any kind of HttpError
    Http(HttpError),
    /// Timeout error
    Timeout(String),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) | Self::Validation(s) | Self::Timeout(s) => s.fmt(f),
            Self::Http(s) => s.fmt(f),
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
pub struct ValidationHttpError {
    detail: Vec<ValidationErrorItem>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum HttpErrorBody {
    Standard(StandardHttpError),
    Validation(ValidationHttpError),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, JsonSchema)]
/// Validation errors have their own schema to provide context for invalid requests eg. mismatched
/// types and out of bounds values. There may be any number of these per 422 UNPROCESSABLE ENTITY
/// error.
pub struct ValidationErrorItem {
    /// The location as a [`Vec`] of [`String`]s -- often in the form `["body", "field_name"]`,
    /// `["query", "field_name"]`, etc. They may, however, be arbitrarily deep.
    pub loc: Vec<String>,

    /// The message accompanying the validation error item.
    pub msg: String,

    /// The type of error, often "type_error" or "value_error", but sometimes with more context like
    /// as "value_error.number.not_ge"
    #[serde(rename = "type")]
    pub ty: String,
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

    pub fn not_implemented(code: Option<String>, detail: Option<String>) -> Self {
        Self::new_standard(
            StatusCode::NOT_IMPLEMENTED,
            code.unwrap_or_else(|| "not_implemented".to_owned()),
            detail.unwrap_or_else(|| "This API endpoint is not yet implemented.".to_owned()),
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
