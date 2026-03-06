// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    fmt,
    io::ErrorKind,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, anyhow};
use fs_err as fs;
use serde::{Deserialize, de::DeserializeOwned};
use tap::Pipe;
use tracing::Level;
use validator::Validate;

use crate::error::{Error, Result};

mod defaults;
mod validators;

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PeerAddr {
    SocketAddr(SocketAddr),
    HostnameAndPort { hostname: String, port: u16 },
}

impl PeerAddr {
    pub fn as_base_url(&self) -> url::Url {
        let base = self.to_string();
        format!("http://{base}")
            .parse()
            .expect("we validated this already")
    }
}

impl fmt::Display for PeerAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SocketAddr(s) => s.fmt(f),
            Self::HostnameAndPort { hostname, port } => write!(f, "{hostname}:{port}"),
        }
    }
}

impl From<SocketAddr> for PeerAddr {
    fn from(value: SocketAddr) -> Self {
        Self::SocketAddr(value)
    }
}

impl FromStr for PeerAddr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(p) = s.parse() {
            return Ok(Self::SocketAddr(p));
        }
        if let Some((host, port_str)) = s.rsplit_once(':') {
            if let Err(e) = addr::parse_domain_name(host) {
                anyhow::bail!("invalid hostname {host}: {e:?}");
            }
            let port = port_str.parse::<u16>().context("parsing port")?;
            Ok(Self::HostnameAndPort {
                hostname: host.to_string(),
                port,
            })
        } else {
            anyhow::bail!("Unable to parse PeerAddr {s:?}");
        }
    }
}

impl<'d> Deserialize<'d> for PeerAddr {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        let raw = String::deserialize(deserializer)?;
        raw.parse()
            .map_err(|e| serde::de::Error::custom(format!("invalid peer address: {e:?}")))
    }
}

impl serde::Serialize for PeerAddr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub filename: Option<String>,
}

/// Wrapper around a path that we know to be an extant dir
#[derive(Clone, Debug)]
pub struct Dir {
    inner: PathBuf,
}

impl Dir {
    /// Construct a new Dir from a path, returning an error if it is not valid.
    ///
    /// Note that this does filesystem metadata operations and can be blocking, particularly
    /// if your filesystem is slow (i.e., NFS).
    pub fn new<P: AsRef<Path>>(candidate: P) -> Result<Self> {
        let dir = candidate.as_ref();
        if !dir.exists()
            && let Err(e) = fs::create_dir_all(dir)
            && e.kind() != ErrorKind::AlreadyExists
        {
            return Err(Error::generic(e));
        }
        if !dir.is_dir() {
            return Err(Error::generic(format!(
                "database directory {} exists but is not a directory",
                dir.display()
            )));
        }
        Ok(Self {
            inner: dir.to_path_buf(),
        })
    }

    /// Adjoin a path to this one; wrapper around `Path::join`.
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.inner.join(path.as_ref())
    }
}

impl From<Dir> for PathBuf {
    fn from(value: Dir) -> Self {
        value.inner
    }
}

impl DatabaseConfig {
    fn default_cache_size() -> u64 {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let ret = sys.total_memory();
        if ret == 0 {
            // Default to fjall's current default (32 MiB)
            32 * 1024 * 1024
        } else {
            // Fjall recommends 20-25% of the memory for cache.
            // We can probably do more, but let's start with that.
            ret / 5
        }
    }

    fn database(dir: &Path, file: &str) -> Result<fjall::Database> {
        let dir = Dir::new(dir)?;
        let path = dir.join(file);
        // FIXME: we should probably make the cache size a config.
        fjall::Database::builder(path)
            .cache_size(Self::default_cache_size())
            .open()
            .map_err(|e| e.into())
    }

    pub fn persistent(db_config: &DatabaseConfig) -> Result<fjall::Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_persistent"),
        )
    }

    pub fn ephemeral(db_config: &DatabaseConfig) -> Result<fjall::Database> {
        Self::database(
            &db_config.path,
            db_config.filename.as_deref().unwrap_or("fjall_ephemeral"),
        )
    }
}

#[derive(Clone, Debug, Deserialize, Validate)]
#[validate(schema(
    function = "validators::validate_cluster_configuration",
    skip_on_field_errors = true
))]
pub struct ClusterConfiguration {
    /// The address to listen on for replication. Defaults to the main listen address,
    /// but with the port incremented by 10000
    #[serde(default)]
    pub listen_address: Option<SocketAddr>,

    #[serde(default = "defaults::cluster_name")]
    pub name: String,

    /// Location to store snapshots.
    ///
    /// This volume must have at least as much space as the persistent DB path
    /// and ephemeral DB path combined. Defaults to a subdirectory under the
    /// persistent DB path if not passed.
    #[serde(default)]
    pub snapshot_path: Option<PathBuf>,

    /// Location to store logs. For high-throughput systems, this should be a separate volume.
    ///
    /// Defaults to a subdirectory under the persistent DB path if not passed.
    #[serde(default)]
    pub log_path: Option<PathBuf>,

    #[serde(default)]
    pub secret: Option<String>,

    /// Timeout for replication requests.
    ///
    /// This should be set to approximately 2X the RTT of your farthest-apart nodes.
    #[serde(
        rename = "replication_request_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_replication_request_timeout"
    )]
    pub replication_request_timeout: Duration,

    /// Timeout for discovery requests.
    ///
    /// This should be set to approximately 2X the RTT of your farthest-apart nodes.
    #[serde(
        rename = "discovery_request_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_discovery_request_timeout"
    )]
    pub discovery_request_timeout: Duration,

    /// Timeout for new connections.
    ///
    /// If you want to be tolerant of dropped packets, this should be set to at least TO + ε,
    /// where TO is the initial TCP retransmission timer (typically either 1s or 3s,
    /// depending on your operating system).
    #[serde(
        rename = "connection_timeout_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_connection_timeout"
    )]
    pub connection_timeout: Duration,

    /// How often to send heartbeats.
    ///
    /// This controls how fast lost leaders can be detected.
    /// Must not be less than `replication_request_timeout`.
    #[serde(
        rename = "heartbeat_interval_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_heartbeat_interval"
    )]
    pub heartbeat_interval: Duration,

    /// The minimum time to let an election run for.
    ///
    /// This should be set to at least 4x the RTT of your farthest-apart nodes,
    /// and must not be less than `heartbeat_interval_ms`.
    #[serde(
        rename = "election_timeout_min_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_election_timeout_min"
    )]
    pub election_timeout_min: Duration,

    /// The minimum time to let an election run for.
    ///
    /// This should be set to at least 5x the RTT of your farthest-apart nodes
    /// and must not be less than `cluster_election_timeout_max`.
    #[serde(
        rename = "election_timeout_max_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::cluster_election_timeout_max"
    )]
    pub election_timeout_max: Duration,

    /// Address that other nodes should use to communicate with this one.
    ///
    /// If not passed, we'll attempt to discover it at boot time.
    /// This cannot currently be changed after cluster initialization.
    #[serde(default)]
    pub advertised_address: Option<PeerAddr>,

    /// Other nodes that we should attempt to join a cluster with at boot time.
    #[serde(default)]
    pub seed_nodes: Vec<PeerAddr>,

    /// Automatically initialize the cluster on bootup if we can't discover any
    /// peers and we don't have any existing state.
    ///
    /// If you initialize all peers at exactly the same time, this can potentially cause errors.
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

    #[serde(
        rename = "log_index_interval_ms",
        with = "crate::serde::duration::millis",
        default = "defaults::log_index_interval"
    )]
    pub log_index_interval: Duration,

    /// Interval (in transactions) between fsyncing the commit log.
    ///
    /// This should be set to 1 for full ACID compliance; at any other value,
    /// data may be lost in the event of catastrophic hardware failure.
    /// If you are using a multi-node cluster and set this to a value other than
    /// 1, if a node experiences catastrophic failure and the OS shuts down uncleanly, that node
    /// should be removed from the cluster and rebuilt. If set to 0, logs will only be fsynced on a
    /// timer
    #[validate(range(min = 0, max = 1024))]
    #[serde(default = "defaults::cluster_log_sync_interval_commits")]
    pub log_sync_interval_commits: usize,

    /// Interval (in milliseconds) between fsyncing the commit log.
    #[validate(custom(function = "validators::validate_log_sync_interval_duration"))]
    #[serde(
        default = "defaults::cluster_log_sync_interval_duration",
        rename = "log_sync_interval_ms",
        with = "crate::serde::duration::millis"
    )]
    pub log_sync_interval_duration: Duration,

    /// Commit logs to the cluster immediately, before fsyncing them to persistent storage.
    ///
    /// This should be set to `false` for full ACID compliance, but can be set to `true` to enable
    /// higher throughput than your fsync rate. Note that we always flush to the OS buffers before
    /// acking, so data will only be lost of the OS crashes.
    #[serde(default = "defaults::default_true")]
    pub log_ack_immediately: bool,

    /// Trigger a background snapshot after this many writes
    pub snapshot_after_writes: Option<u32>,

    /// Trigger a background snapshot after this many milliseconds
    #[serde(
        rename = "snapshot_after_ms",
        with = "crate::serde::duration::opt_millis",
        default = "defaults::cluster_snapshot_after_time"
    )]
    pub snapshot_after_time: Option<Duration>,
}

impl ClusterConfiguration {
    pub fn log_path(&self, root: &ConfigurationInner) -> Result<Dir> {
        if let Some(path) = &self.log_path {
            Dir::new(path)
        } else {
            Dir::new(root.persistent_db.path.join("cluster_logs"))
        }
    }

    pub fn snapshot_path(&self, root: &ConfigurationInner) -> Result<Dir> {
        if let Some(path) = &self.snapshot_path {
            Dir::new(path)
        } else {
            Dir::new(root.persistent_db.path.join("cluster_snapshots"))
        }
    }

    pub fn listen_address(&self, root: &ConfigurationInner) -> SocketAddr {
        self.listen_address.unwrap_or_else(|| {
            let port = root.listen_address.port() + 10000;
            let ip = root.listen_address.ip();
            SocketAddr::new(ip, port)
        })
    }
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

#[derive(Clone, Debug, Deserialize, Validate)]
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

    /// The OpenTelemetry address to send metrics to if given.
    ///
    /// If not specified, the server will attempt to fall back
    /// to `opentelemetry_address`
    pub opentelemetry_metrics_address: Option<String>,

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

    #[validate(nested)]
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

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    macro_rules! opt_env_overrides {
        ( $( $field:ident: $env_var:literal ),* $(,)? ) => {
            $(
                if let Some(value) = env_var($env_var)? {
                    *$field = Some(value);
                }
            )*
        };
    }

    macro_rules! env_ms_overrides {
        ( $( $field:ident: $env_var:literal ),* $(,)? ) => {
            $(
                if let Some(value) = env_var_ms($env_var)? {
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
        opentelemetry_metrics_address,
        opentelemetry_metrics_use_http,
        opentelemetry_metrics_period_seconds,
        opentelemetry_sample_ratio,
        opentelemetry_service_name,
        environment,
        bootstrap_cfg_path,
        cluster:
            ClusterConfiguration {
                advertised_address: cluster_advertised_address,
                listen_address: cluster_listen_address,
                name: cluster_name,
                snapshot_path: cluster_snapshot_path,
                log_path: cluster_log_path,
                secret: cluster_secret,
                replication_request_timeout: cluster_replication_request_timeout,
                discovery_request_timeout: cluster_discovery_request_timeout,
                connection_timeout: cluster_connection_timeout,
                heartbeat_interval: cluster_heartbeat_interval,
                election_timeout_min: cluster_election_timeout_min,
                election_timeout_max: cluster_election_timeout_max,
                seed_nodes: cluster_seed_nodes,
                auto_initialize: cluster_auto_initialize,
                discovery_timeout: cluster_discovery_timeout,
                startup_discovery_delay: cluster_startup_discovery_delay,
                log_index_interval: cluster_log_index_interval,
                log_sync_interval_commits: cluster_log_sync_interval_commits,
                log_sync_interval_duration: cluster_log_sync_interval_duration,
                log_ack_immediately: cluster_log_ack_immediately,
                snapshot_after_writes: cluster_snapshot_after_writes,
                snapshot_after_time: cluster_snapshot_after_time,
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
        cluster_name: "DIOM_CLUSTER_NAME",
        cluster_auto_initialize: "DIOM_CLUSTER_AUTO_INITIALIZE",
        cluster_log_sync_interval_commits: "DIOM_CLUSTER_LOG_SYNC_INTERVAL_COMMITS",
        cluster_log_ack_immediately: "DIOM_CLUSTER_LOG_ACK_IMMEDIATELY"
    );

    opt_env_overrides!(
        persistent_db_filename: "DIOM_PERSISTENT_DB_FILENAME",
        ephemeral_db_filename: "DIOM_EPHEMERAL_DB_FILENAME",
        opentelemetry_address: "DIOM_OPENTELEMETRY_ADDRESS",
        opentelemetry_metrics_address: "DIOM_OPENTELEMETRY_METRICS_ADDRESS",
        opentelemetry_sample_ratio: "DIOM_OPENTELEMETRY_SAMPLE_RATIO",
        bootstrap_cfg_path: "DIOM_BOOTSTRAP_CFG_PATH",
        cluster_listen_address: "DIOM_CLUSTER_LISTEN_ADDRESS",
        cluster_advertised_address: "DIOM_CLUSTER_ADVERTISED_ADDRESS",
        cluster_snapshot_path: "DIOM_CLUSTER_SNAPSHOT_PATH",
        cluster_log_path: "DIOM_CLUSTER_LOG_PATH",
        cluster_secret: "DIOM_CLUSTER_SECRET",
        cluster_snapshot_after_writes: "DIOM_SNAPSHOT_AFTER_WRITES"
    );

    env_ms_overrides!(
        cluster_heartbeat_interval: "DIOM_CLUSTER_HEARTBEAT_INTERVAL_MS",
        cluster_election_timeout_min: "DIOM_CLUSTER_ELECTION_TIMEOUT_MIN_MS",
        cluster_election_timeout_max: "DIOM_CLUSTER_ELECTION_TIMEOUT_MAX_MS",
        cluster_replication_request_timeout: "DIOM_CLUSTER_REPLICATION_REQUEST_TIMEOUT_MS",
        cluster_discovery_request_timeout: "DIOM_CLUSTER_DISCOVERY_REQUEST_TIMEOUT_MS",
        cluster_connection_timeout: "DIOM_CLUSTER_CONNECTION_TIMEOUT_MS",
        cluster_discovery_timeout: "DIOM_CLUSTER_DISCOVERY_TIMEOUT_MS",
        cluster_startup_discovery_delay: "DIOM_CLUSTER_STARTUP_DISCOVERY_DELAY_MS",
        cluster_log_index_interval: "DIOM_LOG_INDEX_INTERVAL_MS",
        cluster_log_sync_interval_duration: "DIOM_CLUSTER_LOG_SYNC_INTERVAL_MS"
    );

    // Fields that require different parsing
    if let Some(value) = env_var_comma_separated("DIOM_CLUSTER_SEED_NODES")? {
        *cluster_seed_nodes = value;
    }
    if let Some(value) = env_var_ms("DIOM_CLUSTER_SNAPSHOT_AFTER_MS")? {
        *cluster_snapshot_after_time = Some(value);
    }

    config.validate()?;

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
        value
            .split(',')
            .map(str::trim_ascii)
            .map(FromStr::from_str)
            .collect()
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
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::{PeerAddr, load_toml};

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

    #[test]
    fn test_peer_addr_parsing() {
        let Ok(PeerAddr::SocketAddr(SocketAddr::V4(sa))) = "127.0.0.2:8050".parse() else {
            panic!("failed to parse v4 addr")
        };
        assert_eq!(sa.port(), 8050);
        assert_eq!(*sa.ip(), Ipv4Addr::new(127, 0, 0, 2));
        let Ok(PeerAddr::SocketAddr(SocketAddr::V6(sa))) = "[::1001:2%1234]:8050".parse() else {
            panic!("failed to parse v6 addr");
        };
        assert_eq!(sa.scope_id(), 1234);
        assert_eq!(sa.port(), 8050);
        assert_eq!(*sa.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0x1001, 0x2));
        assert_eq!(
            "foobar:8050".parse::<PeerAddr>().expect("should parse"),
            PeerAddr::HostnameAndPort {
                hostname: "foobar".to_owned(),
                port: 8050
            }
        );
        "  illegal-hostname:1"
            .parse::<PeerAddr>()
            .expect_err("should fail to parse");
        "no-port-provided"
            .parse::<PeerAddr>()
            .expect_err("should fail to parse");
        "port-out-of-bounds:65537"
            .parse::<PeerAddr>()
            .expect_err("should fail to parse");
    }

    #[test]
    fn test_peer_addr_serde() {
        let addrs = [
            PeerAddr::SocketAddr(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)),
                8050,
            )),
            PeerAddr::SocketAddr(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 2)),
                8051,
            )),
            PeerAddr::HostnameAndPort {
                hostname: "foo".to_owned(),
                port: 8052,
            },
        ];
        for addr in addrs {
            let serialized = serde_json::to_string(&addr).expect("should serialize");
            assert_ne!(&serialized, "");
            let deserialized: PeerAddr =
                serde_json::from_str(&serialized).expect("should deserialize");
            assert_eq!(deserialized, addr);
        }
    }
}
