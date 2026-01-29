use crate::{KvModel, OperationBehavior};

use super::{KvRequest, KvResponse};
use coyote_operations::Result;
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

impl KvResponse for SetResponse {
    type Request = SetOperation;
}

impl SetOperation {
    fn apply_real(self, state: &mut crate::KvStore) -> Result<()> {
        state.set_(&self.key, &self.model, self.behavior)?;
        Ok(())
    }
}

impl KvRequest for SetOperation {
    type Response = SetResponse;

    fn apply(self, state: &mut crate::KvStore) -> Self::Response {
        SetResponse(self.apply_real(state))
    }
}
