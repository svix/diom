mod bootstrap;
mod cache;
mod configgroup;
mod idempotency;
mod kv;
mod msgpack;
mod rate_limiter;
mod stream;

use std::{net::SocketAddr, sync::Arc};

use coyote::{
    cfg::{
        ClusterConfiguration, ConfigurationInner, DatabaseConfig, Environment, InternalConfig,
        LogFormat, LogLevel,
    },
    core::{cluster::proto::HealthResponse, security::JwtSigningConfig},
    run_with_prefix,
};
use jwt_simple::prelude::*;
use tempfile::TempDir;
use test_utils::TestClient;
use tokio::{
    net::TcpListener,
    task::JoinHandle,
    time::{Duration, Instant},
};

/// Handle to an isolated test server.
///
/// Once it's DROPed, the server and it's resources are cleaned up automatically (or at least, that's the intent.)
pub struct IsolatedServerHandle {
    _dir: TempDir,
    server_handle: JoinHandle<()>,
}

impl Drop for IsolatedServerHandle {
    fn drop(&mut self) {
        self.server_handle.abort();
    }
}

pub struct TestContext {
    pub client: TestClient,
    pub cfg: Arc<ConfigurationInner>,
    pub handle: IsolatedServerHandle,
}

async fn check_initialized(client: &reqwest::Client, url: &str) -> anyhow::Result<bool> {
    let response = client
        .get(url)
        .timeout(Duration::from_millis(10))
        .send()
        .await?;
    if response.status() != 200 {
        return Ok(false);
    }
    let body: HealthResponse = response.json().await?;
    if body.server_state.is_leader() {
        tracing::warn!(state=?body.server_state, "booted, but not leader");
        return Ok(false);
    }
    Ok(true)
}

async fn wait_for_initialized(repl_addr: SocketAddr, max_wait: Duration) -> anyhow::Result<()> {
    tracing::info!("waiting for server to boot up");
    let url = format!("http://{repl_addr}/repl/health");
    let deadline = Instant::now() + max_wait;
    let client = reqwest::Client::new();
    loop {
        match tokio::time::timeout_at(deadline, check_initialized(&client, &url)).await {
            Ok(Ok(true)) => {
                tracing::info!("server started!");
                return Ok(());
            }
            Ok(Ok(false)) => {
                tracing::debug!("server not yet up");
            }
            Ok(Err(err)) => {
                tracing::warn!(?err, "error waiting for server to boot");
            }
            Err(_) => anyhow::bail!("timed out waiting for server to boot"),
        }
    }
}

async fn start_server() -> TestContext {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();

    let repl_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let repl_addr: SocketAddr = repl_listener.local_addr().unwrap();

    let jwt_key = HS256Key::generate();
    let token = "Stubbed token. Should probably be legit when we add auth.";

    let workdir = tempfile::tempdir().unwrap();
    let db_dir = workdir.path().join("db");
    let log_path = workdir.path().join("logs");
    let snapshot_path = workdir.path().join("snapshots");

    let cfg = Arc::new(ConfigurationInner {
        listen_address: addr,
        ephemeral_db: Arc::new(DatabaseConfig {
            path: db_dir.clone(),
            ..Default::default()
        }),
        persistent_db: Arc::new(DatabaseConfig {
            path: db_dir,
            ..Default::default()
        }),
        jwt_signing_config: Arc::new(JwtSigningConfig::HS256(jwt_key)),
        log_level: LogLevel::Debug,
        log_format: LogFormat::Default,
        opentelemetry_address: None,
        opentelemetry_metrics_use_http: false,
        opentelemetry_metrics_period_seconds: 60,
        opentelemetry_sample_ratio: None,
        opentelemetry_service_name: "coyote-test".to_string(),
        environment: Environment::Dev,
        internal: InternalConfig {},
        cluster: ClusterConfiguration {
            listen_address: repl_addr,
            name: "coyote-test".to_string(),
            snapshot_path,
            log_path,
            connection_timeout: Duration::from_millis(50),
            heartbeat_interval_ms: 100,
            election_timeout_min_ms: 200,
            election_timeout_max_ms: 300,
            auto_initialize: true,
            discovery_request_timeout: Duration::from_millis(10),
            discovery_timeout: Duration::from_secs(1),
            secret: None,
            seed_nodes: vec![],
            replication_request_timeout: Duration::from_millis(50),
            startup_discovery_delay: Duration::from_millis(0),
        },
    });

    let base_uri = format!("http://{addr}/api/v1");

    // TODO: this directly touches the database, so causes crashes if it runs
    // concurrently with anything else that reads the databases. Should it go through
    // the handle in the app somehow instead?
    coyote::bootstrap::run(None, cfg.clone());

    let server_handle = tokio::spawn({
        let cfg = cfg.clone();
        async move {
            run_with_prefix(cfg, Some(listener), Some(repl_listener)).await;
        }
    });

    let handle = IsolatedServerHandle {
        _dir: workdir,
        server_handle,
    };

    let client = TestClient::new(base_uri, token);

    wait_for_initialized(repl_addr, Duration::from_secs(2))
        .await
        .expect("failed to initialize server");

    TestContext {
        client,
        cfg,
        handle,
    }
}
