use super::{AbandonResponse, IdempotencyRequest};
use crate::IdempotencyStore;
use coyote_operations::Result;
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

impl AbandonOperation {
    fn apply_real(self, state: &mut IdempotencyStore) -> Result<()> {
        state.abandon(&self.key)?;
        Ok(())
    }
}

impl IdempotencyRequest for AbandonOperation {
    fn apply(self, state: &mut IdempotencyStore) -> AbandonResponse {
        AbandonResponse(self.apply_real(state))
    }
}
