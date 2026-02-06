use crate::AppState;

use super::handle::{Request, Response};

pub(super) fn apply_request(request: Request, state: &AppState) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            // TODO: this shouldn't be mut but KvStore currently requires it
            let mut store = state.get_kv_store_by_key(req.key_name())?;
            Response::Kv(req.apply(&mut store))
        }
        Request::RateLimiter(req) => {
            // Rate limiter doesn't need nor use config groups for now
            Response::RateLimiter(req.apply(&state.rate_limiter))
        }
    })
}
