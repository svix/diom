use std::time::Duration;

use diom::cfg::{self, ConfigurationInner};
use diom_core::INSTANCE_ID;
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
use tracing_subscriber::{Layer as _, layer::SubscriberExt as _};

pub(crate) fn setup_tracing(
    cfg: &ConfigurationInner,
    for_test: bool,
) -> (tracing::Dispatch, Option<SdkTracerProvider>) {
    let filter_directives = std::env::var("RUST_LOG").unwrap_or_else(|e| {
        if let std::env::VarError::NotUnicode(_) = e {
            eprintln!("RUST_LOG environment variable has non-utf8 contents, ignoring!");
        }

        let level = cfg.log_level.to_string();
        let var = [
            format!("diom={level}"),
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

pub(crate) fn setup_metrics(cfg: &ConfigurationInner) {
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
