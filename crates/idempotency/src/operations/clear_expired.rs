use super::{ClearExpiredResponse, IdempotencyRequest};
use crate::{State, operations::IdempotencyRaftState};
use diom_core::PersistableValue;
use diom_error::Result;
use diom_operations::OpContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PersistableValue)]
pub struct ClearExpiredOperation {}

impl ClearExpiredOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl ClearExpiredOperation {
    async fn apply_real(self, state: &State, ctx: &OpContext) -> Result<()> {
        state
            .controller()
            .clear_expired_in_raft(ctx.timestamp)
            .await?;
        Ok(())
    }
}

impl IdempotencyRequest for ClearExpiredOperation {
    async fn apply(self, state: IdempotencyRaftState<'_>, ctx: &OpContext) -> ClearExpiredResponse {
        ClearExpiredResponse::new(self.apply_real(state.state, ctx).await)
    }
}
