use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn create_namespace_with_defaults() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = client
        .post("msgs/namespace/create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
    // Default retention: 30 days in millis, 1TB in bytes
    assert_eq!(response["retention"]["ms"], 2_592_000_000u64);
    assert_eq!(response["retention"]["bytes"], 1_000_000_000_000u64);
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
        .post("msgs/namespace/create")
        .json(json!({
            "name": "custom-ns",
            "retention": {
                "bytes": 4194304,
                "ms": 604800000
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let ts = &response["created"];

    assert_eq!(
        response,
        json!({
            "name": "custom-ns",
            "retention": { "bytes": 4194304, "ms": 604800000 },
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
        .post("msgs/namespace/create")
        .json(json!({
            "name": "upsert-ns",
            "retention": { "bytes": 1024, "ms": 9999 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let created_ts = first["created"].assert_str().to_owned();
    assert_eq!(first["name"], "upsert-ns");
    assert_eq!(first["retention"]["bytes"], 1024);
    assert_eq!(first["retention"]["ms"], 9999);

    // Upsert with different retention
    let second = client
        .post("msgs/namespace/create")
        .json(json!({
            "name": "upsert-ns",
            "retention": { "bytes": 2048, "ms": 60000 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(second["name"], "upsert-ns");
    assert_eq!(second["retention"]["bytes"], 2048);
    assert_eq!(second["retention"]["ms"], 60000);
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
        .post("msgs/namespace/create")
        .json(json!({
            "name": "get-test-ns",
            "retention": { "bytes": 5000, "ms": 30000 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("msgs/namespace/get")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "get-test-ns");
    assert_eq!(response["retention"]["bytes"], 5000);
    assert_eq!(response["retention"]["ms"], 30000);
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
        .post("msgs/namespace/get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
