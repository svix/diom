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
        Request::CreateKv(req) => {
            Response::CreateKv(req.apply(&state_machine.state.configgroup_state))
        }
        Request::RateLimiter(req) => {
            // Rate limiter neither needs nor uses config groups for now
            Response::RateLimiter(req.apply(&state_machine.state.rate_limiter))
        }
        Request::Idempotency(req) => {
            let mut store = state_machine
                .state
                .get_idempotency_store_by_key(req.key_name())?;
            Response::Idempotency(req.apply(&mut store))
        }
        Request::CreateIdempotency(req) => {
            Response::CreateIdempotency(req.apply(&state_machine.state.configgroup_state))
        }
        Request::Cache(req) => {
            let mut store = state_machine.state.get_cache_store_by_key(req.key_name())?;
            Response::Cache(req.apply(&mut store))
        }
        Request::CreateCache(req) => {
            Response::CreateCache(req.apply(&state_machine.state.configgroup_state))
        }
        Request::Stream(req) => {
            let state = stream::operations::StreamRaftState {
                stream: &state_machine.state.stream_state,
                configgroup: &state_machine.state.configgroup_state,
            };
            Response::Stream(req.apply(state))
        }
        Request::ClusterInternal(req) => Response::ClusterInternal(req.apply(state_machine).await?),
    })
}
