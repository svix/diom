use std::sync::Arc;

use fjall::OwnedWriteBatch;
use parking_lot::RwLock;

use super::{
    NodeId,
    handle::{Request, RequestWithContext, Response},
    state_machine::{Store, Stores},
};
use openraft::LogId;
use opentelemetry::{Value, propagation::TextMapPropagator};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::{Instrument, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Apply a module request (everything except ClusterInternal).
/// Does not require &mut Store, so it can run concurrently with other module requests.
pub(super) async fn apply_module_request(
    request: RequestWithContext,
    stores: Arc<RwLock<Stores>>,
    batch: Arc<RwLock<OwnedWriteBatch>>,
    namespace_state: diom_namespace::State,
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
        let _ = child_span.set_parent(propagator.extract(ctx));
    }
    child_span.set_attribute("request", Value::String(request.to_string().into()));
    if let Some(hash) = request.hashed_key() {
        child_span.set_attribute("hashed_key", Value::String(hash.into()));
    }

    let context = diom_operations::OpContext {
        timestamp: request.timestamp,
        log_index: log_id.index,
        term: log_id.leader_id.term,
        batch,
    };

    apply_module_request_inner(request.inner, stores, namespace_state, &context)
        .instrument(child_span)
        .await
}

async fn apply_module_request_inner(
    request: Request,
    stores: Arc<RwLock<Stores>>,
    namespace_state: diom_namespace::State,
    context: &diom_operations::OpContext,
) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            let kv_state = stores.read().kv_state.clone();
            let state = diom_kv::operations::KvRaftState {
                state: &kv_state,
                namespace: &namespace_state,
            };
            Response::Kv(req.apply(state, context).await)
        }
        Request::RateLimit(req) => {
            let rate_limit_state = stores.read().rate_limit_state.clone();
            let state = diom_rate_limit::operations::RateLimitRaftState {
                state: &rate_limit_state,
                namespace: &namespace_state,
            };
            Response::RateLimit(req.apply(state, context).await)
        }
        Request::Idempotency(req) => {
            let idempotency_state = stores.read().idempotency_state.clone();
            let state = diom_idempotency::operations::IdempotencyRaftState {
                state: &idempotency_state,
                namespace: &namespace_state,
            };
            Response::Idempotency(req.apply(state, context).await)
        }
        Request::Cache(req) => {
            let cache_state = stores.read().cache_state.clone();
            let state = diom_cache::operations::CacheRaftState {
                state: &cache_state,
                namespace: &namespace_state,
            };
            Response::Cache(req.apply(state, context).await)
        }
        Request::Msgs(req) => {
            let msgs_state = stores.read().msgs_state.clone();
            let state = diom_msgs::operations::MsgsRaftState {
                msgs: &msgs_state,
                namespace: &namespace_state,
            };
            Response::Msgs(req.apply(state, context).await)
        }
        Request::AuthToken(req) => {
            let auth_token_state = stores.read().auth_token_state.clone();
            let state = diom_auth_token::operations::AuthTokenRaftState {
                state: &auth_token_state,
                namespace: &namespace_state,
            };
            Response::AuthToken(req.apply(state, context).await)
        }
        Request::ClusterInternal(_) => {
            anyhow::bail!("ClusterInternal requests must be applied via apply_cluster_internal")
        }
    })
}

/// Apply a ClusterInternal request. Requires exclusive access to the Store.
/// These are rare (tick, set cluster UUID, record log timestamp) and act as
/// serialization barriers between parallel segments.
pub(super) async fn apply_cluster_internal(
    request: &RequestWithContext,
    state_machine: &mut Store,
    batch: Arc<RwLock<OwnedWriteBatch>>,
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
        let _ = child_span.set_parent(propagator.extract(ctx));
    }

    let context = diom_operations::OpContext {
        timestamp: request.timestamp,
        log_index: log_id.index,
        term: log_id.leader_id.term,
        batch,
    };

    let Request::ClusterInternal(req) = &request.inner else {
        anyhow::bail!("expected ClusterInternal request")
    };

    Ok(Response::ClusterInternal(
        req.clone()
            .apply(state_machine, &context)
            .instrument(child_span)
            .await,
    ))
}
