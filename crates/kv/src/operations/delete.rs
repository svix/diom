use super::{KvRequest, KvResponse, Response};
use coyote_operations::{OperationRequest, OperationResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOperation {
    pub(crate) key: String,
}

impl DeleteOperation {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse(pub Result<()>);

impl KvResponse for DeleteResponse {}

impl DeleteOperation {
    fn apply_real(self, state: &mut crate::KvStore) -> Result<()> {
        state.delete(&self.key)?;
        Ok(())
    }
}

impl KvRequest for DeleteOperation {
    fn apply(self, state: &mut crate::KvStore) -> DeleteResponse {
        DeleteResponse(self.apply_real(state))
    }
}

impl OperationResponse for DeleteResponse {
    type ResponseParent = Response;
}

impl OperationRequest for DeleteOperation {
    type Response = DeleteResponse;
}
