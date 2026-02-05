use crate::{KvModel, OperationBehavior};

use super::{KvRequest, KvResponse, Response};
use coyote_operations::{OperationRequest, OperationResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    pub(crate) key: String,
    pub(crate) model: KvModel,
    pub(crate) behavior: OperationBehavior,
}

impl SetOperation {
    pub fn new(key: String, model: KvModel, behavior: OperationBehavior) -> Self {
        Self {
            key,
            model,
            behavior,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResponse(pub Result<()>);

impl KvResponse for SetResponse {}

impl OperationResponse for SetResponse {
    type ResponseParent = Response;
}

impl OperationRequest for SetOperation {
    type Response = SetResponse;
}

impl SetOperation {
    fn apply_real(self, state: &mut crate::KvStore) -> Result<()> {
        state.set_(&self.key, &self.model, self.behavior)?;
        Ok(())
    }
}

impl KvRequest for SetOperation {
    fn apply(self, state: &mut crate::KvStore) -> SetResponse {
        SetResponse(self.apply_real(state))
    }
}
