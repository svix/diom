// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    fmt,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use config::ConfigBuilder;
use fjall::Database;
use serde::Deserialize;
use tap::Pipe;
use tracing::Level;

use crate::error::Result;

const DEFAULTS: &str = include_str!("../../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub filename: Option<String>,
}

impl DatabaseConfig {
    fn database(dir: &Path, file: &str) -> Result<Database> {
        let mut path = PathBuf::from(dir);
        path.push(file);
        fjall::Database::builder(path).open().map_err(|e| e.into())
    }

    pub fn persistent(db_config: &DatabaseConfig) -> Result<Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_persistent"),
        )
    }

    pub fn ephemeral(db_config: &DatabaseConfig) -> Result<Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_ephemeral"),
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClusterConfiguration {
    /// The address to listen on for replication
    pub listen_address: SocketAddr,

    pub name: String,

    pub snapshot_path: PathBuf,

    pub log_path: PathBuf,

    #[serde(default)]
    pub secret: Option<String>,

    #[serde(
        rename = "replication_request_timeout_ms",
        with = "crate::serde::duration::millis"
    )]
    pub replication_request_timeout: Duration,

    #[serde(
        rename = "discovery_request_timeout_ms",
        with = "crate::serde::duration::millis"
    )]
    pub discovery_request_timeout: Duration,

    #[serde(
        rename = "connection_timeout_ms",
        with = "crate::serde::duration::millis"
    )]
    pub connection_timeout: Duration,

    pub heartbeat_interval_ms: u64,

    pub election_timeout_min_ms: u64,

    pub election_timeout_max_ms: u64,

    #[serde(default)]
    pub seed_nodes: Vec<SocketAddr>,

    /// Automatically initialize the cluster on bootup if we can't discover any
    /// peers and we don't have any existing state. If you initialize all peers
    /// at exactly the same time, this can potentially cause errors.
    pub auto_initialize: bool,

    #[serde(
        rename = "discovery_timeout_ms",
        with = "crate::serde::duration::millis"
    )]
    pub discovery_timeout: Duration,

    #[serde(
        rename = "startup_discovery_delay_ms",
        with = "crate::serde::duration::millis"
    )]
    pub startup_discovery_delay: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: SocketAddr,

    pub persistent_db: DatabaseConfig,
    pub ephemeral_db: DatabaseConfig,

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
    /// The service name to use for OpenTelemetry. If not provided, it defaults to "coyote".
    pub opentelemetry_service_name: String,
    /// The environment (dev, staging, or prod) that the server is running in.
    pub environment: Environment,

    pub cluster: ClusterConfiguration,

    #[serde(flatten)]
    pub internal: InternalConfig,
}

#[cfg(test)]
/// make a ConfigurationInner for testing use
impl Default for ConfigurationInner {
    fn default() -> Self {
        use rand::distr::{Alphanumeric, SampleString};

        let jwt_key = Alphanumeric.sample_string(&mut rand::rng(), 32);

        let config = config::Config::builder()
            .add_source(config::File::from_str(DEFAULTS, config::FileFormat::Toml))
            .set_default("jwt_algorithm", "HS256")
            .unwrap()
            .set_default("jwt_secret", jwt_key)
            .unwrap()
            .build()
            .unwrap();

        config.try_deserialize::<ConfigurationInner>().unwrap()
    }
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

pub fn load(config_path: Option<&str>) -> anyhow::Result<Arc<ConfigurationInner>> {
    let config = config::Config::builder()
        .add_source(config::File::from_str(DEFAULTS, config::FileFormat::Toml))
        .pipe(|config: ConfigBuilder<_>| {
            if let Some(path) = config_path {
                config.add_source(config::File::with_name(path))
            } else {
                config
            }
        })
        .add_source(
            config::Environment::with_prefix("COYOTE")
                .list_separator(",")
                .separator(":")
                .try_parsing(true)
                .with_list_parse_key("cluster.seed_nodes"),
        )
        .build()?;

    let config = config
        .try_deserialize::<ConfigurationInner>()
        .context("failed to extract configuration")?;

    Ok(Arc::from(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct TestConfig {
        pub persistent_db: DatabaseConfig,
        pub ephemeral_db: DatabaseConfig,
    }

    #[test]
    fn test_db() {
        let raw_config = r#"
ephemeral_db = { path = "/1", filename = "test1" }
persistent_db.path = "/2"
"#;

        let config = config::Config::builder()
            .add_source(config::File::from_str(raw_config, config::FileFormat::Toml))
            .build()
            .unwrap();

        let db_config = config.try_deserialize::<TestConfig>().unwrap();

        assert_eq!(db_config.ephemeral_db.path, "/1".to_string());
        assert_eq!(db_config.ephemeral_db.filename, Some("test1".to_string()));

        assert_eq!(db_config.persistent_db.path, "/2".to_string());
        assert!(db_config.persistent_db.filename.is_none());
    }
}
