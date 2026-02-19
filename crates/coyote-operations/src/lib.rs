use axum::response::IntoResponse;
use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

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

/// coyote_error::Error isn't Serialize or Deserialize, and can contain arbitrary
/// inner error types, which makes it very hard to make serialize or deserialize.
/// This is a nerfed error type that can be sent across the raft boundary.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationError {
    #[serde(with = "http_serde::status_code")]
    status: http::StatusCode,
}

impl From<coyote_error::Error> for OperationError {
    fn from(value: coyote_error::Error) -> Self {
        Self {
            status: value.into_response().status(),
        }
    }
}

impl From<OperationError> for coyote_error::Error {
    fn from(value: OperationError) -> Self {
        Self::operation(value.status, None)
    }
}

pub type Result<T> = std::result::Result<T, OperationError>;
