use axum::response::IntoResponse;
use opentelemetry::{Context, trace::TraceContextExt};
use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

mod context;
pub use context::OpContext;
pub mod workers;

#[derive(Debug)]
pub enum BackgroundError {
    NotLeader,
    InvalidResponse,
    TooManyPanics,
    Other(coyote_error::Error),
}

impl std::fmt::Display for BackgroundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotLeader => write!(f, "tried to write to a non-leader node"),
            Self::InvalidResponse => write!(f, "raft layer returned invalid response type"),
            Self::TooManyPanics => write!(
                f,
                "a background worker experienced too many panics in a row"
            ),
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
            Self::InvalidResponse | Self::NotLeader | Self::TooManyPanics => None,
        }
    }
}

pub type BackgroundResult<T> = std::result::Result<T, BackgroundError>;

// Not part of the trait below so do_write_request only has to be codegen'ed once
pub trait OperationWriterBase {
    type Request: Sized;
    type Response: Sized;

    /// Execute a write against the replicated state machine
    ///
    /// This should typically not be called by users
    #[allow(async_fn_in_trait)]
    async fn do_write_request(&self, request: Self::Request) -> BackgroundResult<Self::Response>;
}

// The M generic is not required for anything (the only impl is generic over it)
// and could be replaced by further bounds in write_request, but this is more
// convenient for users of the trait.
pub trait OperationWriter<M: ModuleRequest>:
    OperationWriterBase<Request: From<M>, Response: TryInto<M::Response>>
{
    /// Execute an operation against the replicated state machine
    ///
    /// This calls `.do_write_request` internally, and takes care of wrapping/unwrapping
    /// the request appropriately.
    #[allow(async_fn_in_trait)]
    async fn write_request<O>(&self, op: O) -> BackgroundResult<O::Response>
    where
        O: OperationRequest<RequestParent = M, Response: TryFrom<M::Response>>
            + Into<O::RequestParent>,
    {
        let module_request: O::RequestParent = op.into();
        let top_level_request: Self::Request = module_request.into();
        let top_level_response = self.do_write_request(top_level_request).await?;
        let Ok(module_response): std::result::Result<M::Response, _> =
            top_level_response.try_into()
        else {
            return Err(BackgroundError::InvalidResponse);
        };
        module_response
            .try_into()
            .map_err(|_| BackgroundError::InvalidResponse)
    }
}

impl<T, M: ModuleRequest> OperationWriter<M> for T where
    T: OperationWriterBase<Request: From<M>, Response: TryInto<M::Response>>
{
}

/// Macro support module
#[doc(hidden)]
pub mod __reexports {
    pub use pastey;
    pub use serde;
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

/// coyote_error::Error isn't Serialize or Deserialize, and can contain arbitrary
/// inner error types, which makes it very hard to make serialize or deserialize.
/// This is a nerfed error type that can be sent across the raft boundary.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationError {
    #[serde(with = "http_serde::status_code")]
    status: http::StatusCode,
    msg: String,
}

impl From<coyote_error::Error> for OperationError {
    fn from(value: coyote_error::Error) -> Self {
        let msg = value.to_string();
        Self {
            status: value.into_response().status(),
            msg,
        }
    }
}

impl From<OperationError> for coyote_error::Error {
    fn from(value: OperationError) -> Self {
        Self::operation(value.status, None)
    }
}

pub type Result<T> = std::result::Result<T, OperationError>;
