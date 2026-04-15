use std::{
    fmt,
    io::ErrorKind,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context, bail};
use diom_core::{Monotime, types::DurationMs};
use diom_derive::{DumpableConfig, EnvOverridable};
use fs_err as fs;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tracing::Level;
use validator::Validate;

use crate::error::{Error, Result};

mod defaults;
mod dumpable_config;
pub(crate) mod env_overridable;
mod memory_size;
mod validators;

use self::{
    dumpable_config::DumpableConfig,
    env_overridable::{EnvOverridable, env_var},
};
pub use self::{env_overridable::Variable, memory_size::MemorySize};

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

impl Serialize for PeerAddr {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Clone, Debug, Deserialize, EnvOverridable, Serialize, DumpableConfig)]
pub struct DatabaseConfig {
    /// Directory in which this database is stored
    pub path: PathBuf,
    /// Filename under the directory specified in `path`.
    pub filename: String,
    /// Amount of memory to reserve for the database layer's
    /// caches for this database type.
    ///
    /// Can be specified as a bare value of bytes (e.g., 1024000), a unit-ed amount
    /// (e.g., 1024MiB), or a percentage (e.g., 20%), which will be applied against
    /// the current cgroup limit if present and the total system memory otherwise
    #[serde(default = "defaults::default_database_size")]
    pub cache_size: MemorySize,
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
            return Err(Error::internal(e));
        }
        if !dir.is_dir() {
            return Err(Error::internal(format!(
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

    pub fn as_path(&self) -> &Path {
        &self.inner
    }
}

impl From<Dir> for PathBuf {
    fn from(value: Dir) -> Self {
        value.inner
    }
}

impl DatabaseConfig {
    pub fn database(&self, time: Monotime, label: &'static str) -> Result<fjall::Database> {
        let dir = Dir::new(&self.path)?;
        let path = dir.join(&self.filename);
        if path.exists() {
            tracing::info!(path = %path.display(), "loading existing {} database", label);
        } else {
            tracing::debug!(path = %path.display(), "initializing new {} database", label);
        }
        let cache_size = self.cache_size;
        fjall::Database::builder(path)
            .cache_size(cache_size.as_bytes())
            .manual_journal_persist(true)
            .with_compaction_filter_factories(Arc::new(move |keyspace| match keyspace {
                diom_msgs::METADATA_KEYSPACE => Some(Arc::new(
                    diom_msgs::compaction::IdempotencyExpiryFilterFactory::new(time.clone()),
                )),
                _ => None,
            }))
            .open()
            .map_err(|err| {
                tracing::error!(?err, "error building database");
                err.into()
            })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, EnvOverridable, DumpableConfig)]
#[validate(schema(
    function = "validators::validate_cluster_configuration",
    skip_on_field_errors = true
))]
pub struct ClusterConfiguration {
    /// The address to listen on for replication.
    #[serde(default = "defaults::cluster_listen_address")]
    pub listen_address: SocketAddr,

    /// Human-facing name for this cluster.
    ///
    /// Only used in discovery and debugging.
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

    /// Shared secret for intra-cluster communications
    ///
    /// This must be the same on all nodes.
    #[serde(default)]
    pub secret: Option<String>,

    /// Timeout for replication requests.
    ///
    /// This should be set to approximately 2X the RTT of your farthest-apart nodes.
    #[serde(
        rename = "replication_request_timeout_ms",
        default = "defaults::cluster_replication_request_timeout"
    )]
    pub replication_request_timeout: DurationMs,

    /// Timeout for discovery requests.
    ///
    /// This should be set to approximately 2X the RTT of your farthest-apart nodes.
    #[serde(
        rename = "discovery_request_timeout_ms",
        default = "defaults::cluster_discovery_request_timeout"
    )]
    pub discovery_request_timeout: DurationMs,

    /// Timeout for new connections.
    ///
    /// If you want to be tolerant of dropped packets, this should be set to at least TO + ε,
    /// where TO is the initial TCP retransmission timer (typically either 1s or 3s,
    /// depending on your operating system).
    #[serde(
        rename = "connection_timeout_ms",
        default = "defaults::cluster_connection_timeout"
    )]
    pub connection_timeout: DurationMs,

    /// How often to send heartbeats.
    ///
    /// This controls how fast lost leaders can be detected.
    /// Must not be less than `replication_request_timeout`.
    #[serde(
        rename = "heartbeat_interval_ms",
        default = "defaults::cluster_heartbeat_interval"
    )]
    pub heartbeat_interval: DurationMs,

    /// The minimum time to let an election run for.
    ///
    /// This should be set to at least 4x the RTT of your farthest-apart nodes,
    /// and must not be less than `heartbeat_interval_ms`.
    #[serde(
        rename = "election_timeout_min_ms",
        default = "defaults::cluster_election_timeout_min"
    )]
    pub election_timeout_min: DurationMs,

    /// The minimum time to let an election run for.
    ///
    /// This should be set to at least 5x the RTT of your farthest-apart nodes
    /// and must not be less than `cluster_election_timeout_max`.
    #[serde(
        rename = "election_timeout_max_ms",
        default = "defaults::cluster_election_timeout_max"
    )]
    pub election_timeout_max: DurationMs,

    /// The minimum time to let an election run for.
    ///
    /// This should be set to at least 5x the RTT of your farthest-apart nodes
    /// and must not be less than `cluster_election_timeout_max`.
    #[serde(
        rename = "send_snapshot_ms",
        default = "defaults::cluster_send_snapshot_timeout"
    )]
    pub send_snapshot_timeout: DurationMs,

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
        default = "defaults::cluster_discovery_timeout"
    )]
    pub discovery_timeout: DurationMs,

    #[serde(
        rename = "startup_discovery_delay_ms",
        default = "defaults::cluster_startup_discovery_delay"
    )]
    pub startup_discovery_delay: DurationMs,

    #[serde(
        rename = "log_index_interval_ms",
        default = "defaults::log_index_interval"
    )]
    pub log_index_interval: DurationMs,

    /// Interval (in transactions) between fsyncing the commit log.
    ///
    /// This can be used to force transactions to fsync logs more often than the
    /// default `log_sync_interval_ms` timer. If `log_sync_mode` is set to "buffer", it's
    /// reasonable to set this value to `1` to flush to the OS buffer on every log.
    ///
    /// If this is set to 0, only the interval timer will be used
    ///
    /// If this is set to a value higher than 1 and the interval timer is long, then
    /// single-threaded clients (including bootstrap) will be extremely slow.
    #[validate(range(min = 0, max = 1024000))]
    #[serde(default = "defaults::cluster_log_sync_interval_commits")]
    pub log_sync_interval_commits: usize,

    /// Interval (in milliseconds) between fsyncing the commit log.
    ///
    /// If `log_sync_interval_auto` is set to true, this is just the initial estimate
    /// and will be auto-scaled
    #[validate(custom(function = "validators::validate_log_sync_interval_duration"))]
    #[serde(
        rename = "log_sync_interval_ms",
        default = "defaults::cluster_log_sync_interval_duration"
    )]
    pub log_sync_interval_duration: DurationMs,

    /// Automatically attempt to determine the log sync interval from observed fsync timings
    #[serde(default = "defaults::default_true")]
    pub log_sync_interval_auto: bool,

    /// Should a log sync actually trigger an fsync?
    ///
    /// If this is set to "buffer" and a node suffers a catastrophic failure where OS buffers
    /// are not written to disk, that node should be erased and re-snapshotted before being
    /// re-added to the cluster.
    #[serde(default = "SyncMode::sync")]
    pub log_sync_mode: SyncMode,

    /// Trigger a background snapshot after this many writes
    pub snapshot_after_writes: Option<u32>,

    /// Trigger a background snapshot after this many milliseconds
    #[serde(
        rename = "snapshot_after_ms",
        default = "defaults::cluster_snapshot_after_time"
    )]
    pub snapshot_after_time: Option<DurationMs>,

    /// Shut down the process when the it is evicted from the cluster
    ///
    /// This should be true unless you are testing internal details of the replication system
    #[serde(default = "defaults::default_true")]
    #[env_overridable(skip)]
    #[dumpable_config(skip)]
    pub shut_down_on_go_away: bool,

    /// How many commits behind must the current node be to be considered "lagging" and eligible for
    /// re-snapshotting?
    ///
    /// The ideal value here depends both on your data-set size and on your average write-rate. If
    /// your data is large, then setting this value too small can mean that a snapshot can never
    /// catch up because it'll take too long to replicate. Typically this should be around twice the
    /// number of commits that you generate in the time it takes to replicate a full snapshot.
    #[serde(default = "defaults::cluster_replication_lag_threshold")]
    pub replication_lag_threshold: u64,
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
}

impl Default for ClusterConfiguration {
    fn default() -> Self {
        default_from_serde().unwrap()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpenTelemetryProtocol {
    #[default]
    Grpc,
    Http,
}

impl FromStr for OpenTelemetryProtocol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "grpc|Grpc|GRPC" => Ok(OpenTelemetryProtocol::Grpc),
            "http|Http|HTTP" => Ok(OpenTelemetryProtocol::Http),
            _ => anyhow::bail!("Unable to parse OpenTelemetryProtocol"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate, EnvOverridable, DumpableConfig)]
pub struct OpenTelemetryConfig {
    /// The OpenTelemetry address to send events to if given.
    ///
    /// Currently only GRPC exports are supported.
    pub address: Option<String>,

    /// The OpenTelemetry address to send metrics to if given.
    ///
    /// If not specified, the server will attempt to fall back
    /// to `opentelemetry_address`.
    pub metrics_address: Option<String>,

    /// OpenTelemetry metrics protocol
    ///
    /// By default, metrics are sent via GRPC. Some metrics destinations, most
    /// notably Prometheus, only support receiving metrics via HTTP.
    #[serde(default)]
    pub metrics_protocol: OpenTelemetryProtocol,

    #[serde(
        rename = "metrics_period_ms",
        default = "defaults::opentelemetry_metrics_period"
    )]
    pub metrics_period: DurationMs,

    /// The ratio at which to sample spans when sending to OpenTelemetry.
    ///
    /// When not given it defaults to always sending.
    /// If the OpenTelemetry address is not set, this will do nothing.
    pub sample_ratio: Option<f64>,

    /// The service name to use for OpenTelemetry. If not provided, it defaults to "diom".
    #[serde(default = "defaults::opentelemetry_service_name")]
    pub service_name: String,
}

impl Default for OpenTelemetryConfig {
    fn default() -> Self {
        default_from_serde().unwrap()
    }
}

fn default_from_serde<T: DeserializeOwned>() -> Result<T, serde::de::value::Error> {
    let empty: [(String, String); 0] = [];
    T::deserialize(serde::de::value::MapDeserializer::new(empty.into_iter()))
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, EnvOverridable, DumpableConfig)]
pub struct ConfigurationInner {
    /// The address to listen on
    #[serde(default = "defaults::listen_address")]
    pub listen_address: SocketAddr,

    /// Storage configuration for the persistent database, which is used by most modules and
    /// should be placed on durable storage
    #[serde(default = "defaults::persistent_db")]
    #[env_overridable(nest_with_prefix("PERSISTENT_DB"))]
    #[dumpable_config(nest)]
    pub persistent_db: DatabaseConfig,

    /// Storage configuration for the ephemeral database, which is used by rate-limiting and some
    /// other modules, and can be placed on less-durable storage, such as local instance storage
    #[serde(default = "defaults::ephemeral_db")]
    #[env_overridable(nest_with_prefix("EPHEMERAL_DB"))]
    #[dumpable_config(nest)]
    pub ephemeral_db: DatabaseConfig,

    /// The log level to run the service with. Supported: info, debug, trace
    #[serde(default)]
    pub log_level: LogLevel,

    /// The log format that all output will follow. Supported: default, json
    #[serde(default)]
    pub log_format: LogFormat,

    /// The environment (dev, staging, or prod) that the server is running in.
    #[serde(default)]
    pub environment: Environment,

    /// Configuration for the cluster/replication system
    #[validate(nested)]
    #[serde(default)]
    #[env_overridable(nest_with_prefix("CLUSTER"))]
    #[dumpable_config(nest)]
    pub cluster: ClusterConfiguration,

    /// The path to a YAML bootstrap file
    #[serde(default)]
    pub bootstrap_cfg_path: Option<String>,

    /// YAML bootstrap data. Takes precedence over `bootstrap_cfg_path`
    #[serde(default)]
    pub bootstrap_cfg: Option<String>,

    /// Maximum time to wait for cluster initialization at startup
    ///
    /// If this is unset, we will wait indefinitely.
    #[serde(rename = "bootstrap_max_wait_ms", default)]
    pub bootstrap_max_wait_time: Option<DurationMs>,

    /// How often to run background cleanup/garbage collection jobs
    ///
    /// Correctness should never be affected by this, just wasted memory/disk.
    #[serde(
        rename = "background_cleanup_interval_ms",
        default = "defaults::background_cleanup_interval"
    )]
    pub background_cleanup_interval: DurationMs,

    /// How to persist data to the actual underlying database
    ///
    /// This is similar to the `cluster.log_sync` options, but applies to the actual
    /// primary data as opposed to the log, and is applied at every batch commit from the
    /// underlying replication system.
    #[serde(default = "SyncMode::buffer")]
    pub sync_mode: SyncMode,

    /// When fsyncing, should we use fsync(2) or fdatasync(2)
    #[serde(default)]
    pub fsync_mode: FsyncMode,

    /// An auth token for admin access to Diom
    ///
    /// It's useful for bootstrapping a new Diom cluster, or using it in CI pipelines or other
    /// automated testing environments where you need a stable, well-known token for testing or
    /// scripted setup.
    ///
    /// If you're using it to bootstrap a new Diom cluster, it’s recommended that you only use
    /// it during bootstrapping, and remove this configuration once done.
    #[validate(custom(function = "validators::validate_admin_token"))]
    #[serde(default)]
    pub admin_token: Option<String>,

    /// Configuration for verifying JWT bearer tokens.
    ///
    /// When set, bearer tokens in JWT format are verified using this configuration.
    /// The JWT must contain a `role` claim (string) and may contain a `context` claim
    /// (object with string values) that is forwarded to internal diom handlers.
    #[serde(default)]
    #[env_overridable(nest_with_prefix("JWT"))]
    #[dumpable_config(nest)]
    #[validate(nested)]
    pub jwt: JwtConfig,

    #[serde(default)]
    #[env_overridable(nest_with_prefix("OPENTELEMETRY"))]
    #[dumpable_config(nest)]
    pub opentelemetry: OpenTelemetryConfig,
}

impl ConfigurationInner {
    /// Write the current configuration to the given Writer as TOML (with comments!)
    pub fn dump_config<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        self.dump_fields(writer, "".to_string())
    }
}

/// JWT configuration for verifying JWT bearer tokens.
///
/// The `algorithm` field selects the signature scheme:
/// - Symmetric (`HS256`, `HS384`, `HS512`) — requires `secret`.
/// - Asymmetric (`RS256`/`RS384`/`RS512`, `ES256`/`ES384`, `PS256`/`PS384`/`PS512`) —
///   requires `public_key_pem`.
///
/// The optional `audience` and `issuer` fields enable claim validation.  When
/// omitted, the respective claims are not checked (and may be absent from the
/// token).
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, Validate, EnvOverridable, DumpableConfig,
)]
pub struct JwtConfig {
    #[serde(flatten)]
    pub key: Option<JwtKey>,
    /// Expected `aud` values. When set, the token must contain one of these
    /// values in its `aud` claim. When absent, `aud` is not validated.
    #[serde(default)]
    #[validate(length(min = 1, message = "audience must not be empty when set"))]
    pub audience: Option<Vec<String>>,
    /// Expected `iss` values. When set, the token's `iss` claim must match one
    /// of these values. When absent, `iss` is not validated.
    #[serde(default)]
    #[validate(length(min = 1, message = "issuer must not be empty when set"))]
    pub issuer: Option<Vec<String>>,
}

/// Signature algorithm and corresponding key material.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "algorithm")]
pub enum JwtKey {
    #[serde(rename = "HS256")]
    Hs256 { secret: String },
    #[serde(rename = "HS384")]
    Hs384 { secret: String },
    #[serde(rename = "HS512")]
    Hs512 { secret: String },
    #[serde(rename = "RS256")]
    Rs256 { public_key_pem: String },
    #[serde(rename = "RS384")]
    Rs384 { public_key_pem: String },
    #[serde(rename = "RS512")]
    Rs512 { public_key_pem: String },
    #[serde(rename = "ES256")]
    Es256 { public_key_pem: String },
    #[serde(rename = "ES384")]
    Es384 { public_key_pem: String },
    #[serde(rename = "PS256")]
    Ps256 { public_key_pem: String },
    #[serde(rename = "PS384")]
    Ps384 { public_key_pem: String },
    #[serde(rename = "PS512")]
    Ps512 { public_key_pem: String },
}

impl JwtKey {
    fn algorithm(&self) -> &'static str {
        match self {
            Self::Hs256 { .. } => "HS256",
            Self::Hs384 { .. } => "HS384",
            Self::Hs512 { .. } => "HS512",
            Self::Rs256 { .. } => "RS256",
            Self::Rs384 { .. } => "RS384",
            Self::Rs512 { .. } => "RS512",
            Self::Es256 { .. } => "ES256",
            Self::Es384 { .. } => "ES384",
            Self::Ps256 { .. } => "PS256",
            Self::Ps384 { .. } => "PS384",
            Self::Ps512 { .. } => "PS512",
        }
    }

    fn secret(&self) -> Option<&str> {
        match self {
            Self::Hs256 { secret } | Self::Hs384 { secret } | Self::Hs512 { secret } => {
                Some(secret)
            }
            _ => None,
        }
    }

    fn public_key_pem(&self) -> Option<&str> {
        match self {
            Self::Rs256 { public_key_pem }
            | Self::Rs384 { public_key_pem }
            | Self::Rs512 { public_key_pem }
            | Self::Es256 { public_key_pem }
            | Self::Es384 { public_key_pem }
            | Self::Ps256 { public_key_pem }
            | Self::Ps384 { public_key_pem }
            | Self::Ps512 { public_key_pem } => Some(public_key_pem),
            _ => None,
        }
    }

    fn set_secret(&mut self, value: String) -> bool {
        match self {
            Self::Hs256 { secret } | Self::Hs384 { secret } | Self::Hs512 { secret } => {
                *secret = value;
                true
            }
            _ => false,
        }
    }

    fn set_public_key_pem(&mut self, value: String) -> bool {
        match self {
            Self::Rs256 { public_key_pem }
            | Self::Rs384 { public_key_pem }
            | Self::Rs512 { public_key_pem }
            | Self::Es256 { public_key_pem }
            | Self::Es384 { public_key_pem }
            | Self::Ps256 { public_key_pem }
            | Self::Ps384 { public_key_pem }
            | Self::Ps512 { public_key_pem } => {
                *public_key_pem = value;
                true
            }
            _ => false,
        }
    }
}

impl Default for ConfigurationInner {
    fn default() -> Self {
        default_from_serde().unwrap()
    }
}

impl DumpableConfig for Option<JwtKey> {
    fn dump_fields<W: std::io::Write>(
        &self,
        writer: &mut W,
        _prefix: String,
    ) -> anyhow::Result<()> {
        let mut buffer = String::new();

        // algorithm
        writeln!(writer, "# JWT algorithm.")?;
        writeln!(writer, "#")?;
        writeln!(
            writer,
            "# Supported values are HS256, HS384, HS512, RS256, RS384, RS512, ES256, ES384, PS256, PS384, PS512."
        )?;
        writeln!(
            writer,
            "# values in its `aud` claim. When absent, `aud` is not validated.",
        )?;
        if let Some(key) = self {
            let serialized = key
                .algorithm()
                .serialize(toml::ser::ValueSerializer::new(&mut buffer))?;
            writeln!(writer, "algorithm = {serialized}")?;
            buffer.clear();
        } else {
            writeln!(writer, "# algorithm =")?;
        }

        // secret
        writeln!(writer)?;
        writeln!(writer, "# Secret for JWT algorithm HS256, HS384 or HS512")?;
        if let Some(key) = self
            && let Some(secret) = key.secret()
        {
            let serialized = secret.serialize(toml::ser::ValueSerializer::new(&mut buffer))?;
            writeln!(writer, "secret = {serialized}")?;
            buffer.clear();
        } else {
            writeln!(writer, "# secret =")?;
        };

        // public_key_pem
        writeln!(writer)?;
        writeln!(
            writer,
            "# Public key PEM for JWT algorithm RS256, RS384, RS512, ES256, ES384, PS256, PS384 or PS512",
        )?;
        if let Some(key) = self
            && let Some(public_key_pem) = key.public_key_pem()
        {
            let serialized =
                public_key_pem.serialize(toml::ser::ValueSerializer::new(&mut buffer))?;
            writeln!(writer, "public_key_pem = {serialized}")?;
            buffer.clear();
        } else {
            writeln!(writer, "# public_key_pem =")?;
        };

        Ok(())
    }
}

impl EnvOverridable for Option<JwtKey> {
    fn load_environment_with_prefix(&mut self, prefix: String) -> anyhow::Result<()> {
        let algorithm_var = format!("{prefix}_ALGORITHM");
        let secret_var = format!("{prefix}_SECRET");
        let public_key_pem_var = format!("{prefix}_PUBLIC_KEY_PEM");

        let secret: Option<String> = env_var(&secret_var)?;
        let public_key_pem: Option<String> = env_var(&public_key_pem_var)?;
        let Some(algorithm): Option<String> = env_var(&algorithm_var)? else {
            if let Some(secret) = secret {
                if let Some(key) = self {
                    if !key.set_secret(secret) {
                        tracing::warn!(
                            "ignoring {secret_var} as {} algorithm uses {public_key_pem_var}",
                            key.algorithm(),
                        );
                    }
                } else {
                    bail!("must set {algorithm_var} for {secret_var} to be meaningful");
                }
            }

            if let Some(public_key_pem) = public_key_pem {
                if let Some(key) = self {
                    if !key.set_public_key_pem(public_key_pem) {
                        tracing::warn!(
                            "ignoring {public_key_pem_var} as {} algorithm uses {secret_var}",
                            key.algorithm(),
                        );
                    }
                } else {
                    bail!("must set {algorithm_var} for {public_key_pem_var} to be meaningful");
                }
            }

            return Ok(());
        };

        // if algorithm is set in the environment, require secret or public key PEM
        // to also be set in the environment (type from env + value from config seems broken)

        let secret_is_set = secret.is_some();
        let public_key_pem_is_set = public_key_pem.is_some();

        let get_secret = || {
            secret.with_context(|| format!("{algorithm} algorithm requires {secret_var} to be set"))
        };
        let get_public_key_pem = || {
            public_key_pem.with_context(|| {
                format!("{algorithm} algorithm requires {public_key_pem_var} to be set")
            })
        };

        let key = match algorithm.as_str() {
            "HS256" => JwtKey::Hs256 {
                secret: get_secret()?,
            },
            "HS384" => JwtKey::Hs384 {
                secret: get_secret()?,
            },
            "HS512" => JwtKey::Hs512 {
                secret: get_secret()?,
            },
            "RS256" => JwtKey::Rs256 {
                public_key_pem: get_public_key_pem()?,
            },
            "RS384" => JwtKey::Rs384 {
                public_key_pem: get_public_key_pem()?,
            },
            "RS512" => JwtKey::Rs512 {
                public_key_pem: get_public_key_pem()?,
            },
            "ES256" => JwtKey::Es256 {
                public_key_pem: get_public_key_pem()?,
            },
            "ES384" => JwtKey::Es384 {
                public_key_pem: get_public_key_pem()?,
            },
            "PS256" => JwtKey::Ps256 {
                public_key_pem: get_public_key_pem()?,
            },
            "PS384" => JwtKey::Ps384 {
                public_key_pem: get_public_key_pem()?,
            },
            "PS512" => JwtKey::Ps512 {
                public_key_pem: get_public_key_pem()?,
            },
            _ => bail!("unsupported value for {algorithm_var}"),
        };

        match algorithm.as_str() {
            "HS256" | "HS384" | "HS512" => {
                if public_key_pem_is_set {
                    tracing::warn!(
                        "ignoring {public_key_pem_var} as {algorithm} algorithm uses {secret_var}"
                    );
                }
            }
            _ => {
                if secret_is_set {
                    tracing::warn!(
                        "ignoring {secret_var} as {algorithm} algorithm uses {public_key_pem_var}"
                    );
                }
            }
        }

        *self = Some(key);
        Ok(())
    }

    fn list_environment_variables_with_prefix(prefix: String) -> Vec<Variable> {
        vec![
            Variable {
                env_var: if prefix.is_empty() {
                    "ALGORITHM".to_owned()
                } else {
                    format!("{prefix}_ALGORITHM")
                },
                docstring: Some("JWT signature algorithm"),
            },
            Variable {
                env_var: if prefix.is_empty() {
                    "SECRET".to_owned()
                } else {
                    format!("{prefix}_SECRET")
                },
                docstring: Some("Secret for JWT algorithms HS256 / HS384 / HS512"),
            },
            Variable {
                env_var: if prefix.is_empty() {
                    "PUBLIC_KEY_PEM".to_owned()
                } else {
                    format!("{prefix}_PUBLIC_KEY_PEM")
                },
                docstring: Some(
                    "Public key PEM for JWT algorithms RS256 / RS384 / RS512 / ES256 / ES384 / PS256 / PS384 / PS512",
                ),
            },
        ]
    }
}

#[macro_export]
macro_rules! from_str_via_serde {
    ($ty:ty) => {
        impl std::str::FromStr for $ty {
            type Err = serde::de::value::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <Self as serde::Deserialize>::deserialize(serde::de::value::StrDeserializer::new(s))
            }
        }
    };
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

from_str_via_serde!(LogLevel);

impl From<LogLevel> for Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogFormat {
    #[default]
    Default,
    Json,
}

from_str_via_serde!(LogFormat);

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
        Level::from(*self).fmt(f)
    }
}

/// How data is synchronized to the underlying database
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SyncMode {
    /// Write data to the OS, but do not fsync
    Buffer,
    /// fsync the data on every batch apply
    Sync,
}

from_str_via_serde!(SyncMode);

impl SyncMode {
    fn buffer() -> Self {
        Self::Buffer
    }

    fn sync() -> Self {
        Self::Sync
    }

    pub fn into_persist_mode(&self, fsync_mode: FsyncMode) -> fjall::PersistMode {
        match self {
            Self::Buffer => fjall::PersistMode::Buffer,
            Self::Sync => fsync_mode.into(),
        }
    }
}

/// How data is synchronized to the underlying database
///
/// In general, sync-data is faster but may not be safe on some platforms. Please consult
/// your operating system documentation for more details.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FsyncMode {
    /// fdatasync(2)
    #[default]
    SyncData,
    /// fsync(2)
    SyncAll,
}

from_str_via_serde!(FsyncMode);

impl From<FsyncMode> for fjall::PersistMode {
    fn from(value: FsyncMode) -> Self {
        match value {
            FsyncMode::SyncData => fjall::PersistMode::SyncData,
            FsyncMode::SyncAll => fjall::PersistMode::SyncAll,
        }
    }
}

pub fn load(config_path: Option<&Path>) -> anyhow::Result<Arc<ConfigurationInner>> {
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

    config.load_environment()?;

    config.validate()?;

    Ok(Arc::from(config))
}

pub fn describe_environment() -> Vec<Variable> {
    ConfigurationInner::list_environment_variables()
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::{PeerAddr, load_toml};

    #[test]
    fn test_db() {
        let raw_config = r#"
ephemeral_db = { path = "/1", filename = "test1" }
persistent_db = { path = "/2", filename = "fjall_persistent" }
"#;

        let config = load_toml(Some(raw_config)).unwrap();

        assert_eq!(config.ephemeral_db.path, "/1".to_string());
        assert_eq!(config.ephemeral_db.filename, "test1".to_string());

        assert_eq!(config.persistent_db.path, "/2".to_string());
        assert_eq!(
            config.persistent_db.filename,
            "fjall_persistent".to_string()
        );
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
        let Ok(PeerAddr::SocketAddr(SocketAddr::V4(sa))) = "127.0.0.2:8624".parse() else {
            panic!("failed to parse v4 addr")
        };
        assert_eq!(sa.port(), 8624);
        assert_eq!(*sa.ip(), Ipv4Addr::new(127, 0, 0, 2));
        let Ok(PeerAddr::SocketAddr(SocketAddr::V6(sa))) = "[::1001:2%1234]:8624".parse() else {
            panic!("failed to parse v6 addr");
        };
        assert_eq!(sa.scope_id(), 1234);
        assert_eq!(sa.port(), 8624);
        assert_eq!(*sa.ip(), Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0x1001, 0x2));
        assert_eq!(
            "foobar:8624".parse::<PeerAddr>().expect("should parse"),
            PeerAddr::HostnameAndPort {
                hostname: "foobar".to_owned(),
                port: 8624
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
                8624,
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
