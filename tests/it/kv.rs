use std::time::Duration;

use jiff::Timestamp;
use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn kv_set(
    client: &TestClient,
    key: &str,
    expire_in: Option<u64>,
    value: &str,
    behavior: &str,
) -> anyhow::Result<()> {
    let response = client
        .post("v1.kv.set")
        .json(json!({
            "key": key,
            "ttl_ms": expire_in,
            "value": value.as_bytes(),
            "behavior": behavior
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    anyhow::ensure!(response["success"] == true, "set should succeed");
    Ok(())
}

async fn kv_set_unsuccessful(
    client: &TestClient,
    key: &str,
    expire_in: Option<u64>,
    value: &str,
    behavior: &str,
) -> anyhow::Result<()> {
    let response = client
        .post("v1.kv.set")
        .json(json!({
            "key": key,
            "ttl_ms": expire_in,
            "value": value.as_bytes(),
            "behavior": behavior
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    anyhow::ensure!(response["success"] == false, "set should fail");
    Ok(())
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn kv_get(client: &TestClient, key: &str) -> TestResult<serde_json::Value> {
    let response = client
        .post("v1.kv.get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok(response)
}

async fn kv_not_found(client: &TestClient, key: &str) -> anyhow::Result<()> {
    let response = client
        .post("v1.kv.get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    anyhow::ensure!(response["value"].is_null(), "key should be not-found");
    Ok(())
}

#[tokio::test]
async fn test_kv_set_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    kv_set(&client, "kv-key-1", None, "kv-value-456", "upsert").await?;

    let response = kv_get(&client, "kv-key-1").await?;

    assert_eq!(response["value"], json!("kv-value-456".as_bytes()));
    assert!(response["expiry"].is_null());

    let expires_in = 1000;
    let now = time.now();
    kv_set(
        &client,
        "kv-key-1",
        Some(expires_in),
        "kv-value-456",
        "upsert",
    )
    .await?;
    let response = kv_get(&client, "kv-key-1").await?;

    let expires_at: Timestamp = serde_json::from_value(response["expiry"].clone())?;
    let expected = now + Duration::from_millis(expires_in);
    assert!(
        expires_at
            .as_millisecond()
            .abs_diff(expected.as_millisecond())
            < 50 // tolerance
    );

    // set should fail if namespace doesn't exist:
    client
        .post("v1.kv.set")
        .json(json!({
            "namespace": "nonexistentnamespace",
            "key": "key1",
            "ttl_ms": Some(expires_in),
            "value": "123".as_bytes(),
            "behavior": "upsert",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_kv_set_with_insert_and_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    kv_set_unsuccessful(&client, "kv-key-2", None, "value-ignored", "update").await?;
    kv_not_found(&client, "kv-key-2").await?;

    kv_set(&client, "kv-key-2", None, "permanent-value", "insert").await?;

    // This insert gets ignored
    kv_set_unsuccessful(&client, "kv-key-2", None, "value-ignored-2", "insert").await?;

    let response = kv_get(&client, "kv-key-2").await?;
    assert_eq!(response["value"], json!("permanent-value".as_bytes()));
    assert_eq!(response["expiry"], json!(null));

    let delete_response = client
        .post("v1.kv.delete")
        .json(json!({
            "key": "kv-key-2"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(delete_response["success"], true);

    kv_not_found(&client, "kv-key-2").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_expiration() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    kv_set(&client, "test-key-3", Some(100), "test-value-345", "upsert").await?;

    time.fast_forward(Duration::from_secs(1));

    kv_not_found(&client, "test-key-3").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_update_expiration() -> TestResult {
    let TestContext {
        client,
        time,
        handle: _handle,
        ..
    } = start_server().await;

    // Test updating expiration time
    kv_set(&client, "kv4", Some(1500), "v4", "upsert").await?;
    let response = kv_get(&client, "kv4").await?;
    let initial_expires_at = response["expiry"].assert_str();

    kv_set(&client, "kv4", Some(3000), "v4", "upsert").await?;

    let response = kv_get(&client, "kv4").await?;
    let expires_at = response["expiry"].assert_str();
    assert_ne!(initial_expires_at, expires_at);

    time.fast_forward(Duration::from_millis(1500));

    let response = kv_get(&client, "kv4").await?;
    assert_eq!(response["value"], json!("v4".as_bytes()));
    assert_eq!(response["expiry"], expires_at);

    kv_set(&client, "kv5", Some(3000), "v5", "upsert").await?;
    kv_set(&client, "kv5", Some(500), "v5", "upsert").await?;

    time.fast_forward(Duration::from_millis(501));

    kv_not_found(&client, "kv5").await?;

    // Test updating expired key
    kv_set(&client, "kv6", Some(1000), "v6", "upsert").await?;
    kv_set_unsuccessful(&client, "kv6", Some(500), "v6", "insert").await?;

    time.fast_forward(Duration::from_millis(500));

    let response = kv_get(&client, "kv6").await?;
    assert_eq!(response["value"], json!("v6".as_bytes()));

    time.fast_forward(Duration::from_millis(501));

    kv_set_unsuccessful(&client, "kv6", Some(500), "v6", "update").await?;
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
        .post("v1.kv.set")
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
        .post("v1.kv.set")
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
            .post("v1.kv.set")
            .json(json!({
                "key": key,
                "value": "test".as_bytes(),
                "behavior": "upsert"
            }))
            .await?
            .expect(StatusCode::UNPROCESSABLE_ENTITY);
    }

    client
        .post("v1.kv.set")
        .json(json!({
            "key": "kv-key-3",
            "value": "test".as_bytes(),
            "ttl_ms": 0,
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    client
        .post("v1.kv.set")
        .json(json!({
            "key": "kv-key-3",
            "value": "test".as_bytes(),
            "ttl_ms": -1,
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

    let response = client
        .post("v1.kv.delete")
        .json(json!({
            "key": "kv-key-5"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["success"], false);

    kv_set(&client, "kv-key-5", None, "value-5", "upsert").await?;

    let response = client
        .post("v1.kv.delete")
        .json(json!({
            "key": "kv-key-5"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(response["success"], true);

    kv_not_found(&client, "kv-key-5").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_delete_with_version() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // Set a key and capture the version from the set response
    let set_resp = client
        .post("v1.kv.set")
        .json(json!({
            "key": "occ-del-key",
            "value": "v1".as_bytes(),
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    let version = set_resp["version"].as_u64().unwrap();

    // Delete with a wrong version → version_mismatch error
    let err = client
        .post("v1.kv.delete")
        .json(json!({ "key": "occ-del-key", "version": version + 1 }))
        .await?
        .ensure(StatusCode::BAD_REQUEST)?
        .json();
    assert_eq!(err["code"], "version_mismatch");

    // Key should still be intact
    let get_resp = kv_get(&client, "occ-del-key").await?;
    assert_eq!(get_resp["value"], json!("v1".as_bytes()));

    // Delete with the correct version → success
    let del_resp = client
        .post("v1.kv.delete")
        .json(json!({ "key": "occ-del-key", "version": version }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(del_resp["success"], true);

    kv_not_found(&client, "occ-del-key").await?;

    Ok(())
}

#[tokio::test]
async fn test_kv_delete_with_stale_version() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // Set a key then update it — version advances
    let first = client
        .post("v1.kv.set")
        .json(json!({
            "key": "occ-del-stale",
            "value": "v1".as_bytes(),
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    let v1 = first["version"].as_u64().unwrap();

    client
        .post("v1.kv.set")
        .json(json!({
            "key": "occ-del-stale",
            "value": "v2".as_bytes(),
            "behavior": "upsert"
        }))
        .await?
        .expect(StatusCode::OK);

    // Attempting to delete using the old version should fail
    let err = client
        .post("v1.kv.delete")
        .json(json!({ "key": "occ-del-stale", "version": v1 }))
        .await?
        .ensure(StatusCode::BAD_REQUEST)?
        .json();
    assert_eq!(err["code"], "version_mismatch");

    // Key should still hold the updated value
    let get_resp = kv_get(&client, "occ-del-stale").await?;
    assert_eq!(get_resp["value"], json!("v2".as_bytes()));

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
        .post("v1.kv.namespace.create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
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
        .post("v1.kv.namespace.create")
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
        .post("v1.kv.namespace.create")
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
        .post("v1.kv.namespace.create")
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
        .post("v1.kv.namespace.create")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("v1.kv.namespace.get")
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
        .post("v1.kv.namespace.get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
