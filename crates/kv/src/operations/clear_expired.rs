use super::{ClearExpiredResponse, KvRequest};
use crate::{State, operations::KvRaftState};
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

impl KvRequest for ClearExpiredOperation {
    async fn apply(self, state: KvRaftState<'_>, ctx: &OpContext) -> ClearExpiredResponse {
        ClearExpiredResponse::new(self.apply_real(state.state, ctx).await)
    }
}
