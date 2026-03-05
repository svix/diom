// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]

#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    num::NonZero,
    sync::{Arc, LazyLock},
    time::Duration,
};

use aide::axum::ApiRouter;
use axum::{Extension, middleware, serve::ListenerExt as _};
use cfg::ConfigurationInner;
use diom_error::{Error, HttpError, Result};
use diom_kv::KvStore;
use diom_namespace::{
    BothDatabases,
    entities::{CacheConfig, IdempotencyConfig, KeyValueConfig, ModuleConfig},
    parse_namespace,
};
use lru::LruCache;
use opentelemetry::{InstrumentationScope, metrics::Meter, trace::TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::{SdkMeterProvider, periodic_reader_with_async_runtime::PeriodicReader},
    runtime,
    trace::{
        BatchConfigBuilder, Sampler, SdkTracerProvider,
        span_processor_with_async_runtime::BatchSpanProcessor,
    },
};
use tokio::{
    net::TcpListener,
    sync::{Barrier, Mutex},
};
use tower_http::trace::TraceLayer;

use tokio_util::sync::CancellationToken;
use tower_http::{
    ServiceExt,
    cors::{AllowHeaders, Any, CorsLayer},
    normalize_path::NormalizePath,
};
use tracing_subscriber::{Layer as _, layer::SubscriberExt as _};
use uuid::Uuid;

use crate::{
    cfg::{Configuration, DatabaseConfig},
    core::{
        cluster::RaftState,
        db::{Databases, ReadonlyDatabases},
        otel_spans::{AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator},
    },
};
use diom_cache::CacheStore;
use diom_idempotency::IdempotencyStore;

pub mod bootstrap;
pub mod cfg;
pub mod core;
pub use diom_error as error;
pub mod openapi;
mod serde;
pub mod v1;

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

static SHUTTING_DOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);

/// Has someone requested shutdown?
pub fn is_shutting_down() -> bool {
    SHUTTING_DOWN_TOKEN.is_cancelled()
}

/// Request a CancellationToken for the application shut down
pub fn shutting_down_token() -> CancellationToken {
    SHUTTING_DOWN_TOKEN.clone()
}

/// Shut down the application
pub fn start_shut_down() {
    SHUTTING_DOWN_TOKEN.cancel();
}

pub static INSTANCE_ID: LazyLock<String> = LazyLock::new(|| Uuid::new_v4().to_string());

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

pub async fn run(cfg: Configuration) {
    setup_metrics(&cfg);
    run_with_listeners(cfg, None, None).await
}

#[derive(Clone)]
pub struct AppState {
    cfg: Configuration,
    rate_limiter: v1::modules::rate_limiter::RateLimiter,

    namespace_state: diom_namespace::State,

    // FIXME: do we need this?
    // OTHERFIXME: yes, I think so.
    #[allow(unused)]
    pub(crate) ro_dbs: ReadonlyDatabases,

    kv_stores: Arc<Mutex<LruCache<Option<String>, KvStore>>>,

    pub meter: Meter,
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
        .layer(Extension(raft.clone()))
        .layer(middleware::from_fn(diom_proto::capture_accept_hdr))
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

    let listener = listener.tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    });

    axum::serve(listener, svc)
        .with_graceful_shutdown(shutting_down_token().cancelled_owned())
        .await
        .unwrap();

    raft.raft.shutdown().await.unwrap();
}

impl AppState {
    fn new(cfg: Configuration) -> Self {
        let persistent_db = DatabaseConfig::persistent(&cfg.persistent_db).expect("persistent db");
        let ephemeral_db = DatabaseConfig::ephemeral(&cfg.ephemeral_db).expect("ephemeral db");

        let ro_dbs = Databases::new(persistent_db.clone(), ephemeral_db.clone()).readonly();

        let namespace_state = diom_namespace::State::init(BothDatabases {
            persistent_db: persistent_db.clone(),
            ephemeral_db,
        })
        .expect("initializing namespace state");

        const KV_CACHE: NonZero<usize> = NonZero::new(100).unwrap();

        let meter = opentelemetry::global::meter("diom.svix.com");

        AppState {
            cfg,
            rate_limiter: v1::modules::rate_limiter::RateLimiter::new(
                "rate_limiter_default",
                persistent_db,
            ),
            namespace_state,
            ro_dbs,
            kv_stores: Arc::new(Mutex::new(LruCache::new(KV_CACHE))),
            meter,
        }
    }

    async fn get_store_by_key<C: ModuleConfig>(&self, key_name: &str) -> Result<KvStore> {
        let (ns_name, _) = parse_namespace(key_name);

        let mut cache = self.kv_stores.lock().await;

        // TODO: make sure to invalidate the LruCache when we change any namespace
        // properties; right now, there aren't any endpoints to create or
        // edit Kv/etc namespaces.
        cache
            .try_get_or_insert(ns_name.map(|s| s.to_string()), || -> Result<KvStore> {
                let namespace = self
                    .namespace_state
                    .fetch_namespace::<C>(ns_name)?
                    .ok_or_else(|| Error::http(HttpError::not_found(None, None)))?;

                let policy = namespace.config.eviction_policy();
                Ok(KvStore::new(
                    KeyValueConfig::NAMESPACE,
                    self.namespace_state.give_me_the_right_db(&namespace),
                    policy,
                    None,
                ))
            })
            .cloned()
    }

    pub async fn get_kv_store_by_key(&self, key_name: &str) -> Result<KvStore> {
        self.get_store_by_key::<KeyValueConfig>(key_name).await
    }

    pub async fn get_cache_store_by_key(&self, key_name: &str) -> Result<CacheStore> {
        let kv_store = self.get_store_by_key::<CacheConfig>(key_name).await?;
        Ok(CacheStore::new(kv_store))
    }

    pub async fn get_idempotency_store_by_key(&self, key_name: &str) -> Result<IdempotencyStore> {
        let kv_store = self.get_store_by_key::<IdempotencyConfig>(key_name).await?;
        Ok(IdempotencyStore::new(kv_store))
    }
}

// Made public for the purpose of E2E testing in which a queue prefix is necessary to avoid tests
// consuming from each others' queues
pub async fn run_with_listeners(
    cfg: Configuration,
    listener: Option<TcpListener>,
    interserver_listener: Option<TcpListener>,
) {
    // OpenAPI/aide must be initialized before any routers are constructed
    // because its initialization sets generation-global settings which are
    // needed at router-construction time.
    let mut openapi = openapi::initialize_openapi();

    // build our application with a route
    let app_state = AppState::new(cfg.clone());

    let raft_state = core::cluster::initialize_raft(&cfg, app_state.clone())
        .await
        .expect("failed to initialize cluster");

    let v1_router = v1::router()
        .with_state::<()>(app_state.clone())
        .layer(Extension(raft_state.clone()));

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
        let cfg = cfg.clone();
        async move {
            interserver_started_barrier.wait().await;
            raft_state
                .run_discovery_if_necessary(cfg)
                .await
                .expect("should be able to initialize discovery");
        }
    });

    // Initialize all routes which need to be part of OpenAPI first.
    let api_router = ApiRouter::new()
        .nest_api_service("/api/v1", v1_router)
        .finish_api(&mut openapi);

    openapi::postprocess_spec(&mut openapi);
    let docs_router = docs::router(openapi);
    let router = api_router.merge(docs_router);
    let svc = router
        .layer((
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(AllowHeaders::mirror_request())
                .max_age(Duration::from_secs(600)),
            middleware::from_fn(diom_proto::capture_accept_hdr),
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
    tracing::debug!("API: Listening on {}", listener.local_addr().unwrap());

    // Spawn background workers for each module
    let workers = tokio::spawn(async move {
        tracing::debug!("spawned workers");
        // FIXME: gotta do actual error handling...
        let _ = tokio::join!(
            tokio::spawn(v1::modules::kv::worker(app_state.clone())),
            tokio::spawn(v1::modules::rate_limiter::worker(app_state.clone())),
        );
        tracing::debug!("workers died");
    });

    bootstrap::run(cfg, raft_state)
        .await
        .expect("bootstrapping failed");

    let listener = listener.tap_io(|tcp_stream| {
        if let Err(err) = tcp_stream.set_nodelay(true) {
            tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
        }
    });

    axum::serve(listener, make_svc)
        .with_graceful_shutdown(shutting_down_token().cancelled_owned())
        .await
        .unwrap();

    // Wait for workers to finish cleanup
    tracing::debug!("done serving; waiting for background tasks to finish");
    let _ = workers.await;
    let _ = interserver.await;
    tracing::debug!("we're outta here!");
}

pub fn setup_tracing(
    cfg: &ConfigurationInner,
    for_test: bool,
) -> (tracing::Dispatch, Option<SdkTracerProvider>) {
    let filter_directives = std::env::var("RUST_LOG").unwrap_or_else(|e| {
        if let std::env::VarError::NotUnicode(_) = e {
            eprintln!("RUST_LOG environment variable has non-utf8 contents, ignoring!");
        }

        let level = cfg.log_level.to_string();
        let var = [
            format!("{CRATE_NAME}={level}"),
            format!("diom_kv={level}"),
            format!("diom_msgs={level}"),
            format!("fjall_utils={level}"),
            format!("tower_http={level}"),
            "opentelemetry_sdk=ERROR".to_string(),
        ];

        var.join(",")
    });

    let mapped = cfg.opentelemetry_address.as_ref().map(|addr| {
        // Configure the OpenTelemetry tracing layer
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(addr)
            .build()
            .expect("Failed to build span exporter");

        let batch_span_processor = BatchSpanProcessor::builder(exporter, runtime::Tokio)
            .with_batch_config(
                BatchConfigBuilder::default()
                    .with_max_queue_size(32768)
                    .with_scheduled_delay(Duration::from_secs(3))
                    .build(),
            )
            .build();

        let provider = SdkTracerProvider::builder()
            .with_sampler(
                cfg.opentelemetry_sample_ratio
                    .map(Sampler::TraceIdRatioBased)
                    .unwrap_or(Sampler::AlwaysOn),
            )
            .with_span_processor(batch_span_processor)
            .with_resource(
                opentelemetry_sdk::Resource::builder()
                    .with_service_name(cfg.opentelemetry_service_name.clone())
                    .with_attribute(opentelemetry::KeyValue::new(
                        "instance_id",
                        INSTANCE_ID.as_str(),
                    ))
                    .with_attribute(opentelemetry::KeyValue::new(
                        "service.version",
                        option_env!("GITHUB_SHA").unwrap_or("unknown"),
                    ))
                    .build(),
            )
            .build();

        // Based on the private `build_batch_with_exporter` method from opentelemetry-otlp
        let layer = tracing_opentelemetry::layer().with_tracer(
            provider.tracer_with_scope(
                InstrumentationScope::builder("opentelemetry-otlp")
                    .with_schema_url(opentelemetry_semantic_conventions::SCHEMA_URL)
                    .build(),
            ),
        );

        opentelemetry::global::set_tracer_provider(provider.clone());
        (layer, provider)
    });

    let (otel_layer, otel_tracer_provider) = mapped.unzip();

    // Then create a subscriber with an additional layer printing to stdout.
    // This additional layer is either formatted normally or in JSON format.
    let stdout_layer = if for_test {
        tracing_subscriber::fmt::layer().with_test_writer().boxed()
    } else {
        match cfg.log_format {
            cfg::LogFormat::Default => tracing_subscriber::fmt::layer().boxed(),
            cfg::LogFormat::Json => {
                let fmt = tracing_subscriber::fmt::format().json().flatten_event(true);
                let json_fields = tracing_subscriber::fmt::format::JsonFields::new();

                tracing_subscriber::fmt::layer()
                    .event_format(fmt)
                    .fmt_fields(json_fields)
                    .boxed()
            }
        }
    };

    let dispatch = tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(otel_layer)
        .with(tracing_subscriber::EnvFilter::new(filter_directives))
        .into();

    (dispatch, otel_tracer_provider)
}

pub fn setup_metrics(cfg: &ConfigurationInner) {
    if let Some(addr) = cfg
        .opentelemetry_metrics_address
        .as_ref()
        .or(cfg.opentelemetry_address.as_ref())
    {
        let exporter = if cfg.opentelemetry_metrics_use_http {
            tracing::debug!("sending http otel metrics to {addr}");

            opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_endpoint(addr)
                .build()
                .unwrap()
        } else {
            tracing::debug!("sending grpc otel metrics to {addr}");

            opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_endpoint(addr)
                .with_temporality(opentelemetry_sdk::metrics::Temporality::Delta)
                .build()
                .unwrap()
        };

        let reader = PeriodicReader::builder(exporter, runtime::Tokio)
            .with_interval(Duration::from_secs(
                cfg.opentelemetry_metrics_period_seconds,
            ))
            .build();

        let provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(
                opentelemetry_sdk::Resource::builder()
                    .with_service_name(cfg.opentelemetry_service_name.clone())
                    .with_attribute(opentelemetry::KeyValue::new(
                        "instance_id",
                        INSTANCE_ID.as_str(),
                    ))
                    .with_attribute(opentelemetry::KeyValue::new(
                        "service.version",
                        option_env!("GITHUB_SHA").unwrap_or("unknown"),
                    ))
                    .build(),
            )
            .build();

        opentelemetry::global::set_meter_provider(provider);
    };
}

pub fn setup_tracing_for_tests() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
}

#[cfg(test)]
static TEST_TRACING_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    if !TEST_TRACING_INITIALIZED.load(Ordering::Acquire) {
        setup_tracing_for_tests();
        TEST_TRACING_INITIALIZED.store(true, Ordering::Release);
    }
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
