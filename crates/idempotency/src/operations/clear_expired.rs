use super::{ClearExpiredResponse, IdempotencyRequest};
use crate::{State, operations::IdempotencyRaftState};
use diom_operations::Result;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearExpiredOperation {
    storage_type: StorageType,
    max_expirations: usize,
}

impl ClearExpiredOperation {
    pub fn new(storage_type: StorageType) -> Self {
        Self {
            storage_type,
            max_expirations: 10_000, // TODO: make this configurable
        }
    }
}

impl ClearExpiredOperation {
    fn apply_real(self, state: &State, timestamp: jiff::Timestamp) -> Result<()> {
        state
            .controller(self.storage_type)
            .clear_expired(timestamp, self.max_expirations)?;
        Ok(())
    }
}

impl IdempotencyRequest for ClearExpiredOperation {
    fn apply(
        self,
        state: IdempotencyRaftState<'_>,
        timestamp: jiff::Timestamp,
    ) -> ClearExpiredResponse {
        ClearExpiredResponse(self.apply_real(state.state, timestamp))
    }
}
