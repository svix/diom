use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn cache_set(client: &TestClient, key: &str, expire_in: u64, value: &str) -> TestResult<()> {
    client
        .post("cache/set")
        .json(json!({
            "key": key,
            "ttl": expire_in,
            "value": value.as_bytes()
        }))
        .await?
        .expect(StatusCode::OK);
    Ok(())
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn cache_get(client: &TestClient, key: &str) -> TestResult<serde_json::Value> {
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

    assert_eq!(response["value"], json!("test-value-123".as_bytes()));
    assert!(response["expiry"].is_string());

    // set should fail if namespace doesn't exist:
    client
        .post("cache/set")
        .json(json!({
            "namespace": "nonexistentnamespace",
            "key": "key1",
            "ttl": 1,
            "value": "123".as_bytes(),
            "behavior": "upsert",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_cache_set_get_and_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let delete_response = client
        .post("cache/delete")
        .json(json!({
            "key": "test-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["success"], false);

    cache_set(&client, "test-key-2", 30000, "another-value").await?;

    let response = cache_get(&client, "test-key-2").await?;
    assert_eq!(response["value"], json!("another-value".as_bytes()));

    let delete_response = client
        .post("cache/delete")
        .json(json!({
            "key": "test-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["success"], true);

    let response = client
        .post("cache/get")
        .json(json!({
            "key": "test-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert!(response["value"].is_null());

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
        .post("cache/namespace/create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
    assert_eq!(response["eviction_policy"], "NoEviction");
    assert!(response["created"].is_string());
    assert!(response["updated"].is_string());

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
        .post("cache/namespace/create")
        .json(json!({
            "name": "custom-ns",
            "eviction_policy": "LeastRecentlyUsed",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let ts = &response["created"];

    assert_eq!(
        response,
        json!({
            "name": "custom-ns",
            "eviction_policy": "LeastRecentlyUsed",
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
        .post("cache/namespace/create")
        .json(json!({
            "name": "upsert-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let created_ts = first["created"].assert_str().to_owned();
    assert_eq!(first["name"], "upsert-ns");

    // Upsert
    let second = client
        .post("cache/namespace/create")
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
        .post("cache/namespace/create")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("cache/namespace/get")
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
        .post("cache/namespace/get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
