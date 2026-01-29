use axum::response::IntoResponse;
use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait OperationResponse {}

impl<T: Serialize + DeserializeOwned + Clone> OperationResponse for T {}

pub trait OperationRequest {}

impl<T: Serialize + DeserializeOwned + Clone> OperationRequest for T {}

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
