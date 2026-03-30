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
        .post("v1.msgs.namespace.create")
        .json(json!({
            "name": "my-namespace",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "my-namespace");
    // Retention fields are optional; omitting them means no enforcement
    assert!(response["retention"]["period_ms"].is_null());
    assert!(response["retention"]["size_bytes"].is_null());
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
        .post("v1.msgs.namespace.create")
        .json(json!({
            "name": "custom-ns",
            "retention": {
                "size_bytes": 4194304,
                "period_ms": 604800000
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
            "retention": { "size_bytes": 4194304, "period_ms": 604800000 },
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
        .post("v1.msgs.namespace.create")
        .json(json!({
            "name": "upsert-ns",
            "retention": { "size_bytes": 1024, "period_ms": 9999 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let created_ts = first["created"].assert_str().to_owned();
    assert_eq!(first["name"], "upsert-ns");
    assert_eq!(first["retention"]["size_bytes"], 1024);
    assert_eq!(first["retention"]["period_ms"], 9999);

    // Upsert with different retention
    let second = client
        .post("v1.msgs.namespace.create")
        .json(json!({
            "name": "upsert-ns",
            "retention": { "size_bytes": 2048, "period_ms": 60000 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(second["name"], "upsert-ns");
    assert_eq!(second["retention"]["size_bytes"], 2048);
    assert_eq!(second["retention"]["period_ms"], 60000);
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
        .post("v1.msgs.namespace.create")
        .json(json!({
            "name": "get-test-ns",
            "retention": { "size_bytes": 5000, "period_ms": 30000 }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Get it back
    let response = client
        .post("v1.msgs.namespace.get")
        .json(json!({
            "name": "get-test-ns",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["name"], "get-test-ns");
    assert_eq!(response["retention"]["size_bytes"], 5000);
    assert_eq!(response["retention"]["period_ms"], 30000);
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
        .post("v1.msgs.namespace.get")
        .json(json!({
            "name": "nonexistent-ns",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
