use crate::{AppState, error::Result};

pub use coyote_rate_limit::State as RateLimiter;

/// This is the worker function for this module, it does background cleanup and accounting.
pub async fn worker(state: AppState) -> Result<()> {
    let stores = [&state.rate_limiter];
    coyote_rate_limit::worker(&stores, crate::is_shutting_down).await;
    Ok(())
}
