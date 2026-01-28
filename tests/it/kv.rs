use std::time::Duration;

use serde_json::{Value, json};
use test_utils::{StatusCode, TestClient, TestResult};

async fn kv_set(
    client: &TestClient,
    key: &str,
    expire_in: u64,
    value: &str,
    behavior: &str,
) -> TestResult<()> {
    client
        .post("kv/set")
        .json(json!({
            "key": key,
            "expire_in": expire_in,
            "value": value,
            "behavior": behavior
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

async fn kv_get(client: &TestClient, key: &str) -> TestResult<Value> {
    let response = client
        .post("kv/get")
        .json(json!({
            "key": key
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

#[tokio::test]
async fn test_kv_set_and_get() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    kv_set(&client, "kv-key-1", 50000, "kv-value-456", "upsert").await?;

    let response = kv_get(&client, "kv-key-1").await?;

    assert_eq!(response["key"], "kv-key-1");
    assert_eq!(response["value"], "kv-value-456");
    assert!(response["expires_at"].is_string());

    Ok(())
}

#[tokio::test]
async fn test_kv_set_with_insert_and_delete() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    kv_set(&client, "kv-key-2", 0, "permanent-value", "insert").await?;

    let response = kv_get(&client, "kv-key-2").await?;
    assert_eq!(response["value"], "permanent-value");
    assert_eq!(response["expires_at"], json!(null));

    let delete_response = client
        .post("kv/delete")
        .json(json!({
            "key": "kv-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["deleted"], true);

    client
        .post("kv/get")
        .json(json!({
            "key": "kv-key-2"
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn test_kv_expiration() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    kv_set(&client, "test-key-3", 100, "test-value-345", "upsert").await?;

    tokio::time::sleep(Duration::from_millis(1000)).await;

    client
        .post("kv/get")
        .json(json!({
            "key": "test-key-3"
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}
