use serde_json::json;
use test_utils::{StatusCode, TestResult, server::start_server};

const TEST_AUTOCONFIG_TOKEN: &str = "auto_v1_eyJhaWQiOiJhcHBfdGVzdCIsImVpZCI6ImVwX3Rlc3QiLCJzdXJsIjoiaHR0cHM6Ly9hcGkuZXhhbXBsZS50ZXN0IiwiZXNlYyI6Indoc2VjX2RHVnpkQT09IiwidG9rIjoic2tfdGVzdF94eXoifQ==";

#[tokio::test]
async fn test_svix_poller_create() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.svix-poller.create")
        .json(json!({
            "topic": "webhooks",
            "poller_id": "poller_123",
            "token": TEST_AUTOCONFIG_TOKEN,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "webhooks");
    assert_eq!(response["poller_id"], "poller_123");

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_create_invalid_token() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.svix-poller.create")
        .json(json!({
            "topic": "webhooks",
            "poller_id": "poller_x",
            "token": "not_a_valid_token",
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_create_namespace_not_found() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.svix-poller.create")
        .json(json!({
            "namespace": "nonexistent",
            "topic": "webhooks",
            "poller_id": "poller_x",
            "token": TEST_AUTOCONFIG_TOKEN,
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_create_and_delete() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.svix-poller.create")
        .json(json!({
            "topic": "webhooks",
            "poller_id": "poller_del",
            "token": TEST_AUTOCONFIG_TOKEN,
        }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.svix-poller.delete")
        .json(json!({
            "topic": "webhooks",
            "poller_id": "poller_del",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "webhooks");
    assert_eq!(response["poller_id"], "poller_del");
    assert_eq!(response["success"], true);

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_delete_is_idempotent() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.svix-poller.delete")
        .json(json!({
            "topic": "webhooks",
            "poller_id": "nonexistent_poller",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["success"], false);

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_list() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    for poller_id in ["poller_a", "poller_b", "poller_c"] {
        ctx.client
            .post("v1.msgs.svix-poller.create")
            .json(json!({
                "topic": "webhooks",
                "poller_id": poller_id,
                "token": TEST_AUTOCONFIG_TOKEN,
            }))
            .await?
            .expect(StatusCode::OK);
    }

    let response = ctx
        .client
        .post("v1.msgs.svix-poller.list")
        .json(json!({ "topic": "webhooks" }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = response["data"].as_array().expect("data array");
    assert_eq!(data.len(), 3);
    // Ordered by poller id.
    assert_eq!(data[0]["poller_id"], "poller_a");
    assert_eq!(data[2]["poller_id"], "poller_c");
    assert_eq!(data[0]["topic"], "webhooks");
    // Token is obfuscated, not returned in full.
    let token = data[0]["token"].as_str().expect("token string");
    assert!(token.contains("..."));
    assert_ne!(token, TEST_AUTOCONFIG_TOKEN);
    assert_eq!(response["done"], true);

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_list_paginates() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    for poller_id in ["poller_a", "poller_b", "poller_c"] {
        ctx.client
            .post("v1.msgs.svix-poller.create")
            .json(json!({
                "topic": "webhooks",
                "poller_id": poller_id,
                "token": TEST_AUTOCONFIG_TOKEN,
            }))
            .await?
            .expect(StatusCode::OK);
    }

    let page = ctx
        .client
        .post("v1.msgs.svix-poller.list")
        .json(json!({ "topic": "webhooks", "limit": 2 }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = page["data"].as_array().expect("data array");
    assert_eq!(data.len(), 2);
    assert_eq!(data[0]["poller_id"], "poller_a");
    assert_eq!(data[1]["poller_id"], "poller_b");
    assert_eq!(page["done"], false);

    let iterator = page["iterator"].as_str().expect("iterator cursor");
    let next = ctx
        .client
        .post("v1.msgs.svix-poller.list")
        .json(json!({ "topic": "webhooks", "limit": 2, "iterator": iterator }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = next["data"].as_array().expect("data array");
    assert_eq!(data.len(), 1);
    assert_eq!(data[0]["poller_id"], "poller_c");
    assert_eq!(next["done"], true);

    Ok(())
}

#[tokio::test]
async fn test_svix_poller_list_unknown_topic_is_empty() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.svix-poller.list")
        .json(json!({ "topic": "webhooks" }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["data"].as_array().expect("data array").len(), 0);
    assert_eq!(response["done"], true);

    Ok(())
}
