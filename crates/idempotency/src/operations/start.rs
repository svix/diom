use super::{IdempotencyRequest, Operation, Response};
use coyote_operations::{OperationRequest, OperationResponse, Result};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartOperation {
    pub(crate) key: String,
    pub(crate) ttl_seconds: u64,
    pub(crate) now: Timestamp,
}

impl StartOperation {
    pub fn new(key: String, ttl_seconds: u64, now: Timestamp) -> Self {
        Self {
            key,
            ttl_seconds,
            now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartResponse(pub Result<Option<Vec<u8>>>);

impl OperationResponse for StartResponse {
    type ResponseParent = Response;
}

impl OperationRequest for StartOperation {
    type Response = StartResponse;
    type RequestParent = Operation;
}

impl StartOperation {
    fn apply_real(self, state: &mut crate::IdempotencyStore) -> Result<Option<Vec<u8>>> {
        state
            .try_start(&self.key, self.ttl_seconds, self.now)
            .map_err(Into::into)
    }
}

impl IdempotencyRequest for StartOperation {
    fn apply(self, state: &mut crate::IdempotencyStore) -> StartResponse {
        StartResponse(self.apply_real(state))
    }
}
