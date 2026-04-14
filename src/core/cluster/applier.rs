use std::sync::Arc;

use super::{
    LogId,
    handle::{Request, RequestWithContext, Response},
    state_machine::{Store, Stores},
};
use crate::AppState;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::{Instrument, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// The context for a single batch of apply operations.
///
/// Holds a (read) lock against the databases as long as it's live
pub(super) struct ApplyContext {
    pub stores: parking_lot::ArcRwLockReadGuard<parking_lot::RawRwLock, Stores>,
    pub state: AppState,
}

impl ApplyContext {
    pub(super) fn new(state: &Store) -> Self {
        let stores = state.db_handle();
        let state = state.state.clone();
        Self { stores, state }
    }
}

pub(super) async fn apply_request(
    state_context: &ApplyContext,
    request: Arc<RequestWithContext>,
    state_machine: &mut Store,
    log_id: LogId,
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
    child_span.set_attribute("request", request.module());
    if let Some(hash) = request.hashed_key() {
        child_span.set_attribute("hashed_key", hash);
    }

    state_machine.time.update_from_other(request.timestamp);

    let context = diom_operations::OpContext {
        timestamp: request.timestamp,
        log_index: log_id.index,
        term: log_id.leader_id.term,
        rng_seed: request.rng_seed,
    };

    let request = Arc::unwrap_or_clone(request);

    apply_request_with_context(state_context, context, state_machine, request.inner)
        .instrument(child_span)
        .await
}

async fn apply_request_with_context(
    state_context: &ApplyContext,
    context: diom_operations::OpContext,
    state_machine: &mut Store,
    request: Request,
) -> anyhow::Result<Response> {
    Ok(match request {
        Request::Kv(req) => {
            let state = diom_kv::operations::KvRaftState {
                state: &state_context.stores.kv_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::Kv(req.apply(state, &context).await)
        }
        Request::RateLimit(req) => {
            let state = diom_rate_limit::operations::RateLimitRaftState {
                state: &state_context.stores.rate_limit_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::RateLimit(req.apply(state, &context).await)
        }
        Request::Idempotency(req) => {
            let state = diom_idempotency::operations::IdempotencyRaftState {
                state: &state_context.stores.idempotency_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::Idempotency(req.apply(state, &context).await)
        }
        Request::Cache(req) => {
            let state = diom_cache::operations::CacheRaftState {
                state: &state_context.stores.cache_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::Cache(req.apply(state, &context).await)
        }
        Request::Msgs(req) => {
            let state = diom_msgs::operations::MsgsRaftState {
                msgs: &state_context.stores.msgs_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::Msgs(req.apply(state, &context).await)
        }
        Request::ClusterInternal(req) => {
            Response::ClusterInternal(req.apply(state_machine, &context).await)
        }
        Request::AuthToken(req) => {
            let state = diom_auth_token::operations::AuthTokenRaftState {
                state: &state_context.stores.auth_token_state,
                namespace: &state_context.state.namespace_state,
            };
            Response::AuthToken(req.apply(state, &context).await)
        }
        Request::AdminAuth(req) => {
            let state = diom_admin_auth::operations::AdminAuthRaftState {
                state: &state_context.stores.admin_auth_state,
            };
            Response::AdminAuth(req.apply(state, &context).await)
        }
    })
}
