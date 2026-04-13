use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn create_admin_token(
    client: &TestClient,
    name: &str,
    role: &str,
) -> TestResult<(String, String)> {
    let resp = client
        .post("v1.admin.auth-token.create")
        .json(json!({
            "name": name,
            "role": role,
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok((
        resp["id"].assert_str().to_owned(),
        resp["token"].assert_str().to_owned(),
    ))
}

#[tokio::test]
async fn test_admin_auth_token_whoami() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-token.whoami")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["role"], "admin");

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_whoami_role() -> TestResult {
    let TestContext {
        mut client,
        handle: _handle,
        ..
    } = start_server().await;

    let (_id, token) = create_admin_token(&client, "reader-token", "reader").await?;

    client.set_auth_header(token);

    let resp = client
        .post("v1.admin.auth-token.whoami")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["role"], "reader");

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_create_and_list() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, token) = create_admin_token(&client, "my-admin-token", "reader").await?;
    assert!(!id.is_empty());
    assert!(!token.is_empty());

    let resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = resp["data"].assert_array();
    let found = tokens.iter().find(|t| t["id"] == id);
    assert!(found.is_some(), "created token should appear in list");

    let t = found.unwrap();
    assert_eq!(t["name"], "my-admin-token");
    assert_eq!(t["role"], "reader");
    assert_eq!(t["enabled"], true);

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_create_disabled() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-token.create")
        .json(json!({
            "name": "disabled-token",
            "role": "reader",
            "enabled": false,
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let id = resp["id"].assert_str().to_owned();

    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = list_resp["data"].assert_array();
    let found = tokens.iter().find(|t| t["id"] == id).unwrap();
    assert_eq!(found["enabled"], false);

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, _token) = create_admin_token(&client, "to-delete", "writer").await?;

    let resp = client
        .post("v1.admin.auth-token.delete")
        .json(json!({ "id": id }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["success"], true);

    // Confirm it no longer appears in the list
    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = list_resp["data"].assert_array();
    assert!(
        tokens.iter().all(|t| t["id"] != id),
        "deleted token should not appear in list"
    );

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_expire_immediately() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, _token) = create_admin_token(&client, "expire-now", "reader").await?;

    client
        .post("v1.admin.auth-token.expire")
        .json(json!({ "id": id }))
        .await?
        .ensure(StatusCode::OK)?;

    // Confirm token is still in list (expire doesn't delete it)
    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = list_resp["data"].assert_array();
    let found = tokens.iter().find(|t| t["id"] == id);
    assert!(found.is_some(), "expired token should still appear in list");

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_expire_in_future() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    let (id, _token) = create_admin_token(&client, "expire-future", "reader").await?;

    client
        .post("v1.admin.auth-token.expire")
        .json(json!({ "id": id, "expiry_millis": 500u64 }))
        .await?
        .ensure(StatusCode::OK)?;

    // Token still in list before expiry
    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert!(
        list_resp["data"]
            .assert_array()
            .iter()
            .any(|t| t["id"] == id)
    );

    time.fast_forward(Duration::from_secs(1));

    // Token still in list after expiry (expire doesn't remove it)
    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert!(
        list_resp["data"]
            .assert_array()
            .iter()
            .any(|t| t["id"] == id)
    );

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_update() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, _token) = create_admin_token(&client, "original-name", "reader").await?;

    client
        .post("v1.admin.auth-token.update")
        .json(json!({
            "id": id,
            "name": "updated-name",
            "enabled": false,
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = list_resp["data"].assert_array();
    let found = tokens.iter().find(|t| t["id"] == id).unwrap();
    assert_eq!(found["name"], "updated-name");
    assert_eq!(found["enabled"], false);

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_use_for_kv() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        addr,
        ..
    } = start_server().await;

    client
        .post("v1.admin.auth-role.upsert")
        .json(json!({
            "id": "editor",
            "description": "Can edit things",
            "rules": [
                {
                    "effect": "allow",
                    "resource": "kv:*:*",
                    "actions": ["get", "set", "list"],
                }
            ],
            "policies": [],
            "context": {},
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let (_id, token) = create_admin_token(&client, "kv-token", "editor").await?;

    // Use the created token to make KV API calls
    let token_client = TestClient::new(format!("http://{addr}/api"), &token);

    let set_resp = token_client
        .post("v1.kv.set")
        .json(json!({
            "key": "hello",
            "value": "world".as_bytes(),
            "behavior": "upsert"
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(set_resp["success"], true);

    let resp = token_client
        .post("v1.kv.get")
        .json(json!({ "key": "hello" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["value"], json!("world".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn test_admin_auth_token_create_with_expiry() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-token.create")
        .json(json!({
            "name": "expiring-token",
            "role": "reader",
            "expiry_ms": 60_000u64,
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let id = resp["id"].assert_str().to_owned();
    assert!(!id.is_empty());
    assert!(resp["created"].is_i64());
    assert!(resp["updated"].is_i64());

    let list_resp = client
        .post("v1.admin.auth-token.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let tokens = list_resp["data"].assert_array();
    let found = tokens.iter().find(|t| t["id"] == id).unwrap();
    assert!(
        !found["expiry"].is_null(),
        "token should have an expiry set"
    );

    Ok(())
}
