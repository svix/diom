mod cache;
mod kv;
mod msgpack;
mod rate_limiter;
mod stream;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use diom::{
    cfg::{
        ClusterConfiguration, ConfigurationInner, DatabaseConfig, Environment, InternalConfig,
        LogFormat, LogLevel,
    },
    core::security::JwtSigningConfig,
    run_with_prefix,
};
use jwt_simple::prelude::*;
use tempfile::TempDir;
use test_utils::TestClient;
use tokio::{net::TcpListener, task::JoinHandle};

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

async fn start_server() -> (TestClient, IsolatedServerHandle) {
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
        opentelemetry_service_name: "diom-test".to_string(),
        environment: Environment::Dev,
        internal: InternalConfig {},
        cluster: ClusterConfiguration {
            listen_address: repl_addr,
            name: "diom-test".to_string(),
            snapshot_path,
            log_path,
            connection_timeout: Duration::from_millis(50),
            heartbeat_interval_ms: 100,
            election_timeout_min_ms: 200,
            election_timeout_max_ms: 300,
        },
    });

    let base_uri = format!("http://{addr}/api/v1");

    let server_handle = tokio::spawn(async move {
        run_with_prefix(cfg, Some(listener), Some(repl_listener)).await;
    });

    let handle = IsolatedServerHandle {
        _dir: workdir,
        server_handle,
    };

    let client = TestClient::new(base_uri, token);

    (client, handle)
}
