use crate::{AppState, error::Result};

pub use diom_kv::{EvictionPolicy, KvModel, KvStore, OperationBehavior};

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(mut state: AppState) -> Result<()> {
    let mut stores = [&mut state.kv_store, &mut state.cache_store.kv];
    diom_kv::worker(&mut stores, crate::is_shutting_down).await;
    Ok(())
}
