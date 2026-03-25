use std::{net::SocketAddr, path::Path, sync::Arc};

use crate::TestClient;
use coyote::{
    cfg::{
        ClusterConfiguration, ConfigurationInner, DatabaseConfig, Environment, LogFormat, LogLevel,
    },
    core::cluster::{ClusterId, NodeId, proto::HealthResponse},
    run_with_listeners,
};
use coyote_core::Monotime;
use futures_util::TryFutureExt;
use tempfile::TempDir;
use tokio::{
    net::TcpListener,
    task::JoinHandle,
    time::{Duration, Instant},
};

/// Handle to an isolated test server.
///
/// Once it's DROPed, the server and it's resources are cleaned up automatically (or at least, that's the intent.)
pub struct IsolatedServerHandle {
    _dir: Option<TempDir>,
    server_handle: JoinHandle<()>,
}

impl Drop for IsolatedServerHandle {
    fn drop(&mut self) {
        self.server_handle.abort();
    }
}

pub struct TestServerBuilder {
    cfg: ConfigurationInner,
    token: Option<String>,
    listener: Option<TcpListener>,
    repl_listener: Option<TcpListener>,
    workdir: Option<TempDir>,
}

impl TestServerBuilder {
    pub fn with_default_config() -> Self {
        let workdir = tempfile::tempdir().unwrap();
        let cfg = default_server_config(workdir.path());
        Self {
            cfg,
            workdir: Some(workdir),
            token: None,
            listener: None,
            repl_listener: None,
        }
    }

    /// Mutate the current configuration.
    ///
    /// Panics if no config is set (that is to say, if TestServerBuilder wasn't
    /// created with `.with_default_cfg`, or `.cfg` has not been called).
    pub fn tap_cfg(mut self, f: impl FnOnce(&mut ConfigurationInner)) -> Self {
        f(&mut self.cfg);
        self
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn listener(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }

    pub fn repl_listener(mut self, listener: TcpListener) -> Self {
        self.repl_listener = Some(listener);
        self
    }

    /// Replace the configuration for this test server with a new one
    pub fn cfg(mut self, cfg: ConfigurationInner) -> Self {
        self.cfg = cfg;
        self
    }

    pub async fn build(self) -> TestContext {
        let token = self.token.unwrap_or_else(|| TEST_ADMIN_TOKEN.to_string());

        let listener = if let Some(listener) = self.listener {
            listener
        } else {
            TcpListener::bind("127.0.0.1:0").await.unwrap()
        };

        let repl_listener = if let Some(listener) = self.repl_listener {
            listener
        } else {
            TcpListener::bind("127.0.0.1:0").await.unwrap()
        };

        let addr: SocketAddr = listener.local_addr().unwrap();
        let repl_addr: SocketAddr = repl_listener.local_addr().unwrap();

        let cfg = {
            let mut cfg = self.cfg;
            cfg.listen_address = addr;
            cfg.cluster.listen_address = Some(repl_addr);
            cfg.cluster.advertised_address = Some(repl_addr.into());
            Arc::new(cfg)
        };

        let base_uri = format!("http://{addr}/api");

        let time = Monotime::initial();
        time.update_now();

        let server_handle = tokio::spawn({
            let cfg = cfg.clone();
            let time = time.clone();
            async move {
                run_with_listeners(cfg, Some(listener), Some(repl_listener), time).await;
            }
        });

        let handle = IsolatedServerHandle {
            _dir: self.workdir,
            server_handle,
        };
        let client = TestClient::new(base_uri, &token);

        let (node_id, cluster_id) = wait_for_initialized(addr, repl_addr, Duration::from_secs(8))
            .await
            .expect("failed to initialize server");

        TestContext {
            client,
            cfg,
            handle,
            token,
            addr,
            repl_addr,
            node_id,
            cluster_id,
            time,
        }
    }
}

pub struct TestContext {
    pub client: TestClient,
    pub cfg: Arc<ConfigurationInner>,
    pub handle: IsolatedServerHandle,
    pub token: String,
    pub addr: SocketAddr,
    pub repl_addr: SocketAddr,
    pub node_id: NodeId,
    pub cluster_id: ClusterId,
    pub time: Monotime,
}

async fn check_initialized(
    client: &reqwest::Client,
    url: &str,
) -> anyhow::Result<Option<(NodeId, ClusterId)>> {
    tracing::debug!("checking if server is initialized yet...");
    let response = client
        .get(url)
        .timeout(Duration::from_millis(10))
        .send()
        .await?;
    if response.status() != 200 {
        tracing::debug!(status=%response.status(), "server returned an error");
        return Ok(None);
    }
    let body: HealthResponse = response.json().await?;
    if body.server_state.is_candidate() {
        tracing::warn!(state=?body.server_state, "booted, but just a candidate");
        return Ok(None);
    }
    if let Some(cluster_id) = body.cluster_id {
        Ok(Some((body.node_id, cluster_id)))
    } else {
        Ok(None)
    }
}

async fn wait_for_initialized(
    main_addr: SocketAddr,
    repl_addr: SocketAddr,
    max_wait: Duration,
) -> anyhow::Result<(NodeId, ClusterId)> {
    tracing::info!("waiting for server to boot up");
    let main_url = format!("http://{main_addr}/api/v1.health.ping");
    let url = format!("http://{repl_addr}/repl/health");
    let deadline = Instant::now() + max_wait;
    let client = reqwest::Client::new();
    let mut backoff_ms = 10;
    let max_backoff_time = Duration::from_millis(500);
    let mut repl_booted = None;
    let mut regular_booted = false;
    loop {
        // First, check that the cluster has initialized
        match tokio::time::timeout_at(deadline, check_initialized(&client, &url)).await {
            Ok(Ok(Some(info))) => {
                tracing::info!("replication is ready");
                repl_booted = Some(info);
            }
            Ok(Ok(None)) => {
                tracing::debug!("server not yet up");
            }
            Ok(Err(err)) => {
                tracing::warn!(?err, "error waiting for server to boot");
            }
            Err(_) => anyhow::bail!("timed out waiting for server to boot"),
        }
        // Then, make sure that the regular comms port is up
        match tokio::time::timeout_at(
            deadline,
            client
                .get(&main_url)
                .timeout(Duration::from_millis(10))
                .send()
                .and_then(|r| async { r.error_for_status() }),
        )
        .await
        {
            Ok(Ok(_)) => {
                tracing::info!("regular HTTP server is ready");
                regular_booted = true
            }
            Ok(Err(err)) => {
                tracing::warn!(?err, "error waiting for server to boot");
            }
            Err(_) => anyhow::bail!("timed out waiting for server to boot"),
        }
        if regular_booted && let Some(info) = repl_booted {
            return Ok(info);
        }
        tokio::time::sleep(Duration::from_millis(backoff_ms).min(max_backoff_time)).await;
        backoff_ms *= 2;
    }
}

/// TestContext without a running server.
///
/// Since there's no background task for a server, you need to ensure to keep this context object
/// alive for your whole test to prevent the workdir from being dropped and cleaned up.
pub struct ServerlessTestContext {
    pub cfg: Arc<ConfigurationInner>,
    _workdir: TempDir,
}

pub const TEST_ADMIN_TOKEN: &str = "admin_abcdefghijlmnopqrstuvwxyz";

pub fn default_server_config(workdir: &Path) -> ConfigurationInner {
    let db_dir = workdir.join("db");
    let log_path = workdir.join("logs");
    let snapshot_path = workdir.join("snapshots");

    let addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let cluster_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();

    ConfigurationInner {
        listen_address: addr,
        ephemeral_db: DatabaseConfig {
            path: db_dir.clone(),
            ..Default::default()
        },
        persistent_db: DatabaseConfig {
            path: db_dir,
            ..Default::default()
        },
        log_level: LogLevel::Debug,
        log_format: LogFormat::Default,
        opentelemetry_address: None,
        opentelemetry_metrics_address: None,
        opentelemetry_metrics_use_http: false,
        opentelemetry_metrics_period_seconds: 60,
        opentelemetry_sample_ratio: None,
        opentelemetry_service_name: "coyote-test".to_string(),
        environment: Environment::Dev,
        bootstrap_max_wait_time: Some(Duration::from_secs(10)),
        cluster: ClusterConfiguration {
            advertised_address: None,
            listen_address: Some(cluster_addr),
            name: "coyote-test".to_string(),
            snapshot_path: Some(snapshot_path),
            log_path: Some(log_path),
            connection_timeout: Duration::from_millis(50),
            heartbeat_interval: Duration::from_millis(50),
            election_timeout_min: Duration::from_millis(100),
            election_timeout_max: Duration::from_millis(300),
            auto_initialize: true,
            discovery_request_timeout: Duration::from_secs(3),
            discovery_timeout: Duration::from_secs(10),
            secret: None,
            seed_nodes: vec![],
            replication_request_timeout: Duration::from_millis(50),
            startup_discovery_delay: Duration::from_millis(0),
            log_index_interval: Duration::from_millis(500),
            snapshot_after_writes: None,
            snapshot_after_time: None,
            log_sync_interval_commits: 0,
            log_sync_interval_duration: Duration::from_secs(30),
            log_ack_immediately: true,
            shut_down_on_go_away: true,
        },
        bootstrap_cfg: None,
        bootstrap_cfg_path: None,
        admin_token: Some(TEST_ADMIN_TOKEN.to_string()),
    }
}

pub async fn start_server() -> TestContext {
    TestServerBuilder::with_default_config().build().await
}
