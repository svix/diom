use crate::core::cluster::handle::Request;

use super::{
    NodeId,
    handle::{RequestWithContext, Response},
    state_machine::Store,
};
use openraft::LogId;
use opentelemetry::{Value, propagation::TextMapPropagator};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::info_span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub(super) async fn apply_request(
    request: RequestWithContext,
    state_machine: &mut Store,
    log_id: LogId<NodeId>,
) -> anyhow::Result<Response> {
    let child_span = info_span!("apply_request");
    let trace_ctx = request
        .context
        .as_ref()
        .and_then(|x| x.trace_context.as_ref());
    if let Some(ctx) = trace_ctx {
        let propagator = TraceContextPropagator::new();
        // This should only fail if OTEL is not enabled:
        let _ = child_span.set_parent(propagator.extract(ctx));
    }
    child_span.set_attribute("request", Value::String(request.to_string().into()));
    let _exit = child_span.enter();

    Ok(match request.inner {
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
