// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{fmt, net::SocketAddr, sync::Arc};

use anyhow::Context;
use serde::Deserialize;
use tracing::Level;

use crate::core::security::JwtSigningConfig;

const DEFAULTS: &str = include_str!("../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: SocketAddr,

    /// Contains the secret and algorithm for signing JWTs
    #[serde(flatten)]
    pub jwt_signing_config: Arc<JwtSigningConfig>,

    /// The log level to run the service with. Supported: info, debug, trace
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    pub log_format: LogFormat,
    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,
    /// By default, `opentelemetry_address` is expected to be a GRPC server.
    ///
    /// When this is set to true, HTTP is used instead.
    #[serde(default)]
    pub opentelemetry_metrics_use_http: bool,
    #[serde(default = "default_opentelemetry_metrics_period")]
    pub opentelemetry_metrics_period_seconds: u64,
    /// The ratio at which to sample spans when sending to OpenTelemetry.
    ///
    /// When not given it defaults to always sending.
    /// If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,
    /// The service name to use for OpenTelemetry. If not provided, it defaults to "diom".
    pub opentelemetry_service_name: String,
    /// The environment (dev, staging, or prod) that the server is running in.
    pub environment: Environment,

    #[serde(flatten)]
    pub internal: InternalConfig,
}

const fn default_opentelemetry_metrics_period() -> u64 {
    60
}

#[derive(Clone, Debug, Deserialize)]
pub struct InternalConfig {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Default,
    Json,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Dev => "dev",
                Environment::Staging => "staging",
                Environment::Prod => "prod",
            }
        )
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
            Self::Trace => Level::TRACE,
        }
        .fmt(f)
    }
}

pub fn load() -> anyhow::Result<Arc<ConfigurationInner>> {
    let config = config::Config::builder()
        .add_source(config::File::from_str(DEFAULTS, config::FileFormat::Toml))
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("DIOM"))
        .build()?;

    let config = config
        .try_deserialize::<ConfigurationInner>()
        .context("failed to extract configuration")?;

    Ok(Arc::from(config))
}
