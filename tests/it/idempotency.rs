use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn start(client: &TestClient, key: &str, ttl_ms: u64) -> TestResult<serde_json::Value> {
    let response = client
        .post("v1.idempotency.start")
        .json(json!({
            "key": key,
            "lock_period_ms": ttl_ms
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

async fn complete(client: &TestClient, key: &str, response: &str, ttl_ms: u64) -> TestResult<()> {
    client
        .post("v1.idempotency.complete")
        .json(json!({
            "key": key,
            "response": response.as_bytes(),
            "ttl_ms": ttl_ms
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

async fn abandon(client: &TestClient, key: &str) -> TestResult<()> {
    client
        .post("v1.idempotency.abort")
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

    let response = start(&client, "k1", 60_000).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k2", 60_000).await?;

    // start again should return locked
    let response = client
        .post("v1.idempotency.start")
        .json(json!({
            "key": "k2",
            "lock_period_ms": 60_000
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(response["status"], "locked");

    start(&client, "k3", 60_000).await?;
    complete(&client, "k3", "v1", 60_000).await?;
    complete(&client, "k3", "v2", 60_000).await?; // can complete same key again

    let response = start(&client, "k3", 60_000).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["data"]["response"], json!("v2".as_bytes()));

    start(&client, "k4", 60_000).await?;
    abandon(&client, "k4").await?;
    let response = start(&client, "k4", 60_000).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k5", 60_000).await?;
    complete(&client, "k5", "v2", 60_000).await?;
    let response = start(&client, "k5", 60_000).await?;
    assert_eq!(response["data"]["response"], json!("v2".as_bytes()));

    complete(&client, "k5", "v3", 60_000).await?;
    let response = start(&client, "k5", 60_000).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["data"]["response"], json!("v3".as_bytes()));

    start(&client, "k6", 60_000).await?;
    complete(&client, "k6", "v4", 60_000).await?;

    let response = start(&client, "k6", 60_000).await?;
    assert_eq!(response["data"]["response"], json!("v4".as_bytes()));

    abandon(&client, "k6").await?;

    let response = start(&client, "k6", 60_000).await?;
    assert_eq!(response["status"], "started");

    start(&client, "k7", 60_000).await?;
    complete(&client, "k7", "v1", 60_000).await?;
    // can abandon after completing
    abandon(&client, "k7").await?;
    let response = start(&client, "k7", 60_000).await?;
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

    start(&client, "k1", 1_000).await?;
    complete(&client, "k1", "v1", 1_000).await?;
    // can abandon after completing
    abandon(&client, "k1").await?;
    let response = start(&client, "k1", 1_000).await?;
    assert_eq!(response["status"], "started");

    // can abandon before starting
    abandon(&client, "k2").await?;

    start(&client, "k3", 1_000).await?;
    // can abandon before completing
    abandon(&client, "k3").await?;
    let response = start(&client, "k3", 1_000).await?;
    assert_eq!(response["status"], "started");

    Ok(())
}

#[tokio::test]
async fn test_idempotency_expiration() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    // start again after expired
    start(&client, "k1", 1_000).await?;
    time.fast_forward(Duration::from_secs(2));
    let response = start(&client, "k1", 1_000).await?;
    assert_eq!(response["status"], "started");

    // start again after expired (completed)
    complete(&client, "k2", "v1", 1_000).await?;
    time.fast_forward(Duration::from_secs(2));
    let response = start(&client, "k2", 1_000).await?;
    assert_eq!(response["status"], "started");

    // complete TTL shorter than start
    start(&client, "k4", 60_000).await?;
    complete(&client, "k4", "v1", 1_000).await?;
    time.fast_forward(Duration::from_secs(2));
    let response = start(&client, "k4", 60_000).await?;
    assert_eq!(response["status"], "started");

    // complete TTL longer than start
    start(&client, "k5", 1_000).await?;
    complete(&client, "k5", "v1", 60_000).await?;
    time.fast_forward(Duration::from_secs(1));
    let response = start(&client, "k5", 1_000).await?;
    assert_eq!(response["status"], "completed");
    assert_eq!(response["data"]["response"], json!("v1".as_bytes()));

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
        .post("v1.idempotency.start")
        .json(json!({
            "key": "k",
            "lock_period_ms": 0
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    client
        .post("v1.idempotency.start")
        .json(json!({
            "key": "k"
            // TTL missing
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn create_namespace_with_defaults() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = client
        .post("v1.idempotency.namespace.create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
    assert!(response["created"].is_u64());
    assert!(response["updated"].is_u64());

    Ok(())
}

#[tokio::test]
async fn create_namespace_with_custom_config() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = client
        .post("v1.idempotency.namespace.create")
        .json(json!({
            "name": "custom-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let ts = &response["created"];

    assert_eq!(
        response,
        json!({
            "name": "custom-ns",
            "created": ts,
            "updated": ts,
        })
    );

    Ok(())
}

#[tokio::test]
async fn create_namespace_upserts() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let first = client
        .post("v1.idempotency.namespace.create")
        .json(json!({
            "name": "upsert-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let created_ts = first["created"].assert_u64();
    assert_eq!(first["name"], "upsert-ns");

    // Upsert
    let second = client
        .post("v1.idempotency.namespace.create")
        .json(json!({
            "name": "upsert-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(second["name"], "upsert-ns");
    // created timestamp should remain the same
    assert_eq!(second["created"], created_ts);
    // updated timestamp should change
    assert_ne!(second["updated"], first["updated"]);

    Ok(())
}

#[tokio::test]
async fn get_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // Create a namespace first
    let created = client
        .post("v1.idempotency.namespace.create")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("v1.idempotency.namespace.get")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "get-test-ns");
    assert_eq!(response["created"], created["created"]);
    assert_eq!(response["updated"], created["updated"]);

    Ok(())
}

#[tokio::test]
async fn get_namespace_not_found() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.idempotency.namespace.get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
