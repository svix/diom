use diom_error::Result;
use diom_operations::OpContext;
use fjall_utils::StorageType;
use serde::{Deserialize, Serialize};

use super::{CacheRequest, ClearExpiredResponse};
use crate::{State, operations::CacheRaftState};

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
    async fn apply_real(self, state: &State, timestamp: jiff::Timestamp) -> Result<()> {
        state
            .controller(self.storage_type)
            .clear_expired(timestamp, self.max_expirations, self.storage_type)
            .await?;
        Ok(())
    }
}

impl CacheRequest for ClearExpiredOperation {
    async fn apply(self, state: CacheRaftState<'_>, ctx: &OpContext) -> ClearExpiredResponse {
        ClearExpiredResponse::new(self.apply_real(state.state, ctx.timestamp).await)
    }
}
