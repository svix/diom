use super::{IdempotencyRequest, Operation, Response};
use coyote_operations::{OperationRequest, OperationResponse, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    pub(crate) key: String,
    pub(crate) response: Vec<u8>,
    pub(crate) ttl_seconds: u64,
    pub(crate) now: Timestamp,
}

impl CompleteOperation {
    pub fn new(key: String, response: Vec<u8>, ttl_seconds: u64, now: Timestamp) -> Self {
        Self {
            key,
            response,
            ttl_seconds,
            now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteResponse(pub Result<()>);

impl OperationResponse for CompleteResponse {
    type ResponseParent = Response;
}

impl OperationRequest for CompleteOperation {
    type Response = CompleteResponse;
    type RequestParent = Operation;
}

impl CompleteOperation {
    fn apply_real(self, state: &mut crate::IdempotencyStore) -> Result<()> {
        state
            .complete(&self.key, self.response, self.ttl_seconds, self.now)
            .map_err(Into::into)
    }
}

impl IdempotencyRequest for CompleteOperation {
    fn apply(self, state: &mut crate::IdempotencyStore) -> CompleteResponse {
        CompleteResponse(self.apply_real(state))
    }
}
