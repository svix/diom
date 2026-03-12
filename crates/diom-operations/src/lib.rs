use axum::response::IntoResponse;
use opentelemetry::{Context, trace::TraceContextExt};
use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum BackgroundError {
    NotLeader,
    Other(diom_error::Error),
}

impl std::fmt::Display for BackgroundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLeader => write!(f, "tried to write to a non-leader node"),
            Self::Other(e) => std::fmt::Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for BackgroundError {
    fn description(&self) -> &str {
        "Error while writing from background job"
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::Other(e) => Some(e),
            Self::NotLeader => None,
        }
    }
}

pub type BackgroundResult<T> = std::result::Result<T, BackgroundError>;

/// Macro support module
#[doc(hidden)]
pub mod __reexports {
    pub use paste;
}

#[macro_use]
mod macros;
pub trait OperationResponse: Serialize + DeserializeOwned + Clone + Debug {
    /// The module-level `Response` enum
    type ResponseParent: ModuleResponse + TryInto<Self>;
}

pub trait OperationRequest: Serialize + DeserializeOwned + Clone + Debug {
    /// The specific Response structure for this request
    type Response: OperationResponse;
    type RequestParent: ModuleRequest;
}

pub trait ModuleResponse: Serialize + DeserializeOwned + Clone + Debug {}

pub trait ModuleRequest: Serialize + DeserializeOwned + Clone + Debug {
    type Response: ModuleResponse;
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct OperationRequestMetadata {
    pub trace_context: Option<HashMap<String, String>>,
}

impl From<Context> for OperationRequestMetadata {
    fn from(ctx: Context) -> Self {
        let span = ctx.span();
        let span_ctx = span.span_context();
        let traceparent = format!(
            "00-{}-{}-{}",
            span_ctx.trace_id(),
            span_ctx.span_id(),
            span_ctx.trace_flags().to_u8(),
        );
        OperationRequestMetadata {
            trace_context: Some(HashMap::from([("traceparent".to_string(), traceparent)])),
        }
    }
}

/// diom_error::Error isn't Serialize or Deserialize, and can contain arbitrary
/// inner error types, which makes it very hard to make serialize or deserialize.
/// This is a nerfed error type that can be sent across the raft boundary.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationError {
    #[serde(with = "http_serde::status_code")]
    status: http::StatusCode,
    msg: String,
}

impl From<diom_error::Error> for OperationError {
    fn from(value: diom_error::Error) -> Self {
        let msg = value.to_string();
        Self {
            status: value.into_response().status(),
            msg,
        }
    }
}

impl From<OperationError> for diom_error::Error {
    fn from(value: OperationError) -> Self {
        Self::operation(value.status, None)
    }
}

pub type Result<T> = std::result::Result<T, OperationError>;
