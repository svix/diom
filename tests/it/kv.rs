use std::time::Duration;

use jiff::Timestamp;
use serde_json::json;
use test_utils::{
    StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn kv_set(
    client: &TestClient,
    key: &str,
    expire_in: Option<u64>,
    value: &str,
    behavior: &str,
) -> TestResult<()> {
    client
        .post("kv/set")
        .json(json!({
            "key": key,
            "ttl": expire_in,
            "value": value.as_bytes(),
            "behavior": behavior
        }))
        .await?
        .ensure(StatusCode::OK)?;
    Ok(())
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn kv_get(client: &TestClient, key: &str) -> TestResult<serde_json::Value> {
    let response = client
        .post("kv/get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok(response)
}

async fn kv_not_found(client: &TestClient, key: &str) -> TestResult<()> {
    let _ = client
        .post("kv/get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::NOT_FOUND)?
        .json();
    Ok(())
}

#[tokio::test]
async fn test_kv_set_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    kv_set(&client, "kv-key-1", None, "kv-value-456", "upsert").await?;

    let response = kv_get(&client, "kv-key-1").await?;

    assert_eq!(response["key"], "kv-key-1");
    assert_eq!(response["value"], json!("kv-value-456".as_bytes()));
    assert!(response["expiry"].is_null());

    let expires_in = 1000;
    let now = Timestamp::now();
    kv_set(
        &client,
        "kv-key-1",
        Some(expires_in),
        "kv-value-456",
        "upsert",
    )
    .await?;
    let response = kv_get(&client, "kv-key-1").await?;

    let expires_at: Timestamp = serde_json::from_value(response["expiry"].clone()).unwrap();
    let expected = now + Duration::from_millis(expires_in);
    assert!(
        expires_at
            .as_millisecond()
            .abs_diff(expected.as_millisecond())
            < 50 // tolerance
    );

    // set should fail if namespace doesn't exist:
    client
        .post("kv/set")
        .json(json!({
            "key": "nonexistentnamespace:key1",
            "ttl": Some(expires_in),
            "value": "123".as_bytes(),
            "behavior": "upsert",
        }))
        .await?
        .ensure(StatusCode::NOT_FOUND)?;

    Ok(())
}

#[tokio::test]
async fn test_kv_set_with_insert_and_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    kv_set(&client, "kv-key-2", None, "value-ignored", "update").await?;
    kv_not_found(&client, "kv-key-2").await?;

    kv_set(&client, "kv-key-2", None, "permanent-value", "insert").await?;

    // This insert gets ignored
    kv_set(&client, "kv-key-2", None, "value-ignored-2", "insert").await?;

    let response = kv_get(&client, "kv-key-2").await?;
    assert_eq!(response["value"], json!("permanent-value".as_bytes()));
    assert_eq!(response["expiry"], json!(null));

    let delete_response = client
        .post("kv/delete")
        .json(json!({
            "key": "kv-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["deleted"], true);

    kv_not_found(&client, "kv-key-2").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_expiration() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    kv_set(&client, "test-key-3", Some(100), "test-value-345", "upsert").await?;

    tokio::time::sleep(Duration::from_millis(1000)).await;

    kv_not_found(&client, "test-key-3").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_update_expiration() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // Test updating expiration time
    kv_set(&client, "kv4", Some(1500), "v4", "upsert").await?;
    kv_set(&client, "kv4", Some(3000), "v4", "upsert").await?;

    let response = kv_get(&client, "kv4").await?;
    let expires_at = response["expiry"].as_str().unwrap();

    tokio::time::sleep(Duration::from_millis(1500)).await;

    let response = kv_get(&client, "kv4").await?;
    assert_eq!(response["value"], json!("v4".as_bytes()));
    assert_eq!(response["expiry"], expires_at);

    kv_set(&client, "kv5", Some(3000), "v5", "upsert").await?;
    kv_set(&client, "kv5", Some(500), "v5", "upsert").await?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    kv_not_found(&client, "kv5").await?;

    // Test updating expired key
    kv_set(&client, "kv6", Some(1000), "v6", "upsert").await?;
    kv_set(&client, "kv6", Some(500), "v6", "insert").await?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    let response = kv_get(&client, "kv6").await?;
    assert_eq!(response["value"], json!("v6".as_bytes()));

    tokio::time::sleep(Duration::from_millis(500)).await;

    kv_set(&client, "kv6", Some(500), "v6", "update").await?;
    kv_not_found(&client, "kv6").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_binary_data() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let binary_data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    client
        .post("kv/set")
        .json(json!({
            "key": "kv-key-4",
            "value": binary_data,
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::OK);

    let response = kv_get(&client, "kv-key-4").await?;
    assert_eq!(response["value"], json!(binary_data));

    let empty_vec: Vec<u8> = vec![];
    client
        .post("kv/set")
        .json(json!({
            "key": "kv-key-4",
            "value": empty_vec,
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::OK);

    let response = kv_get(&client, "kv-key-4").await?;
    assert_eq!(response["value"], json!([]));

    Ok(())
}

#[tokio::test]
async fn test_kv_validation() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let invalid_keys = ["", "key with spaces", "key@special", &"a".repeat(257)];

    for key in invalid_keys {
        client
            .post("kv/set")
            .json(json!({
                "key": key,
                "value": "test".as_bytes(),
                "behavior": "upsert"
            }))
            .await?
            .expect(StatusCode::UNPROCESSABLE_ENTITY);
    }

    client
        .post("kv/set")
        .json(json!({
            "key": "kv-key-3",
            "value": "test".as_bytes(),
            "ttl": 0,
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    client
        .post("kv/set")
        .json(json!({
            "key": "kv-key-3",
            "value": "test".as_bytes(),
            "ttl": -1,
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn test_kv_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // let response = client
    //     .post("kv/delete")
    //     .json(json!({
    //         "key": "kv-key-5"
    //     }))
    //     .await?
    //     .expect(StatusCode::OK)
    //     .json();

    // FIXME(@svix-lucho): Behavior not implemented yet
    // assert_eq!(response["deleted"], false);

    kv_set(&client, "kv-key-5", None, "value-5", "upsert").await?;

    let response = client
        .post("kv/delete")
        .json(json!({
            "key": "kv-key-5"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(response["deleted"], true);

    kv_not_found(&client, "kv-key-5").await?;

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
        .post("kv/namespace/create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
    assert_eq!(response["storage_type"], "Persistent");
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
        .post("kv/namespace/create")
        .json(json!({
            "name": "custom-ns",
            "storage_type": "Ephemeral",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let ts = &response["created"];

    assert_eq!(
        response,
        json!({
            "name": "custom-ns",
            "storage_type": "Ephemeral",
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
        .post("kv/namespace/create")
        .json(json!({
            "name": "upsert-ns",
            "storage_type": "Ephemeral",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let created_ts = first["created"].as_str().unwrap().to_owned();
    assert_eq!(first["name"], "upsert-ns");
    assert_eq!(first["storage_type"], "Ephemeral");

    // Upsert
    let second = client
        .post("kv/namespace/create")
        .json(json!({
            "name": "upsert-ns",
            "storage_type": "Persistent",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(second["name"], "upsert-ns");
    assert_eq!(second["storage_type"], "Persistent");
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
        .post("kv/namespace/create")
        .json(json!({
            "name": "get-test-ns",
            "storage_type": "Ephemeral",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("kv/namespace/get")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "get-test-ns");
    assert_eq!(response["storage_type"], "Ephemeral");
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
        .post("kv/namespace/get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}
