use crate::{AppState, error::Result};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    coyote_cache::worker(state.do_not_use_dbs.clone(), crate::is_shutting_down).await;
    Ok(())
}
