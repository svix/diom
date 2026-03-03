use super::{
    NodeId,
    handle::{Request, Response},
    state_machine::Store,
};
use openraft::LogId;

#[tracing::instrument(skip_all, fields(request = %request))]
pub(super) async fn apply_request(
    request: Request,
    state_machine: &mut Store,
    log_id: LogId<NodeId>,
) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            // TODO: this shouldn't be mut but KvStore currently requires it
            let mut store = state_machine
                .state
                .get_kv_store_by_key(req.key_name())
                .await?;
            Response::Kv(req.apply(&mut store))
        }
        Request::CreateKv(req) => {
            Response::CreateKv(req.apply(&state_machine.state.namespace_state))
        }
        Request::RateLimiter(req) => {
            // Rate limiter neither needs nor uses namespaces for now
            Response::RateLimiter(req.apply(&state_machine.state.rate_limiter))
        }
        Request::Idempotency(req) => {
            let mut store = state_machine
                .state
                .get_idempotency_store_by_key(req.key_name())
                .await?;
            Response::Idempotency(req.apply(&mut store))
        }
        Request::CreateIdempotency(req) => {
            Response::CreateIdempotency(req.apply(&state_machine.state.namespace_state))
        }
        Request::Cache(req) => {
            let mut store = state_machine
                .state
                .get_cache_store_by_key(req.key_name())
                .await?;
            Response::Cache(req.apply(&mut store))
        }
        Request::CreateCache(req) => {
            Response::CreateCache(req.apply(&state_machine.state.namespace_state))
        }
        Request::Msgs(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_msgs::operations::MsgsRaftState {
                msgs: &stores.msgs_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::Msgs(req.apply(state))
        }
        Request::ClusterInternal(req) => {
            Response::ClusterInternal(req.apply((state_machine, log_id)).await)
        }
    })
}
