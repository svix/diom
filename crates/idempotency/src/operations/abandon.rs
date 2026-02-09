use super::{IdempotencyRequest, Operation, Response};
use coyote_operations::{OperationRequest, OperationResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonOperation {
    pub(crate) key: String,
}

impl AbandonOperation {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbandonResponse(pub Result<()>);

impl AbandonOperation {
    fn apply_real(self, state: &mut crate::IdempotencyStore) -> Result<()> {
        state.abandon(&self.key).map_err(Into::into)
    }
}

impl IdempotencyRequest for AbandonOperation {
    fn apply(self, state: &mut crate::IdempotencyStore) -> AbandonResponse {
        AbandonResponse(self.apply_real(state))
    }
}

impl OperationResponse for AbandonResponse {
    type ResponseParent = Response;
}

impl OperationRequest for AbandonOperation {
    type Response = AbandonResponse;
    type RequestParent = Operation;
}
