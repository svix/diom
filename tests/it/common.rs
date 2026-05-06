use anyhow::Context;
use serde_json::json;
use tap::Pipe;
use test_utils::{
    JsonFastAndLoose, StatusCode, TestClient, TestResponse, TestResult,
    retry::run_with_retries,
    server::{TestContext, start_cluster, start_server},
};

async fn kv_set(
    client: &TestClient,
    key: &str,
    value: &str,
    behavior: &str,
) -> anyhow::Result<TestResponse> {
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
async fn kv_get(client: &TestClient, key: &str) -> anyhow::Result<TestResponse> {
    client
        .post("v1.kv.get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)
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
    let context = start_cluster(2).await;

    test_mutation_header(context.leader_client().await).await?;
    test_mutation_header(context.follower_client().await).await?;

    Ok(())
}

#[tokio::test]
async fn test_replication_end_to_end() -> TestResult {
    let context = start_cluster(3).await;

    let key = format!("key{}", uuid::Uuid::new_v4().simple());
    let value = "leader-value";
    let leader_client = context.leader_client().await;
    let follower_client = context.follower_client().await;

    async fn ensure_visible(
        client: &TestClient,
        key: &str,
        expected_value: &str,
    ) -> anyhow::Result<()> {
        let found_value = kv_get(client, key)
            .await?
            .json()
            .get("value")
            .context("no val in response")?
            .assert_bytes()
            .pipe(String::from_utf8)
            .unwrap();
        anyhow::ensure!(found_value == expected_value);
        Ok(())
    }

    // insert a key on the leader
    kv_set(leader_client, &key, value, "upsert").await?;
    // it should immediately be visible on the leader
    ensure_visible(leader_client, &key, value).await?;
    // it should eventually be visible on a follower
    run_with_retries(async || ensure_visible(follower_client, &key, value).await).await?;

    // insert it from the follower (which forwards it)
    let value = "follower-value";
    kv_set(follower_client, &key, value, "upsert").await?;
    // it should immediately be visible on the leader
    ensure_visible(leader_client, &key, value).await?;
    // it should eventually be visible on a follower
    run_with_retries(async || ensure_visible(follower_client, &key, value).await).await?;

    Ok(())
}
