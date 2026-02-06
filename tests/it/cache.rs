use serde_json::{Value, json};
use test_utils::{
    StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn cache_set(client: &TestClient, key: &str, expire_in: u64, value: &str) -> TestResult<()> {
    client
        .post("cache/set")
        .json(json!({
            "key": key,
            "expire_in": expire_in,
            "value": value
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

async fn cache_get(client: &TestClient, key: &str) -> TestResult<Value> {
    let response = client
        .post("cache/get")
        .json(json!({
            "key": key
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

#[tokio::test]
async fn test_cache_set_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    cache_set(&client, "test-key-1", 60000, "test-value-123").await?;

    let response = cache_get(&client, "test-key-1").await?;

    assert_eq!(response["key"], "test-key-1");
    assert_eq!(response["value"], "test-value-123");
    assert!(response["expires_at"].is_string());

    Ok(())
}

#[tokio::test]
async fn test_cache_set_get_and_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    cache_set(&client, "test-key-2", 30000, "another-value").await?;

    let response = cache_get(&client, "test-key-2").await?;
    assert_eq!(response["value"], "another-value");

    let delete_response = client
        .post("cache/delete")
        .json(json!({
            "key": "test-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["deleted"], true);

    client
        .post("cache/get")
        .json(json!({
            "key": "test-key-2"
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}
