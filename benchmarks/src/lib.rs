#![allow(unused)]

use coyote::cfg::LogLevel;
use coyote_client::{CoyoteClient, CoyoteOptions};
use tempfile::TempDir;
use tokio::runtime::Runtime;

use test_utils::server::{TestContext, TestServerBuilder, default_server_config};

pub struct BenchmarkContext {
    pub rt: Runtime,
    pub server: TestContext,
    pub client: CoyoteClient,
    pub workdir: TempDir,
}

pub fn setup_server_simple() -> BenchmarkContext {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let (server, workdir) = rt.block_on(async {
        let workdir = tempfile::tempdir().unwrap();
        let mut cfg = default_server_config(workdir.path());
        cfg.log_level = LogLevel::Info;
        (TestServerBuilder::new().build().await, workdir)
    });

    let server_url = Some(format!("http://{}", server.addr));

    let client = coyote_client::CoyoteClient::new(
        server.token.clone(),
        Some(CoyoteOptions {
            debug: false,
            server_url,
            ..Default::default()
        }),
    );

    BenchmarkContext {
        rt,
        server,
        client,
        workdir,
    }
}
