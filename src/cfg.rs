// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{fmt, marker::PhantomData, net::SocketAddr, path::PathBuf, sync::Arc};

use crate::error::Result;
use anyhow::Context;
use fjall::Database;
use serde::Deserialize;
use serde_with::with_prefix;
use tracing::Level;

use crate::core::security::JwtSigningConfig;

const DEFAULTS: &str = include_str!("../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

with_prefix!(ephemeral_db "ephemeral_db_");
with_prefix!(persistent_db "persistent_db_");
with_prefix!(management_db "management_db_");

pub trait StorageType {}

#[derive(Debug, Default)]
pub struct Ephemeral {}
impl StorageType for Ephemeral {}

#[derive(Debug, Default)]
pub struct Persistent {}
impl StorageType for Persistent {}

#[derive(Debug, Default)]
pub struct Management {}
impl StorageType for Management {}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DatabaseConfig<S: StorageType> {
    pub path: String,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(skip_serializing, default)]
    pub _phantom: PhantomData<S>,
}

impl<S: StorageType> DatabaseConfig<S> {
    fn database(dir: &str, file: &str) -> Result<Database> {
        let mut path = PathBuf::from(dir);
        path.push(file);
        fjall::Database::builder(path).open().map_err(|e| e.into())
    }
}

impl DatabaseConfig<Ephemeral> {
    pub fn persistent(db_config: Arc<DatabaseConfig<Persistent>>) -> Result<Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_persistent"),
        )
    }
}

impl DatabaseConfig<Persistent> {
    pub fn ephemeral(db_config: Arc<DatabaseConfig<Ephemeral>>) -> Result<Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_ephemeral"),
        )
    }
}

impl DatabaseConfig<Management> {
    pub fn management(db_config: Arc<DatabaseConfig<Management>>) -> Result<Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_management"),
        )
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: SocketAddr,

    /// The address to listen on for replication/etc
    pub interserver_listen_address: SocketAddr,

    #[serde(flatten, with = "management_db")]
    pub management_db_config: Arc<DatabaseConfig<Management>>,

    #[serde(flatten, with = "persistent_db")]
    pub persistent_db_config: Arc<DatabaseConfig<Persistent>>,

    #[serde(flatten, with = "ephemeral_db")]
    pub ephemeral_db_config: Arc<DatabaseConfig<Ephemeral>>,

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
    /// The service name to use for OpenTelemetry. If not provided, it defaults to "coyote".
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
        .add_source(config::Environment::with_prefix("COYOTE"))
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
        #[serde(flatten, with = "persistent_db")]
        pub persistent_db_config: Arc<DatabaseConfig<Persistent>>,

        #[serde(flatten, with = "ephemeral_db")]
        pub ephemeral_db_config: Arc<DatabaseConfig<Ephemeral>>,

        #[serde(flatten, with = "management_db")]
        pub management_db_config: Arc<DatabaseConfig<Management>>,
    }

    #[test]
    fn test_db() {
        let raw_config = r#"
ephemeral_db_path="/1"
ephemeral_db_filename="test1"
persistent_db_path="/2"
management_db_path="/3"
"#;

        let config = config::Config::builder()
            .add_source(config::File::from_str(raw_config, config::FileFormat::Toml))
            .build()
            .unwrap();

        let db_config = config.try_deserialize::<TestConfig>().unwrap();

        assert_eq!(db_config.ephemeral_db_config.path, "/1".to_string());
        assert_eq!(
            db_config.ephemeral_db_config.filename,
            Some("test1".to_string())
        );

        assert_eq!(db_config.persistent_db_config.path, "/2".to_string());
        assert!(db_config.persistent_db_config.filename.is_none());

        assert_eq!(db_config.management_db_config.path, "/3".to_string());
        assert!(db_config.management_db_config.filename.is_none());
    }
}
