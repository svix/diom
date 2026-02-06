use super::{
    handle::{Request, Response},
    state_machine::Store,
};

pub(super) async fn apply_request(
    request: Request,
    state_machine: &mut Store,
) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            // TODO: this shouldn't be mut but KvStore currently requires it
            let mut store = state_machine.state.get_kv_store_by_key(req.key_name())?;
            Response::Kv(req.apply(&mut store))
        }
        Request::ClusterInternal(req) => Response::ClusterInternal(req.apply(state_machine).await?),
        Request::RateLimiter(req) => {
            // Rate limiter neither needs nor uses config groups for now
            Response::RateLimiter(req.apply(&state_machine.state.rate_limiter))
        }
    })
}
