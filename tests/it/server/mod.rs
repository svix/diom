use std::{net::SocketAddr, sync::Arc};

use diom::{
    cfg::{
        ClusterConfiguration, ConfigurationInner, DatabaseConfig, Environment, InternalConfig,
        LogFormat, LogLevel,
    },
    core::{cluster::proto::HealthResponse, security::JwtSigningConfig},
    run_with_prefix, setup_tracing_for_tests,
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

pub(crate) struct TestServerBuilder {
    cfg: Option<ConfigurationInner>,
    token: Option<String>,
    listener: Option<TcpListener>,
    repl_listener: Option<TcpListener>,
}

impl TestServerBuilder {
    pub(crate) fn new() -> Self {
        Self {
            cfg: None,
            token: None,
            listener: None,
            repl_listener: None,
        }
    }

    #[allow(unused)]
    pub(crate) fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    #[allow(unused)]
    pub(crate) fn listener(mut self, listener: TcpListener) -> Self {
        self.listener = Some(listener);
        self
    }

    #[allow(unused)]
    pub(crate) fn repl_listener(mut self, listener: TcpListener) -> Self {
        self.repl_listener = Some(listener);
        self
    }

    #[allow(unused)]
    pub(crate) fn cfg(mut self, cfg: ConfigurationInner) -> Self {
        self.cfg = Some(cfg);
        self
    }

    pub(crate) async fn build(self) -> TestContext {
        let token = if let Some(token) = self.token {
            token
        } else {
            "Stubbed token. Should probably be legit when we add auth.".to_string()
        };

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

        let ServerlessTestContext { cfg, workdir } = build_config_without_server(self.cfg);

        let cfg = {
            let mut cfg_inner = Arc::unwrap_or_clone(cfg);
            cfg_inner.listen_address = addr;
            cfg_inner.cluster.listen_address = repl_addr;
            Arc::new(cfg_inner)
        };

        let base_uri = format!("http://{addr}/api/v1");

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

        let client = TestClient::new(base_uri, &token);

        wait_for_initialized(repl_addr, Duration::from_secs(8))
            .await
            .expect("failed to initialize server");

        TestContext {
            client,
            cfg,
            handle,
            token,
        }
    }
}

pub struct TestContext {
    pub client: TestClient,
    pub cfg: Arc<ConfigurationInner>,
    pub handle: IsolatedServerHandle,
    pub token: String,
}

async fn check_initialized(client: &reqwest::Client, url: &str) -> anyhow::Result<bool> {
    tracing::debug!("checking if server is initialized yet...");
    let response = client
        .get(url)
        .timeout(Duration::from_millis(10))
        .send()
        .await?;
    if response.status() != 200 {
        tracing::debug!(status=%response.status(), "server returned an error");
        return Ok(false);
    }
    let body: HealthResponse = response.json().await?;
    if !body.server_state.is_leader() {
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
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

/// TestContext without a running server. Since there's no background task for a server,
/// you need to ensure to keep this context object alive for your whole test to prevent
/// the workdir from being dropped and cleaned up
pub(crate) struct ServerlessTestContext {
    pub cfg: Arc<ConfigurationInner>,
    workdir: TempDir,
}

pub(crate) fn build_config_without_server(
    cfg: Option<ConfigurationInner>,
) -> ServerlessTestContext {
    setup_tracing_for_tests();

    let jwt_key = HS256Key::generate();

    let workdir = tempfile::tempdir().unwrap();
    let db_dir = workdir.path().join("db");
    let log_path = workdir.path().join("logs");
    let snapshot_path = workdir.path().join("snapshots");

    let addr: SocketAddr = "0.0.0.0:0".parse().unwrap();

    let cfg = cfg.unwrap_or_else(|| ConfigurationInner {
        listen_address: addr,
        ephemeral_db: DatabaseConfig {
            path: db_dir.clone(),
            ..Default::default()
        },
        persistent_db: DatabaseConfig {
            path: db_dir,
            ..Default::default()
        },
        jwt_signing_config: Arc::new(JwtSigningConfig::HS256(jwt_key)),
        log_level: LogLevel::Debug,
        log_format: LogFormat::Default,
        opentelemetry_address: None,
        opentelemetry_metrics_use_http: false,
        opentelemetry_metrics_period_seconds: 60,
        opentelemetry_sample_ratio: None,
        opentelemetry_service_name: "diom-test".to_string(),
        environment: Environment::Dev,
        internal: InternalConfig {},
        cluster: ClusterConfiguration {
            listen_address: addr,
            name: "diom-test".to_string(),
            snapshot_path,
            log_path,
            connection_timeout: Duration::from_millis(50),
            heartbeat_interval_ms: 5,
            election_timeout_min_ms: 10,
            election_timeout_max_ms: 30,
            auto_initialize: true,
            discovery_request_timeout: Duration::from_millis(10),
            discovery_timeout: Duration::from_secs(1),
            secret: None,
            seed_nodes: vec![],
            replication_request_timeout: Duration::from_millis(50),
            startup_discovery_delay: Duration::from_millis(0),
        },
    });

    let cfg = Arc::new(cfg);

    // TODO: do bootstrap through the server APIs instead of just directly
    // touching the databases? Need to make sure that this never runs
    // concurrently with the other DB accesses
    diom::bootstrap::run(None, Arc::clone(&cfg));

    ServerlessTestContext { cfg, workdir }
}

pub(crate) async fn start_server() -> TestContext {
    TestServerBuilder::new().build().await
}
