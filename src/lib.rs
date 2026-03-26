// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use ::serde::{Serialize, de::DeserializeOwned};
use aide::axum::ApiRouter;
use axum::{
    Extension, extract::DefaultBodyLimit, middleware, response::IntoResponse as _,
    serve::ListenerExt as _,
};
use diom_authorization::RoleId;
use diom_core::Monotime;
use diom_error::Error;
use diom_proto::{InternalClient, InternalRequest, InternalRequestError};
use fjall_utils::{Databases, ReadonlyDatabases};
use http::StatusCode;
use opentelemetry::metrics::Meter;
use tokio::{
    net::TcpListener,
    sync::{Barrier, mpsc},
};
use tower::ServiceExt as _;
use tower_http::{
    ServiceExt,
    catch_panic::CatchPanicLayer,
    cors::{AllowHeaders, Any, CorsLayer},
    normalize_path::NormalizePath,
    trace::TraceLayer,
};

use crate::{
    cfg::{Configuration, DatabaseConfig},
    core::{
        auth::Permissions,
        cluster::RaftState,
        metrics::{ConnectionMetrics, ConnectionType, RequestMetrics},
        otel_spans::{AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator},
    },
    workers::Workers,
};
use diom_core::shutdown::{shutting_down_token, start_shut_down};

pub mod bootstrap;
pub mod cfg;
pub mod core;
pub use diom_error as error;
pub mod openapi;
mod serde;
pub mod v1;
mod workers;

async fn graceful_shutdown_handler() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = sigterm => {},
    }

    tracing::info!("Received shutdown signal. Shutting down gracefully...");
    start_shut_down();
}

#[derive(Clone)]
pub struct AppState {
    cfg: Configuration,

    namespace_state: diom_namespace::State,

    pub(crate) ro_dbs: ReadonlyDatabases,

    // FIXME: temporarily here until we make ro_dbs usable.
    pub(crate) do_not_use_dbs: Databases,

    pub meter: Meter,
    pub request_metrics: Arc<RequestMetrics>,
    pub conn_metrics: Arc<ConnectionMetrics>,

    internal_client: InternalClient,

    pub(crate) auth_token_cache: Arc<parking_lot::RwLock<core::auth::FifoCache<Permissions>>>,

    pub(crate) time: Monotime,
}

#[derive(Debug, Serialize)]
struct MinimalError {
    message: &'static str,
}

fn handle_panic(err: Box<dyn std::any::Any + Send + 'static>) -> axum::response::Response {
    if let Some(err) = err.downcast_ref::<String>() {
        tracing::error!(?err, "Unhandled panic");
    } else if let Some(err) = err.downcast_ref::<&'static str>() {
        tracing::error!(?err, "Unhandled panic");
    } else {
        tracing::error!("Unhandled non-string panic");
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        diom_proto::MsgPackOrJson(MinimalError {
            message: "unhandled internal panic",
        }),
    )
        .into_response()
}

async fn run_interserver(
    cfg: Configuration,
    state: AppState,
    raft: RaftState,
    listener: Option<TcpListener>,
    bind_barrier: Arc<Barrier>,
) {
    let listen_address = cfg.cluster.listen_address(&cfg);
    let listener = match listener {
        Some(l) => l,
        None => TcpListener::bind(listen_address)
            .await
            .expect("Error binding to listen_address"),
    };
    tracing::debug!(
        "Inter-Server: Listening on {}",
        listener.local_addr().unwrap()
    );
    bind_barrier.wait().await;

    let app = core::cluster::router(&cfg)
        .with_state(state.clone())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            core::otel_spans::request_metrics_middleware,
        ))
        .layer(Extension(raft.clone()))
        .layer(DefaultBodyLimit::disable())
        .layer(middleware::from_fn(diom_proto::capture_accept_hdr))
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(AxumOtelSpanCreator)
                .on_response(AxumOtelOnResponse)
                .on_failure(AxumOtelOnFailure),
        );
    let svc = tower::make::Shared::new(
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        NormalizePath::trim_trailing_slash(app),
    );

    let node_id = raft.node_id;
    let listener = listener.tap_io(move |tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
        state
            .conn_metrics
            .accepted(node_id, ConnectionType::Internal);
    });

    axum::serve(listener, svc)
        .with_graceful_shutdown(shutting_down_token().cancelled_owned())
        .await
        .unwrap();

    raft.raft.shutdown().await.unwrap();
}

async fn run_internal(
    api_router: axum::Router,
    mut internal_req_rx: mpsc::Receiver<InternalRequest>,
) {
    let svc = api_router
        .layer((
            TraceLayer::new_for_http()
                .make_span_with(AxumOtelSpanCreator)
                .on_response(AxumOtelOnResponse)
                .on_failure(AxumOtelOnFailure),
            middleware::from_fn(diom_proto::capture_accept_hdr),
            CatchPanicLayer::custom(handle_panic),
        ))
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        .trim_trailing_slash();

    // FIXME: Do we want to delay graceful shutdown of the internal API server
    //        a little compared to public / inter-server?
    let shutdown_tok = shutting_down_token();
    while let Some(Some(mut req)) = shutdown_tok
        .run_until_cancelled(internal_req_rx.recv())
        .await
    {
        // FIXME: Do we want to limit the maximum number of concurrently-running internal requests?
        let svc = svc.clone();
        tokio::spawn(async move {
            req.inner.extensions_mut().insert(Permissions {
                role: RoleId::operator(),
                auth_token_id: None,
            });

            // FIXME: Do we want to cancel request handling when the response channel is closed?
            //        As-is, we always complete request processing even if the internal caller
            //        loses interest (e.g. because it is cancelled itself).
            let response = svc
                .oneshot(req.inner)
                .await
                .unwrap_or_else(|never| match never {});
            _ = req.response_tx.send(response);
        });
    }
}

impl AppState {
    fn new(cfg: Configuration, time: Monotime, internal_client: InternalClient) -> Self {
        let persistent_db = DatabaseConfig::persistent(&cfg.persistent_db).expect("persistent db");
        let ephemeral_db = DatabaseConfig::ephemeral(&cfg.ephemeral_db).expect("ephemeral db");

        let dbs = Databases::new(persistent_db, ephemeral_db);
        let ro_dbs = dbs.readonly();

        let namespace_state =
            diom_namespace::State::init(dbs.clone()).expect("initializing namespace state");

        let meter = opentelemetry::global::meter("diom.svix.com");

        let request_metrics = Arc::new(RequestMetrics::new(&meter));
        let conn_metrics = Arc::new(ConnectionMetrics::new(&meter));

        let mut listen_addr = cfg.listen_address;
        if listen_addr.ip().is_unspecified() {
            listen_addr.set_ip(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
        }

        let auth_token_cache =
            Arc::new(parking_lot::RwLock::new(core::auth::FifoCache::new(10_000)));

        AppState {
            cfg,
            namespace_state,
            ro_dbs,
            do_not_use_dbs: dbs,
            meter,
            request_metrics,
            conn_metrics,
            internal_client,
            auth_token_cache,
            time,
        }
    }

    /// Make an internal call to a specific op id
    pub async fn internal_call<T: Serialize, U: DeserializeOwned>(
        &self,
        op_id: &'static str,
        body: &T,
    ) -> Result<U, InternalRequestError> {
        let path = format!("/api/{op_id}");
        self.internal_client.post(&path, body).await
    }
}

/// Run the server with the given configuration
pub async fn run(cfg: Configuration) {
    run_with_listeners(cfg, None, None, Monotime::initial()).await
}

static BOOTSTRAPPED: AtomicBool = AtomicBool::new(false);

async fn fail_until_bootstrapped(
    path: Option<axum::extract::MatchedPath>,
    request: axum::extract::Request,
    next: middleware::Next,
) -> axum::response::Response {
    let ignore_bootstrap = if let Some(path) = path {
        path.as_str().starts_with("/api/v1.admin.cluster.")
    } else {
        // don't wait for bootstrap for things axum will return a 404 on
        true
    };
    if !(ignore_bootstrap || BOOTSTRAPPED.load(Ordering::Relaxed)) {
        let response = diom_proto::MsgPackOrJson(MinimalError {
            message: "this node has not yet finished bootstarpping",
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, response).into_response();
    }

    next.run(request).await
}

#[derive(Debug, Clone)]
pub struct Initialized {
    inner: Arc<tokio::sync::SetOnce<()>>,
}

impl Initialized {
    fn new() -> Self {
        Self {
            inner: Arc::new(tokio::sync::SetOnce::new()),
        }
    }

    fn set(&self) -> Result<(), tokio::sync::SetOnceError<()>> {
        self.inner.set(())
    }

    // Wait until initialization is finished or the server shuts down
    pub async fn wait(self) -> diom_error::Result<()> {
        let shutting_down_token = shutting_down_token();
        shutting_down_token
            .run_until_cancelled(self.inner.wait())
            .await
            .copied()
            .ok_or_else(Error::shutting_down)
    }
}

/// Run the server with the given configuration and initial state
///
/// This is public for integration tests to use it, but should not be used
/// by any callers other than integration tests
pub async fn run_with_listeners(
    cfg: Configuration,
    listener: Option<TcpListener>,
    interserver_listener: Option<TcpListener>,
    time: Monotime,
) {
    // OpenAPI/aide must be initialized before any routers are constructed
    // because its initialization sets generation-global settings which are
    // needed at router-construction time.
    let mut openapi = openapi::initialize_openapi();

    let initialized = Initialized::new();

    let (internal_req_tx, internal_req_rx) = mpsc::channel(1);
    let internal_client = InternalClient::new(internal_req_tx);
    let app_state = AppState::new(cfg.clone(), time.clone(), internal_client);

    let raft_state =
        core::cluster::initialize_raft(&cfg, app_state.clone(), time, initialized.clone())
            .await
            .expect("failed to initialize cluster");
    let node_id = raft_state.node_id;

    let v1_router = v1::router(Some(app_state.clone()))
        .with_state::<()>(app_state.clone())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            core::otel_spans::request_metrics_middleware,
        ))
        .layer(Extension(raft_state.clone()))
        .layer(middleware::from_fn(fail_until_bootstrapped));

    let interserver_started_barrier = Arc::new(Barrier::new(2));

    tokio::spawn(graceful_shutdown_handler());

    let interserver = tokio::spawn(run_interserver(
        cfg.clone(),
        app_state.clone(),
        raft_state.clone(),
        interserver_listener,
        Arc::clone(&interserver_started_barrier),
    ));

    tokio::spawn({
        let raft_state = raft_state.clone();
        async move {
            interserver_started_barrier.wait().await;
            raft_state
                .run_discovery_if_necessary()
                .await
                .expect("should be able to initialize discovery");
        }
    });

    // Initialize all routes which need to be part of OpenAPI first.
    let api_router = ApiRouter::new()
        .nest_api_service("/api", v1_router)
        .finish_api(&mut openapi);

    tokio::spawn(run_internal(api_router.clone(), internal_req_rx));

    openapi::postprocess_spec(&mut openapi);
    let docs_router = docs::router(openapi);
    let router = api_router.merge(docs_router);
    let svc = router
        .layer((
            TraceLayer::new_for_http()
                .make_span_with(AxumOtelSpanCreator)
                .on_response(AxumOtelOnResponse)
                .on_failure(AxumOtelOnFailure),
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(AllowHeaders::mirror_request())
                .max_age(Duration::from_secs(600)),
            middleware::from_fn(diom_proto::capture_accept_hdr),
            CatchPanicLayer::custom(handle_panic),
        ))
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        .trim_trailing_slash();
    let make_svc = tower::make::Shared::new(svc);

    let listen_address = cfg.listen_address;
    let listener = match listener {
        Some(l) => l,
        None => TcpListener::bind(listen_address)
            .await
            .expect("Error binding to listen_address"),
    };
    tokio::task::spawn({
        let cfg = cfg.clone();
        let raft_state = raft_state.clone();
        let initialized = initialized.clone();
        async move {
            if let Err(err) = bootstrap::run(cfg, raft_state).await {
                tracing::error!(?err, "bootstrap failed");
                start_shut_down();
            }
            BOOTSTRAPPED.store(true, Ordering::SeqCst);
            if initialized.set().is_err() {
                tracing::error!("bootstrap ran twice???");
            }
        }
    });

    tracing::debug!("API: Listening on {}", listener.local_addr().unwrap());

    let listener = listener.tap_io(move |tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
        app_state
            .conn_metrics
            .accepted(node_id, ConnectionType::External);
    });

    let worker_handle = tokio::task::spawn({
        let raft_state = raft_state.clone();
        let shutting_down = shutting_down_token();
        async move {
            initialized.wait().await?;
            let mut workers = Workers::new();
            workers.spawn_all(raft_state).await;
            shutting_down.cancelled().await;
            workers.shutdown().await;
            Ok::<(), Error>(())
        }
    });

    axum::serve(listener, make_svc)
        .with_graceful_shutdown(shutting_down_token().cancelled_owned())
        .await
        .unwrap();

    // Wait for workers to finish cleanup
    tracing::debug!("done serving; waiting for background tasks to finish");
    let _ = worker_handle.await;
    let _ = interserver.await;
    tracing::debug!("we're outta here!");
}

static TEST_TRACING_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn setup_tracing_for_tests() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    if TEST_TRACING_INITIALIZED.load(Ordering::Acquire) {
        return;
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Output is only printed for failing tests, but still we shouldn't overload
                // the output with unnecessary info. When debugging a specific test, it's easy
                // to override this default by setting the `RUST_LOG` environment variable.
                "diom=debug,it=debug,test_utils=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
    TEST_TRACING_INITIALIZED.store(true, Ordering::Release);
}

#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    setup_tracing_for_tests();
}

mod docs {
    use aide::{axum::ApiRouter, openapi::OpenApi};
    use axum::{
        response::{Html, IntoResponse, Redirect},
        routing::get,
    };

    // TODO: switch to generated docs instead of hardcoded JSON once generated
    // is comparable/better than hardcoded one.
    pub(crate) fn router(_docs: OpenApi) -> ApiRouter {
        ApiRouter::new()
            .route("/", get(|| async { Redirect::temporary("/docs") }))
            .route("/docs", get(get_docs))
            .route("/api/v1/openapi.json", get(get_openapi_json))
            .with_state(_docs)
    }

    async fn get_docs() -> Html<&'static str> {
        Html(include_str!("static/docs.html"))
    }

    async fn get_openapi_json() -> impl IntoResponse {
        static BODY: &str = include_str!("../openapi.json");
        ([(http::header::CONTENT_TYPE, "application/json")], BODY)
    }
}
