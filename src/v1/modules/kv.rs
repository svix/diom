use crate::{AppState, error::Result};

pub use diom_kv::{KvModel, KvStore, OperationBehavior};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    diom_kv::worker(&state.namespace_state, crate::is_shutting_down).await;
    Ok(())
}
