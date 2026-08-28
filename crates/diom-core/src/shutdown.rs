use std::{sync::LazyLock, time::Duration};
use tokio_util::sync::CancellationToken;

static SHUTTING_DOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static GRACEFUL_SHUTTING_DOWN_TOKEN: LazyLock<CancellationToken> =
    LazyLock::new(CancellationToken::new);

/// Has someone requested shutdown?
pub fn is_shutting_down() -> bool {
    SHUTTING_DOWN_TOKEN.is_cancelled() || GRACEFUL_SHUTTING_DOWN_TOKEN.is_cancelled()
}

/// Request a CancellationToken for the application shut down
pub fn shutting_down_token() -> CancellationToken {
    SHUTTING_DOWN_TOKEN.clone()
}

/// Request a CancellationToken for the application entering pre-shutdown
pub fn graceful_shutting_down_token() -> CancellationToken {
    GRACEFUL_SHUTTING_DOWN_TOKEN.clone()
}

/// Shut down the application immediately
pub fn start_shut_down() {
    if GRACEFUL_SHUTTING_DOWN_TOKEN.is_cancelled() {
        tracing::trace!("shutdown already started");
        return;
    };
    GRACEFUL_SHUTTING_DOWN_TOKEN.cancel();
    SHUTTING_DOWN_TOKEN.cancel();
}

/// Shut down the application with a grace period
pub async fn start_shut_down_gracefully(timeout: Duration) {
    if GRACEFUL_SHUTTING_DOWN_TOKEN.is_cancelled() {
        tracing::debug!("shutdown already started");
        return;
    }
    GRACEFUL_SHUTTING_DOWN_TOKEN.cancel();
    tracing::info!(
        "graceful shutdown requested; waiting {:?} before shutting down",
        timeout
    );
    tokio::time::sleep(timeout).await;
    SHUTTING_DOWN_TOKEN.cancel();
}
