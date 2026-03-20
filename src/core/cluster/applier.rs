use super::{
    NodeId,
    handle::{Request, RequestWithContext, Response},
    state_machine::Store,
};
use openraft::LogId;
use opentelemetry::{Value, propagation::TextMapPropagator};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::{Instrument, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub(super) async fn apply_request(
    request: RequestWithContext,
    state_machine: &mut Store,
    log_id: LogId<NodeId>,
) -> anyhow::Result<Response> {
    let child_span = info_span!(
        "apply_request",
        module = request.module(),
        timestamp = %request.timestamp,
        log_index = log_id.index
    );
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
    if let Some(hash) = request.hashed_key() {
        child_span.set_attribute("hashed_key", Value::String(hash.into()));
    }

    state_machine.time.bump(request.timestamp);

    let context = coyote_operations::OpContext {
        timestamp: request.timestamp,
        log_index: log_id.index,
        term: log_id.leader_id.term,
    };

    apply_request_with_context(state_machine, context, request.inner)
        .instrument(child_span)
        .await
}

async fn apply_request_with_context(
    state_machine: &mut Store,
    context: coyote_operations::OpContext,
    request: Request,
) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_kv::operations::KvRaftState {
                state: &stores.kv_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::Kv(req.apply(state, &context).await)
        }
        Request::RateLimit(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_rate_limit::operations::RateLimitRaftState {
                state: &stores.rate_limit_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::RateLimit(req.apply(state, &context).await)
        }
        Request::Idempotency(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_idempotency::operations::IdempotencyRaftState {
                state: &stores.idempotency_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::Idempotency(req.apply(state, &context).await)
        }
        Request::Cache(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_cache::operations::CacheRaftState {
                state: &stores.cache_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::Cache(req.apply(state, &context).await)
        }
        Request::Msgs(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_msgs::operations::MsgsRaftState {
                msgs: &stores.msgs_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::Msgs(req.apply(state, &context).await)
        }
        Request::ClusterInternal(req) => {
            Response::ClusterInternal(req.apply(state_machine, &context).await)
        }
        Request::AuthToken(req) => {
            let stores = state_machine.db_handle();
            let state = coyote_auth_token::operations::AuthTokenRaftState {
                state: &stores.auth_token_state,
                namespace: &state_machine.state.namespace_state,
            };
            Response::AuthToken(req.apply(state, &context).await)
        }
    })
}
