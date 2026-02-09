use super::{CacheRequest, Operation, Response};
use crate::v1::modules::cache::CacheModel;
use coyote_operations::{OperationRequest, OperationResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    pub(crate) key: String,
    pub(crate) model: CacheModel,
}

impl SetOperation {
    pub fn new(key: String, model: CacheModel) -> Self {
        Self { key, model }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResponse(pub Result<()>);

impl OperationResponse for SetResponse {
    type ResponseParent = Response;
}

impl OperationRequest for SetOperation {
    type Response = SetResponse;
    type RequestParent = Operation;
}

impl SetOperation {
    fn apply_real(self, state: &mut crate::v1::modules::cache::CacheStore) -> Result<()> {
        state.set(&self.key, self.model)?;
        Ok(())
    }
}

impl CacheRequest for SetOperation {
    fn apply(self, state: &mut crate::v1::modules::cache::CacheStore) -> SetResponse {
        SetResponse(self.apply_real(state))
    }
}
