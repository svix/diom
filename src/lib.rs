// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]

use std::{sync::LazyLock, time::Duration};

use aide::axum::ApiRouter;
use cfg::ConfigurationInner;
use coyote_configgroup::{
    BothDatabases,
    entities::{CacheConfig, EvictionPolicy, IdempotencyConfig, KeyValueConfig},
    group_name,
};
use coyote_error::Result;
use coyote_kv::KvStore;
use opentelemetry::{InstrumentationScope, trace::TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::{SdkMeterProvider, periodic_reader_with_async_runtime::PeriodicReader},
    runtime,
    trace::{
        BatchConfigBuilder, Sampler, SdkTracerProvider,
        span_processor_with_async_runtime::BatchSpanProcessor,
    },
};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::{
    cors::{AllowHeaders, Any, CorsLayer},
    normalize_path::NormalizePath,
};
use tracing_subscriber::{Layer as _, layer::SubscriberExt as _};
use uuid::Uuid;

use crate::{
    cfg::{Configuration, DatabaseConfig},
    v1::modules::{cache::CacheStore, idempotency::IdempotencyStore},
};

pub mod cfg;
pub mod core;
pub use coyote_error as error;
pub mod openapi;
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
    run_with_prefix(cfg, None, None).await
}

#[derive(Clone)]
pub struct AppState {
    cfg: Configuration,
    rate_limiter: crate::v1::modules::rate_limiter::RateLimiter,
    queue_store: crate::v1::modules::queue::QueueStore,

    raft: core::cluster::Raft,
    node_id: core::cluster::NodeId,

    stream_state: stream::State,
    configgroup_state: coyote_configgroup::State,

    // TODO: Remove this once we have proper default config groups
    persistent_db: fjall::Database,
}

async fn run_interserver(cfg: Configuration, state: AppState, listener: Option<TcpListener>) {
    let listen_address = cfg.cluster.listen_address;
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

    let app = core::cluster::router().with_state(state.clone());
    let svc = tower::make::Shared::new(
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        NormalizePath::trim_trailing_slash(app),
    );

    axum::serve(listener, svc)
        .with_graceful_shutdown(graceful_shutdown_handler())
        .await
        .unwrap();

    state.raft.shutdown().await.unwrap();
}

impl AppState {
    // FIXME: Blocking
    pub fn kv_store_by_key(&self, key_name: &str) -> Result<KvStore> {
        let Some(group_name_str) = group_name(key_name) else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(KvStore::new(
                KeyValueConfig::NAMESPACE,
                self.persistent_db.clone(),
                EvictionPolicy::NoEviction,
            ));
        };

        let Some(group) = self
            .configgroup_state
            .fetch_group::<KeyValueConfig>(group_name_str.to_string())?
        else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(KvStore::new(
                KeyValueConfig::NAMESPACE,
                self.persistent_db.clone(),
                EvictionPolicy::NoEviction,
            ));
        };

        let policy = group.config.eviction_policy();
        let kv_store = KvStore::new(
            KeyValueConfig::NAMESPACE,
            self.configgroup_state.give_me_the_right_db(&group),
            policy,
        );

        Ok(kv_store)
    }

    // FIXME: Blocking
    pub fn cache_store_by_key(&self, key_name: &str) -> Result<CacheStore> {
        let Some(group_name_str) = group_name(key_name) else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(CacheStore {
                kv: KvStore::new(
                    CacheConfig::NAMESPACE,
                    self.persistent_db.clone(),
                    EvictionPolicy::NoEviction,
                ),
            });
        };

        let Some(group) = self
            .configgroup_state
            .fetch_group::<CacheConfig>(group_name_str.to_string())?
        else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(CacheStore {
                kv: KvStore::new(
                    CacheConfig::NAMESPACE,
                    self.persistent_db.clone(),
                    EvictionPolicy::NoEviction,
                ),
            });
        };

        let policy = group.config.eviction_policy();
        let kv_store = KvStore::new(
            CacheConfig::NAMESPACE,
            self.configgroup_state.give_me_the_right_db(&group),
            policy,
        );

        Ok(CacheStore { kv: kv_store })
    }

    // FIXME: Blocking
    pub fn idempotency_store_by_key(&self, key_name: &str) -> Result<IdempotencyStore> {
        let Some(group_name_str) = group_name(key_name) else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(IdempotencyStore {
                kv: KvStore::new(
                    IdempotencyConfig::NAMESPACE,
                    self.persistent_db.clone(),
                    EvictionPolicy::NoEviction,
                ),
            });
        };

        let Some(group) = self
            .configgroup_state
            .fetch_group::<IdempotencyConfig>(group_name_str.to_string())?
        else {
            // FIXME: This should be a default ConfigGroup struct
            return Ok(IdempotencyStore {
                kv: KvStore::new(
                    IdempotencyConfig::NAMESPACE,
                    self.persistent_db.clone(),
                    EvictionPolicy::NoEviction,
                ),
            });
        };

        let policy = group.config.eviction_policy();
        let kv_store = KvStore::new(
            IdempotencyConfig::NAMESPACE,
            self.configgroup_state.give_me_the_right_db(&group),
            policy,
        );

        Ok(IdempotencyStore { kv: kv_store })
    }
}

// Made public for the purpose of E2E testing in which a queue prefix is necessary to avoid tests
// consuming from each others' queues
pub async fn run_with_prefix(
    cfg: Configuration,
    listener: Option<TcpListener>,
    interserver_listener: Option<TcpListener>,
) {
    // OpenAPI/aide must be initialized before any routers are constructed
    // because its initialization sets generation-global settings which are
    // needed at router-construction time.
    let mut openapi = openapi::initialize_openapi();

    let persistent_db = DatabaseConfig::persistent(&cfg.persistent_db).expect("persistent db");
    let ephemeral_db = DatabaseConfig::ephemeral(&cfg.ephemeral_db).expect("ephemeral db");

    let configgroup_state = coyote_configgroup::State::init(BothDatabases {
        persistent_db: persistent_db.clone(),
        ephemeral_db: ephemeral_db.clone(),
    })
    .expect("initializing configgroup state");

    let stream_state =
        stream::State::init(persistent_db.clone()).expect("initializing stream state");

    let (raft, node_id) = core::cluster::initialize_raft(&cfg, persistent_db.clone())
        .await
        .expect("failed to initialize cluster");

    // build our application with a route
    let app_state = AppState {
        cfg: cfg.clone(),
        rate_limiter: crate::v1::modules::rate_limiter::RateLimiter::new(
            "rate_limiter_default",
            persistent_db.clone(),
        ),
        queue_store: crate::v1::modules::queue::QueueStore::new(),
        stream_state,
        raft,
        node_id,
        persistent_db,
        configgroup_state,
    };
    let v1_router = v1::router().with_state::<()>(app_state.clone());

    let interserver = tokio::spawn(run_interserver(
        cfg.clone(),
        app_state.clone(),
        interserver_listener,
    ));

    // Initialize all routes which need to be part of OpenAPI first.
    let app = ApiRouter::new()
        .nest_api_service("/api/v1", v1_router)
        .finish_api(&mut openapi);

    openapi::postprocess_spec(&mut openapi);
    let docs_router = docs::router(openapi);
    let app = app.merge(docs_router).layer((CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(AllowHeaders::mirror_request())
        .max_age(Duration::from_secs(600)),));
    let svc = tower::make::Shared::new(
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        NormalizePath::trim_trailing_slash(app),
    );

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
        // FIXME: gotta do actual error handling...
        let _ = tokio::join!(
            tokio::spawn(v1::modules::kv::worker(app_state.clone())),
            tokio::spawn(v1::modules::rate_limiter::worker(app_state.clone())),
            tokio::spawn(v1::modules::queue::worker(app_state.clone())),
        );
    });

    axum::serve(listener, svc)
        .with_graceful_shutdown(graceful_shutdown_handler())
        .await
        .unwrap();

    // Wait for workers to finish cleanup
    let _ = workers.await;
    let _ = interserver.await;
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
            format!("tower_http={level}"),
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
    if let Some(addr) = &cfg.opentelemetry_address {
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
                "coyote=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
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
