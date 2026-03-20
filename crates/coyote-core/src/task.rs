use tracing::Span;

pub fn spawn_blocking_in_current_span<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> tokio::task::JoinHandle<T> {
    let current_span = Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
