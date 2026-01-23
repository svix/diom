use crate::{AppState, error::Result};

pub use coyote_kv::{KvModel, KvStore, OperationBehavior};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    let stores = [&state.kv_store, &state.cache_store.kv];
    coyote_kv::worker(&stores, crate::is_shutting_down).await;
    Ok(())
}
