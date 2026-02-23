// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    fmt, fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, anyhow};
use fjall::Database;
use serde::{Deserialize, de::DeserializeOwned};
use tap::Pipe;
use tracing::Level;

use crate::error::Result;

mod defaults;

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
    #[serde(default = "defaults::cluster_listen_address")]
    pub listen_address: SocketAddr,

    #[serde(default = "defaults::cluster_name")]
    pub name: String,

    #[serde(default = "defaults::cluster_snapshot_path")]
    pub snapshot_path: PathBuf,

    #[serde(default = "defaults::cluster_log_path")]
    pub log_path: PathBuf,

    #[serde(default)]
    pub secret: Option<String>,

    #[serde(
        rename = "replication_request_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_replication_request_timeout"
    )]
    pub replication_request_timeout: Duration,

    #[serde(
        rename = "discovery_request_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_discovery_request_timeout"
    )]
    pub discovery_request_timeout: Duration,

    #[serde(
        rename = "connection_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_connection_timeout"
    )]
    pub connection_timeout: Duration,

    #[serde(default = "defaults::cluster_heartbeat_interval_ms")]
    pub heartbeat_interval_ms: u64,

    #[serde(default = "defaults::cluster_election_timeout_min_ms")]
    pub election_timeout_min_ms: u64,

    #[serde(default = "defaults::cluster_election_timeout_max_ms")]
    pub election_timeout_max_ms: u64,

    #[serde(default)]
    pub seed_nodes: Vec<SocketAddr>,

    /// Automatically initialize the cluster on bootup if we can't discover any
    /// peers and we don't have any existing state. If you initialize all peers
    /// at exactly the same time, this can potentially cause errors.
    #[serde(default = "defaults::cluster_auto_initialize")]
    pub auto_initialize: bool,

    #[serde(
        rename = "discovery_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_discovery_timeout"
    )]
    pub discovery_timeout: Duration,

    #[serde(
        rename = "startup_discovery_delay_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_startup_discovery_delay"
    )]
    pub startup_discovery_delay: Duration,
}

impl Default for ClusterConfiguration {
    fn default() -> Self {
        default_from_serde().unwrap()
    }
}

fn default_from_serde<T: DeserializeOwned>() -> Result<T, serde::de::value::Error> {
    let empty: [(String, String); 0] = [];
    T::deserialize(serde::de::value::MapDeserializer::new(empty.into_iter()))
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationInner {
    /// The address to listen on
    #[serde(default = "defaults::listen_address")]
    pub listen_address: SocketAddr,

    #[serde(default = "defaults::persistent_db")]
    pub persistent_db: DatabaseConfig,
    #[serde(default = "defaults::ephemeral_db")]
    pub ephemeral_db: DatabaseConfig,

    /// The log level to run the service with. Supported: info, debug, trace
    #[serde(default)]
    pub log_level: LogLevel,

    /// The log format that all output will follow. Supported: default, json
    #[serde(default)]
    pub log_format: LogFormat,

    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,

    /// By default, `opentelemetry_address` is expected to be a GRPC server.
    ///
    /// When this is set to true, HTTP is used instead.
    #[serde(default)]
    pub opentelemetry_metrics_use_http: bool,

    #[serde(default = "defaults::opentelemetry_metrics_period")]
    pub opentelemetry_metrics_period_seconds: u64,

    /// The ratio at which to sample spans when sending to OpenTelemetry.
    ///
    /// When not given it defaults to always sending.
    /// If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,

    /// The service name to use for OpenTelemetry. If not provided, it defaults to "diom".
    #[serde(default = "defaults::opentelemetry_service_name")]
    pub opentelemetry_service_name: String,

    /// The environment (dev, staging, or prod) that the server is running in.
    #[serde(default)]
    pub environment: Environment,

    #[serde(default)]
    pub cluster: ClusterConfiguration,

    #[serde(default)]
    pub bootstrap_cfg_path: Option<String>,
}

impl Default for ConfigurationInner {
    fn default() -> Self {
        default_from_serde().unwrap()
    }
}

macro_rules! from_str_via_serde {
    ($ty:ty) => {
        impl FromStr for $ty {
            type Err = serde::de::value::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <Self as serde::Deserialize>::deserialize(serde::de::value::StrDeserializer::new(s))
            }
        }
    };
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

from_str_via_serde!(LogLevel);

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Default,
    Json,
}

from_str_via_serde!(LogFormat);

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    #[default]
    Dev,
    Staging,
    Prod,
}

from_str_via_serde!(Environment);

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
    let config_toml = match config_path {
        Some(path) => Some(fs::read_to_string(path).context("reading config file")?),
        None => None,
    };
    load_toml(config_toml.as_deref())
}

fn load_toml(config_toml: Option<&str>) -> anyhow::Result<Arc<ConfigurationInner>> {
    let mut config = match config_toml {
        Some(config_toml) => toml::from_str(config_toml).context("parsing config file")?,
        None => ConfigurationInner::default(),
    };

    macro_rules! env_overrides {
        ( $( $field:ident: $env_var:literal ),* $(,)? ) => {
            $(
                if let Some(value) = env_var($env_var)? {
                    *$field = value;
                }
            )*
        };
    }

    let ConfigurationInner {
        listen_address,
        persistent_db:
            DatabaseConfig {
                path: persistent_db_path,
                filename: persistent_db_filename,
            },
        ephemeral_db:
            DatabaseConfig {
                path: ephemeral_db_path,
                filename: ephemeral_db_filename,
            },
        log_level,
        log_format,
        opentelemetry_address,
        opentelemetry_metrics_use_http,
        opentelemetry_metrics_period_seconds,
        opentelemetry_sample_ratio,
        opentelemetry_service_name,
        environment,
        bootstrap_cfg_path,
        cluster:
            ClusterConfiguration {
                listen_address: cluster_listen_address,
                name: cluster_name,
                snapshot_path: cluster_snapshot_path,
                log_path: cluster_log_path,
                secret: cluster_secret,
                replication_request_timeout: cluster_replication_request_timeout,
                discovery_request_timeout: cluster_discovery_request_timeout,
                connection_timeout: cluster_connection_timeout,
                heartbeat_interval_ms: cluster_heartbeat_interval_ms,
                election_timeout_min_ms: cluster_election_timeout_min_ms,
                election_timeout_max_ms: cluster_election_timeout_max_ms,
                seed_nodes: cluster_seed_nodes,
                auto_initialize: cluster_auto_initialize,
                discovery_timeout: cluster_discovery_timeout,
                startup_discovery_delay: cluster_startup_discovery_delay,
            },
    } = &mut config;

    env_overrides!(
        listen_address: "DIOM_LISTEN_ADDRESS",
        persistent_db_path: "DIOM_PERSISTENT_DB_PATH",
        ephemeral_db_path: "DIOM_EPHEMERAL_DB_PATH",
        log_level: "DIOM_LOG_LEVEL",
        log_format: "DIOM_LOG_FORMAT",
        opentelemetry_metrics_use_http: "DIOM_OPENTELEMETRY_METRICS_USE_HTTP",
        opentelemetry_metrics_period_seconds: "DIOM_OPENTELEMETRY_METRICS_PERIOD_SECONDS",
        opentelemetry_service_name: "DIOM_OPENTELEMETRY_SERVICE_NAME",
        environment: "DIOM_ENVIRONMENT",
        cluster_listen_address: "DIOM_CLUSTER_LISTEN_ADDRESS",
        cluster_name: "DIOM_CLUSTER_NAME",
        cluster_snapshot_path: "DIOM_CLUSTER_SNAPSHOT_PATH",
        cluster_log_path: "DIOM_CLUSTER_LOG_PATH",
        cluster_heartbeat_interval_ms: "DIOM_CLUSTER_HEARTBEAT_INTERVAL_MS",
        cluster_election_timeout_min_ms: "DIOM_CLUSTER_ELECTION_TIMEOUT_MIN_MS",
        cluster_election_timeout_max_ms: "DIOM_CLUSTER_ELECTION_TIMEOUT_MAX_MS",
        cluster_auto_initialize: "DIOM_CLUSTER_AUTO_INITIALIZE",
    );

    // Option fields not supported by the simple macro above.
    if let Some(value) = env_var("DIOM_PERSISTENT_DB_FILENAME")? {
        *persistent_db_filename = Some(value);
    }
    if let Some(value) = env_var("DIOM_EPHEMERAL_DB_FILENAME")? {
        *ephemeral_db_filename = Some(value);
    }
    if let Some(value) = env_var("DIOM_OPENTELEMETRY_ADDRESS")? {
        *opentelemetry_address = Some(value);
    }
    if let Some(value) = env_var("DIOM_OPENTELEMETRY_SAMPLE_RATIO")? {
        *opentelemetry_sample_ratio = Some(value);
    }
    if let Some(value) = env_var("DIOM_CLUSTER_SECRET")? {
        *cluster_secret = Some(value);
    }
    if let Some(value) = env_var("DIOM_BOOTSTRAP_CFG_PATH")? {
        *bootstrap_cfg_path = Some(value);
    }

    // Fields that require different parsing
    if let Some(value) = env_var_comma_separated("DIOM_CLUSTER_SEED_NODES")? {
        *cluster_seed_nodes = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_REPLICATION_REQUEST_TIMEOUT_MS")? {
        *cluster_replication_request_timeout = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_DISCOVERY_REQUEST_TIMEOUT_MS")? {
        *cluster_discovery_request_timeout = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_CONNECTION_TIMEOUT_MS")? {
        *cluster_connection_timeout = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_DISCOVERY_TIMEOUT_MS")? {
        *cluster_discovery_timeout = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_STARTUP_DISCOVERY_DELAY_MS")? {
        *cluster_startup_discovery_delay = value;
    };

    Ok(Arc::from(config))
}

fn env_var<T>(name: &'static str) -> anyhow::Result<Option<T>>
where
    T: FromStr<Err: fmt::Display>,
{
    env_var_parse(name, FromStr::from_str)
}

fn env_var_ms(name: &'static str) -> anyhow::Result<Option<Duration>> {
    env_var::<u64>(name)?.map(Duration::from_millis).pipe(Ok)
}

fn env_var_comma_separated<T>(name: &'static str) -> anyhow::Result<Option<Vec<T>>>
where
    T: FromStr<Err: fmt::Display>,
{
    env_var_parse(name, |value| {
        value.split(',').map(FromStr::from_str).collect()
    })
}

fn env_var_parse<T, E>(
    name: &'static str,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> anyhow::Result<Option<T>>
where
    E: fmt::Display,
{
    match std::env::var(name) {
        Ok(value) => parse(&value)
            .map_err(|e| anyhow!("invalid format for `{name}`: {e}"))
            .map(Some),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err(anyhow!("invalid format for `{name}`: invalid UTF-8"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::load_toml;

    #[test]
    fn test_db() {
        let raw_config = r#"
ephemeral_db = { path = "/1", filename = "test1" }
persistent_db.path = "/2"
"#;

        let config = load_toml(Some(raw_config)).unwrap();

        assert_eq!(config.ephemeral_db.path, "/1".to_string());
        assert_eq!(config.ephemeral_db.filename, Some("test1".to_string()));

        assert_eq!(config.persistent_db.path, "/2".to_string());
        assert!(config.persistent_db.filename.is_none());
    }

    #[test]
    // run this test as:
    // DIOM_LISTEN_ADDRESS='0.0.0.0:1234' cargo test -p diom config_env_override -- --ignored
    #[ignore]
    fn test_config_env_override() {
        let config = load_toml(None).unwrap();
        assert_eq!(
            config.listen_address,
            "0.0.0.0:1234".parse::<SocketAddr>().unwrap()
        );
    }
}
