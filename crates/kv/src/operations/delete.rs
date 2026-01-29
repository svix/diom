use super::{KvRequest, KvResponse};
use coyote_operations::Result;
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

impl KvResponse for DeleteResponse {
    type Request = DeleteOperation;
}

impl DeleteOperation {
    fn apply_real(self, state: &mut crate::KvStore) -> Result<()> {
        state.delete(&self.key)?;
        Ok(())
    }
}

impl KvRequest for DeleteOperation {
    type Response = DeleteResponse;

    fn apply(self, state: &mut crate::KvStore) -> Self::Response {
        DeleteResponse(self.apply_real(state))
    }
}
