use std::time::Duration;

use bytes::Bytes;
use diom_backend::cfg::{ConfigurationInner, OpenTelemetryProtocol, build_fmt_layer};
use diom_core::INSTANCE_ID;
use opentelemetry::{InstrumentationScope, trace::TracerProvider as _};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
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
        let cluster_level = cfg.cluster.log_level.unwrap_or(cfg.log_level).to_string();
        let var = [
            format!("diom_backend::core::cluster={cluster_level}"),
            format!("diom={level}"),
            format!("fjall_utils={level}"),
            format!("tower_http={level}"),
            "fjall=warn".to_string(),
            "openraft=warn".to_string(),
            "opentelemetry_sdk=error".to_string(),
        ];

        var.join(",")
    });

    let mapped = cfg.opentelemetry.address.as_ref().map(|addr| {
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
                cfg.opentelemetry
                    .sample_ratio
                    .map(Sampler::TraceIdRatioBased)
                    .unwrap_or(Sampler::AlwaysOn),
            )
            .with_span_processor(batch_span_processor)
            .with_resource(
                opentelemetry_sdk::Resource::builder()
                    .with_service_name(cfg.opentelemetry.service_name.clone())
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
        (layer.boxed(), provider)
    });

    let (otel_layer, otel_tracer_provider) = mapped.unzip();

    // Then create a subscriber with an additional layer printing to stdout.
    // This additional layer is either formatted normally or in JSON format.
    let stdout_layer = if for_test {
        tracing_subscriber::fmt::layer().with_test_writer().boxed()
    } else {
        build_fmt_layer(cfg.log_format)
    };

    let debugging_layer = vec![Some(stdout_layer), otel_layer]
        .with_filter(tracing_subscriber::EnvFilter::new(&filter_directives));

    let dispatch = tracing_subscriber::Registry::default().with(debugging_layer);

    #[cfg(feature = "tokio-console")]
    let dispatch = {
        let layer = console_subscriber::spawn();
        let mut filter_directives = filter_directives;
        filter_directives.push_str(",tokio=trace,runtime=trace");
        dispatch.with(layer.with_filter(tracing_subscriber::EnvFilter::new(filter_directives)))
    };

    (dispatch.into(), otel_tracer_provider)
}

pub(crate) fn setup_metrics(cfg: &ConfigurationInner) {
    if let Some(addr) = cfg
        .opentelemetry
        .metrics_address
        .as_ref()
        .or(cfg.opentelemetry.address.as_ref())
    {
        let exporter = if matches!(
            cfg.opentelemetry.metrics_protocol,
            OpenTelemetryProtocol::Http
        ) {
            tracing::debug!("sending http otel metrics to {addr}");

            opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_endpoint(addr)
                .with_http_client(OtelReqwestClient::from(reqwest::Client::new()))
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
            .with_interval(cfg.opentelemetry.metrics_period.into())
            .build();

        let provider = SdkMeterProvider::builder()
            .with_reader(reader)
            .with_resource(
                opentelemetry_sdk::Resource::builder()
                    .with_service_name(cfg.opentelemetry.service_name.clone())
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

#[derive(Debug, Default)]
struct OtelReqwestClient(reqwest::Client);

impl From<reqwest::Client> for OtelReqwestClient {
    fn from(value: reqwest::Client) -> Self {
        Self(value)
    }
}

// From https://docs.rs/opentelemetry-http/0.31.0/src/opentelemetry_http/lib.rs.html#94-108
// Using our own custom type because upstream hasn't released support for reqwest 0.13 yet.
#[async_trait::async_trait]
impl opentelemetry_http::HttpClient for OtelReqwestClient {
    async fn send_bytes(
        &self,
        request: http::Request<Bytes>,
    ) -> Result<http::Response<Bytes>, opentelemetry_http::HttpError> {
        let request = request.try_into()?;
        let mut response = self.0.execute(request).await?.error_for_status()?;
        let headers = std::mem::take(response.headers_mut());
        let mut http_response = http::Response::builder()
            .status(response.status())
            .body(response.bytes().await?)?;
        *http_response.headers_mut() = headers;

        Ok(http_response)
    }
}
