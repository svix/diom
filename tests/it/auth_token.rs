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
        .post("v1.auth-token.create")
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
        .post("v1.auth-token.verify")
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
        .post("v1.auth-token.verify")
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
        .post("v1.auth-token.expire")
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
        .post("v1.auth-token.expire")
        .json(json!({ "id": id, "expiry_ms": 500 }))
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
        .post("v1.auth-token.delete")
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
        .post("v1.auth-token.delete")
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
        .post("v1.auth-token.update")
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
        .post("v1.auth-token.namespace.create")
        .json(json!({ "name": "at-ns-1" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["name"], "at-ns-1");
    assert!(resp["created"].is_string());
    assert!(resp["updated"].is_string());

    let get_resp = client
        .post("v1.auth-token.namespace.get")
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
        .post("v1.auth-token.rotate")
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
        .post("v1.auth-token.rotate")
        .json(json!({ "id": id, "expiry_ms": 1000 }))
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
        .post("v1.auth-token.rotate")
        .json(json!({ "id": "key_06egrha0d5x9x8wa4kfcy1prhr" }))
        .await?
        .ensure_not_found()?;

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
        .post("v1.auth-token.namespace.get")
        .json(json!({ "name": "no-such-ns" }))
        .await?
        .ensure_not_found()?;

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
        .post("v1.auth-token.namespace.create")
        .json(json!({ "name": "custom-at-ns" }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = client
        .post("v1.auth-token.create")
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
        .post("v1.auth-token.verify")
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

#[tokio::test]
async fn test_auth_token_list_pagination() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let owner = "list-owner";

    // Create 5 tokens.
    let mut ids = Vec::new();
    for i in 0..5u32 {
        let (id, _) = create_token(&client, &format!("tok-{i}"), owner).await?;
        ids.push(id);
    }

    // List all (limit > count): done=true, no iterator.
    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "owner_id": owner, "limit": 10 }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["done"], true);
    assert!(!resp["iterator"].is_null());
    assert_eq!(resp["data"].as_array().unwrap().len(), 5);
    // Now fetch again with the iterator
    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "owner_id": owner, "iterator": resp["iterator"].as_str(), "limit": 10 }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["done"], true);
    assert!(!resp["iterator"].is_null());
    assert_eq!(resp["data"].as_array().unwrap().len(), 0);

    // First page (limit=2): done=false
    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "owner_id": owner, "limit": 2 }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["done"], false);
    let iter = resp["iterator"].assert_str().to_owned();
    let page1_ids: Vec<String> = resp["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].assert_str().to_owned())
        .collect();
    assert_eq!(page1_ids.len(), 2);

    // Second page: 2 more items, done=false.
    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "owner_id": owner, "limit": 2, "iterator": iter }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["done"], false);
    let iter = resp["iterator"].assert_str().to_owned();
    let page2_ids: Vec<String> = resp["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].assert_str().to_owned())
        .collect();
    assert_eq!(page2_ids.len(), 2);

    // Pages don't overlap.
    for id in &page1_ids {
        assert!(!page2_ids.contains(id));
    }

    // Third page: 1 item, done=true.
    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "owner_id": owner, "limit": 2, "iterator": iter }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(resp["done"], true);
    let page3_ids: Vec<String> = resp["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["id"].assert_str().to_owned())
        .collect();
    assert_eq!(page3_ids.len(), 1);

    // All 5 tokens accounted for across 3 pages, no duplicates.
    let mut all_ids = page1_ids;
    all_ids.extend(page2_ids);
    all_ids.extend(page3_ids);
    all_ids.sort();
    all_ids.dedup();
    assert_eq!(all_ids.len(), 5);

    Ok(())
}

#[tokio::test]
async fn test_global_admin_token_cannot_access_reserved_ns() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.auth-token.list")
        .json(json!({ "namespace": "_internal", "owner_id": "foo" }))
        .await?
        .ensure(StatusCode::FORBIDDEN)?
        .json();

    assert_eq!(
        resp,
        json!({
            "code": "forbidden",
            "detail": "You do not have permission to perform `list` on `auth_token:_internal:*`",
        })
    );

    Ok(())
}
