use super::{CompleteResponse, IdempotencyRequest};
use crate::IdempotencyStore;
use diom_operations::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteOperation {
    pub(crate) key: String,
    pub(crate) response: Vec<u8>,
    pub(crate) ttl_seconds: u64,
}

impl CompleteOperation {
    pub fn new(key: String, response: Vec<u8>, ttl_seconds: u64) -> Self {
        Self {
            key,
            response,
            ttl_seconds,
        }
    }
}

impl CompleteOperation {
    fn apply_real(self, state: &mut IdempotencyStore) -> Result<()> {
        state.complete(&self.key, self.response, self.ttl_seconds)?;
        Ok(())
    }
}

impl IdempotencyRequest for CompleteOperation {
    fn apply(self, state: &mut IdempotencyStore) -> CompleteResponse {
        CompleteResponse(self.apply_real(state))
    }
}
