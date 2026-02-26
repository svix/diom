#![allow(unused)]

use diom::{cfg::LogLevel, core::cluster::proto::HealthResponse};
use diom_client::{DiomClient, DiomOptions};
use openraft::ServerState;
use std::sync::{Arc, Once};
use tempfile::TempDir;
use test_utils::TestClient;
use tokio::runtime::Runtime;

use test_utils::server::{TestContext, TestServerBuilder, default_server_config};

pub struct BenchmarkContext {
    pub rt: Runtime,
    pub servers: Arc<Vec<TestContext>>,
    pub client: DiomClient,
    pub test_client: TestClient,
    pub workdirs: Vec<TempDir>,
}

pub fn setup_single_server() -> BenchmarkContext {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let (server, workdir) = rt.block_on(async {
        let workdir = tempfile::tempdir().unwrap();
        let mut cfg = default_server_config(workdir.path());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        (TestServerBuilder::new().build().await, workdir)
    });

    let server_url = Some(format!("http://{}", server.addr));

    let client = DiomClient::new(
        server.token.clone(),
        Some(DiomOptions {
            debug: false,
            server_url,
            ..Default::default()
        }),
    );

    let servers = Arc::new(vec![server]);
    let test_client = servers[0].client.clone();

    BenchmarkContext {
        rt,
        servers,
        client,
        test_client,
        workdirs: vec![workdir],
    }
}

async fn make_sure_cluster_sane_and_get_client(
    servers: Arc<Vec<TestContext>>,
) -> anyhow::Result<(DiomClient, TestClient)> {
    let expected_followers = servers.len() - 1;
    let mut followers = 0;
    let mut server_url = None;
    let mut leader_test_client = None;

    let client = reqwest::Client::new();
    for server in servers.as_ref() {
        let resp = client
            .get(format!("http://{}/repl/health", server.repl_addr))
            .send()
            .await?;
        if !resp.status().is_success() {
            anyhow::bail!(
                "Failed getting node health status {}",
                resp.status().as_str()
            );
        }
        let resp = resp.json::<HealthResponse>().await?;
        if resp.server_state.is_leader() {
            server_url = Some(format!("http://{}", server.addr));
            leader_test_client = Some(server.client.clone());
        } else if resp.server_state.is_follower() {
            followers += 1;
        }
    }
    if followers != expected_followers {
        anyhow::bail!("Expected {expected_followers} servers, got {followers}");
    }

    let client = DiomClient::new(
        servers[0].token.clone(),
        Some(DiomOptions {
            debug: false,
            server_url,
            ..Default::default()
        }),
    );

    let test_client =
        leader_test_client.ok_or_else(|| anyhow::anyhow!("No leader found in cluster"))?;

    Ok((client, test_client))
}

pub fn setup_cluster(num_servers: usize) -> BenchmarkContext {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut seed_nodes = vec![];
    let mut servers: Vec<TestContext> = vec![];
    let mut workdirs = vec![];
    for i in 0..num_servers {
        let (server, workdir) = rt.block_on(async {
            let workdir = tempfile::tempdir().unwrap();
            let mut cfg = default_server_config(workdir.path());
            cfg.cluster.seed_nodes = seed_nodes.clone();
            cfg.cluster.auto_initialize = true;
            let server = TestServerBuilder::new().cfg(cfg).build().await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            (server, workdir)
        });

        seed_nodes.push(server.repl_addr.into());
        servers.push(server);
        workdirs.push(workdir);
    }

    let servers = Arc::new(servers);

    let mut retries = 10;
    let (client, test_client) = loop {
        if retries == 0 {
            panic!("Failed to set up cluster");
        }
        retries -= 1;
        match rt.block_on({
            let servers = servers.clone();
            async { make_sure_cluster_sane_and_get_client(servers).await }
        }) {
            Ok(result) => break result,
            Err(e) => {
                println!("Error during cluster setup: {e}");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    };

    BenchmarkContext {
        rt,
        servers,
        client,
        test_client,
        workdirs,
    }
}
