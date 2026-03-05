use crate::{AppState, error::Result};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    coyote_kv::worker(&state.namespace_state, crate::is_shutting_down).await;
    Ok(())
}
