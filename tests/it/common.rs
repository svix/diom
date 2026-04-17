use diom_backend::cfg::PeerAddr;
use serde_json::json;
use test_utils::{
    JsonFastAndLoose, StatusCode, TestClient, TestResponse, TestResult,
    server::{TestContext, TestServerBuilder, start_server},
};

async fn kv_set(
    client: &TestClient,
    key: &str,
    value: &str,
    behavior: &str,
) -> TestResult<TestResponse> {
    client
        .post("v1.kv.set")
        .json(json!({
            "key": key,
            "value": value.as_bytes(),
            "behavior": behavior
        }))
        .await
        .map_err(Into::into)
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn kv_get(client: &TestClient, key: &str) -> TestResult<TestResponse> {
    client
        .post("v1.kv.get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)
        .map_err(Into::into)
}

async fn test_mutation_header(client: &TestClient) -> TestResult {
    let response = kv_set(client, "foo", "var", "upsert").await?;
    let value = response
        .headers()
        .get("Diom-Mutation-Version")
        .expect("header should be set");
    let initial_index: u64 = value
        .to_str()
        .expect("value should start out a a str")
        .parse()
        .expect("value should be a u64");
    tracing::debug!("initial index is {initial_index}");

    let response = kv_set(client, "foo", "bar", "upsert").await?;
    let new_index: u64 = response
        .headers()
        .get("Diom-Mutation-Version")
        .expect("header should still be set on second request")
        .to_str()
        .expect("value should start out a str")
        .parse()
        .expect("value should be a u64");
    tracing::debug!("second request index is {new_index}");
    assert!(new_index > initial_index);

    let response = kv_get(client, "foo").await?;
    assert!(
        response.headers().get("Diom-Mutation-Version").is_none(),
        "Diom-Mutation-Version should not be set on get requests"
    );

    Ok(())
}

#[tokio::test]
async fn test_diom_mutation_header_single_node() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;
    test_mutation_header(&client).await
}

#[tokio::test]
async fn test_diom_mutation_header_cluster() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        repl_addr,
        ..
    } = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
        })
        .build()
        .await;

    let TestContext {
        client: second_client,
        handle: _second_handle,
        ..
    } = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.cluster.seed_nodes = vec![PeerAddr::from(repl_addr)];
            cfg.cluster.auto_initialize = false;
        })
        .build()
        .await;
    let cluster_status = client
        .get("v1.cluster-admin.status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cluster_status["nodes"].assert_array().len(), 2);

    let (leader_client, follower_client) = if cluster_status["this_node_state"] == "leader" {
        (&client, &second_client)
    } else {
        (&second_client, &client)
    };

    test_mutation_header(leader_client).await?;
    test_mutation_header(follower_client).await?;

    Ok(())
}
