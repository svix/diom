use super::{CacheRequest, DeleteResponse};
use crate::CacheStore;
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
    fn apply_real(self, state: &mut CacheStore) -> Result<()> {
        state.delete(&self.key)?;
        Ok(())
    }
}

impl CacheRequest for DeleteOperation {
    fn apply(self, state: &mut CacheStore) -> DeleteResponse {
        DeleteResponse(self.apply_real(state))
    }
}
