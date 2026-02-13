use std::time::Duration;

use serde_json::json;
use test_utils::{
    StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn start(client: &TestClient, key: &str, ttl_seconds: u64) -> TestResult<serde_json::Value> {
    let response = client
        .post("idempotency/start")
        .json(json!({
            "key": key,
            "ttl": ttl_seconds
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

async fn complete(
    client: &TestClient,
    key: &str,
    response: &str,
    ttl_seconds: u64,
) -> TestResult<()> {
    client
        .post("idempotency/complete")
        .json(json!({
            "key": key,
            "response": response.as_bytes(),
            "ttl": ttl_seconds
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

async fn abandon(client: &TestClient, key: &str) -> TestResult<()> {
    client
        .post("idempotency/abort")
        .json(json!({
            "key": key
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_idempotency_start_and_complete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = start(&client, "k1", 60).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k2", 60).await?;

    // start again should return locked
    let response = client
        .post("idempotency/start")
        .json(json!({
            "key": "k2",
            "ttl": 60
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(response["status"], "locked");

    start(&client, "k3", 60).await?;
    complete(&client, "k3", "v1", 60).await?;
    complete(&client, "k3", "v2", 60).await?; // can complete same key again

    let response = start(&client, "k3", 60).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["response"], json!("v2".as_bytes()));

    start(&client, "k4", 60).await?;
    abandon(&client, "k4").await?;
    let response = start(&client, "k4", 60).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k5", 60).await?;
    complete(&client, "k5", "v2", 60).await?;
    let response = start(&client, "k5", 60).await?;
    assert_eq!(response["response"], json!("v2".as_bytes()));

    complete(&client, "k5", "v3", 60).await?;
    let response = start(&client, "k5", 60).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["response"], json!("v3".as_bytes()));

    start(&client, "k6", 60).await?;
    complete(&client, "k6", "v4", 60).await?;

    let response = start(&client, "k6", 60).await?;
    assert_eq!(response["response"], json!("v4".as_bytes()));

    abandon(&client, "k6").await?;

    let response = start(&client, "k6", 60).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k7", 60).await?;
    complete(&client, "k7", "v1", 60).await?;
    // can abandon after completing
    abandon(&client, "k7").await?;
    let response = start(&client, "k7", 60).await?;
    assert_eq!(response["status"], "started");

    Ok(())
}

#[tokio::test]
async fn test_idempotency_abandon() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    start(&client, "k1", 1).await?;
    complete(&client, "k1", "v1", 1).await?;
    // can abandon after completing
    abandon(&client, "k1").await?;
    let response = start(&client, "k1", 1).await?;
    assert_eq!(response["status"], "started");

    // can abandon before starting
    abandon(&client, "k2").await?;

    start(&client, "k3", 1).await?;
    // can abandon before completing
    abandon(&client, "k3").await?;
    let response = start(&client, "k3", 1).await?;
    assert_eq!(response["status"], "started");

    Ok(())
}

#[tokio::test]
async fn test_idempotency_expiration() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // start again after expired
    start(&client, "k1", 1).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = start(&client, "k1", 1).await?;
    assert_eq!(response["status"], "started");

    // start again after expired (completed)
    complete(&client, "k2", "v1", 1).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = start(&client, "k2", 1).await?;
    assert_eq!(response["status"], "started");

    // complete TTL shorter than start
    start(&client, "k4", 60).await?;
    complete(&client, "k4", "v1", 1).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = start(&client, "k4", 60).await?;
    assert_eq!(response["status"], "started");

    // complete TTL longer than start
    start(&client, "k5", 1).await?;
    complete(&client, "k5", "v1", 60).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = start(&client, "k5", 1).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["response"], json!("v1".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn test_idempotency_validation() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("idempotency/start")
        .json(json!({
            "key": "k",
            "ttl": 0
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    client
        .post("idempotency/start")
        .json(json!({
            "key": "k"
            // TTL missing
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}
