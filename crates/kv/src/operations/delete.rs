use super::{DeleteResponse, KvRequest};
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
