use super::{AbortResponse, IdempotencyRequest};
use crate::IdempotencyStore;
use coyote_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbortOperation {
    pub(crate) key: String,
}

impl AbortOperation {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl AbortOperation {
    fn apply_real(self, state: &mut IdempotencyStore) -> Result<()> {
        state.abort(&self.key)?;
        Ok(())
    }
}

impl IdempotencyRequest for AbortOperation {
    fn apply(self, state: &mut IdempotencyStore) -> AbortResponse {
        AbortResponse(self.apply_real(state))
    }
}
