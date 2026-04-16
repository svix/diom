use super::handle::APPLIED_LOG_ID;
use axum::{extract::Request, middleware::Next, response::Response};
use parking_lot::Mutex;

pub(crate) fn capture_log_id(request: Request, next: Next) -> impl Future<Output = Response> {
    APPLIED_LOG_ID.scope(Mutex::new(None), async {
        let mut response = next.run(request).await;
        if let Some(log_id) = APPLIED_LOG_ID.with(|val| *val.lock()) {
            // TODO: also include the leader ID and term
            response
                .headers_mut()
                .insert("Diom-Mutation-Version", log_id.index.into());
        }
        response
    })
}
