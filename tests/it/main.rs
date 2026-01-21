use std::{net::SocketAddr, sync::Arc};

use diom::{
    cfg::{ConfigurationInner, Environment, InternalConfig, LogFormat, LogLevel},
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
    _db_dir: TempDir,
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

    let jwt_key = HS256Key::generate();
    let token = "Stubbed token. Should probably be legit when we add auth.";

    let db_dir = tempfile::tempdir().unwrap();

    let cfg = Arc::new(ConfigurationInner {
        listen_address: addr,
        db_directory: db_dir.path().to_string_lossy().into_owned(),
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
    });

    let base_uri = format!("http://{addr}/api/v1");

    let server_handle = tokio::spawn(async move {
        run_with_prefix(cfg, Some(listener)).await;
    });

    let handle = IsolatedServerHandle {
        _db_dir: db_dir,
        server_handle,
    };

    let client = TestClient::new(base_uri, token);

    (client, handle)
}
