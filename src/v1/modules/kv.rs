use crate::{AppState, error::Result};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState, kv_state: coyote_kv::State) -> Result<()> {
    coyote_kv::worker(kv_state, state.time.clone()).await;
    Ok(())
}
