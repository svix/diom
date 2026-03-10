use std::sync::LazyLock;
use tokio_util::sync::CancellationToken;

static SHUTTING_DOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);

/// Has someone requested shutdown?
pub fn is_shutting_down() -> bool {
    SHUTTING_DOWN_TOKEN.is_cancelled()
}

/// Request a CancellationToken for the application shut down
pub fn shutting_down_token() -> CancellationToken {
    SHUTTING_DOWN_TOKEN.clone()
}

/// Shut down the application
pub fn start_shut_down() {
    SHUTTING_DOWN_TOKEN.cancel();
}
