use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn create_token(
    client: &TestClient,
    name: &str,
    owner_id: &str,
) -> TestResult<(String, String)> {
    let resp = client
        .post("auth-token/create")
        .json(json!({
            "name": name,
            "owner_id": owner_id,
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok((
        resp["id"].assert_str().to_owned(),
        resp["token"].assert_str().to_owned(),
    ))
}

#[allow(clippy::disallowed_types)]
async fn verify_token(client: &TestClient, token: &str) -> TestResult<serde_json::Value> {
    let resp = client
        .post("auth-token/verify")
        .json(json!({ "token": token }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok(resp)
}

#[allow(clippy::disallowed_types)]
async fn verify_token_with_namespace(
    client: &TestClient,
    token: &str,
    namespace: Option<&str>,
) -> TestResult<serde_json::Value> {
    let resp = client
        .post("auth-token/verify")
        .json(json!({ "token": token, "namespace": namespace }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok(resp)
}

#[tokio::test]
async fn test_auth_token_verify_invalid() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = verify_token(&client, "not-a-real-token").await?;
    assert!(resp["token"].is_null());

    Ok(())
}

#[tokio::test]
async fn test_auth_token_create_and_verify() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, token) = create_token(&client, "my-token", "user-1").await?;
    assert!(!id.is_empty());
    assert!(!token.is_empty());

    let resp = verify_token(&client, &token).await?;
    assert_eq!(resp["token"]["owner_id"], "user-1");
    assert_eq!(resp["token"]["name"], "my-token");
    assert_eq!(resp["token"]["enabled"], true);

    Ok(())
}

#[tokio::test]
async fn test_auth_token_expire_immediately() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, token) = create_token(&client, "expire-now", "user-2").await?;

    client
        .post("auth-token/expire")
        .json(json!({ "id": id }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = verify_token(&client, &token).await?;
    assert!(resp["token"].is_null());

    Ok(())
}

#[tokio::test]
async fn test_auth_token_expire_in_future() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    let (id, token) = create_token(&client, "expire-future", "user-3").await?;

    client
        .post("auth-token/expire")
        .json(json!({ "id": id, "expiry_millis": 500 }))
        .await?
        .ensure(StatusCode::OK)?;

    verify_token(&client, &token).await?;

    time.fast_forward(Duration::from_secs(1));

    let resp = verify_token(&client, &token).await?;
    assert!(resp["token"].is_null());

    Ok(())
}

#[tokio::test]
async fn test_auth_token_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, token) = create_token(&client, "to-delete", "user-4").await?;

    verify_token(&client, &token).await?;

    let resp = client
        .post("auth-token/delete")
        .json(json!({ "id": id }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["success"], true);

    let resp = verify_token(&client, &token).await?;
    assert!(resp["token"].is_null());

    Ok(())
}

#[tokio::test]
async fn test_auth_token_delete_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("auth-token/delete")
        .json(json!({ "id": "key_06egrha0d5x9x8wa4kfcy1prhr" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["success"], false);

    Ok(())
}

#[tokio::test]
async fn test_auth_token_update() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, token) = create_token(&client, "original-name", "user-5").await?;

    client
        .post("auth-token/update")
        .json(json!({
            "id": id,
            "name": "renamed",
            "enabled": false,
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = verify_token(&client, &token).await?;
    assert_eq!(resp["token"]["name"], "renamed");
    assert_eq!(resp["token"]["enabled"], false);

    Ok(())
}

#[tokio::test]
async fn test_auth_token_namespace_create_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("auth-token/namespace/create")
        .json(json!({ "name": "at-ns-1" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["name"], "at-ns-1");
    assert_eq!(resp["storage_type"], "Persistent");
    assert!(resp["created"].is_string());
    assert!(resp["updated"].is_string());

    let get_resp = client
        .post("auth-token/namespace/get")
        .json(json!({ "name": "at-ns-1" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(get_resp["name"], "at-ns-1");
    assert_eq!(get_resp["created"], resp["created"]);
    assert_eq!(get_resp["updated"], resp["updated"]);

    Ok(())
}

#[tokio::test]
async fn test_auth_token_rotate() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let (id, old_token) = create_token(&client, "rotate-me", "user-r1").await?;

    let resp = client
        .post("auth-token/rotate")
        .json(json!({ "id": id }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let new_id = resp["id"].assert_str();
    let new_token = resp["token"].assert_str();
    assert_ne!(new_id, id);
    assert_ne!(new_token, old_token);
    assert!(resp["created"].is_string());

    // Old token is immediately expired.
    let resp = verify_token(&client, &old_token).await?;
    assert!(resp["token"].is_null());

    // New token is valid and has the same name/owner.
    let resp = verify_token(&client, new_token).await?;
    assert_eq!(resp["token"]["name"], "rotate-me");
    assert_eq!(resp["token"]["owner_id"], "user-r1");

    Ok(())
}

#[tokio::test]
async fn test_auth_token_rotate_with_expiry() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    let (id, old_token) = create_token(&client, "rotate-grace", "user-r2").await?;

    client
        .post("auth-token/rotate")
        .json(json!({ "id": id, "expiry_millis": 1000 }))
        .await?
        .ensure(StatusCode::OK)?;

    // Old token still valid during grace period.
    let resp = verify_token(&client, &old_token).await?;
    assert!(resp["token"].is_object());

    time.fast_forward(Duration::from_secs(2));

    // Old token expired after grace period.
    let resp = verify_token(&client, &old_token).await?;
    assert!(resp["token"].is_null());

    Ok(())
}

#[tokio::test]
async fn test_auth_token_rotate_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("auth-token/rotate")
        .json(json!({ "id": "key_06egrha0d5x9x8wa4kfcy1prhr" }))
        .await?
        .ensure(StatusCode::NOT_FOUND)?;

    Ok(())
}

#[tokio::test]
async fn test_auth_token_namespace_get_not_found() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("auth-token/namespace/get")
        .json(json!({ "name": "no-such-ns" }))
        .await?
        .ensure(StatusCode::NOT_FOUND)?;

    Ok(())
}

#[tokio::test]
async fn test_auth_token_in_custom_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("auth-token/namespace/create")
        .json(json!({ "name": "custom-at-ns" }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = client
        .post("auth-token/create")
        .json(json!({
            "namespace": "custom-at-ns",
            "name": "ns-token",
            "owner_id": "user-6",
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let token = resp["token"].assert_str();

    client
        .post("auth-token/verify")
        .json(json!({
            "namespace": "custom-at-ns",
            "token": token,
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = verify_token_with_namespace(&client, token, None).await?;
    assert!(resp["token"].is_null());

    Ok(())
}
