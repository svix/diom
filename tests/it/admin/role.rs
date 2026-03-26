use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn test_admin_role_upsert_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.role.upsert")
        .json(json!({
            "id": "editor",
            "description": "Can edit things",
            "rules": [],
            "policies": [],
            "context": {},
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["id"], "editor");
    assert!(resp["created"].is_string());
    assert!(resp["updated"].is_string());

    let get_resp = client
        .post("v1.admin.role.get")
        .json(json!({ "id": "editor" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(get_resp["id"], "editor");
    assert_eq!(get_resp["description"], "Can edit things");

    Ok(())
}

#[tokio::test]
async fn test_admin_role_upsert_preserves_created() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let first = client
        .post("v1.admin.role.upsert")
        .json(json!({
            "id": "viewer",
            "description": "Read-only access",
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let created_at = first["created"].assert_str().to_owned();

    let second = client
        .post("v1.admin.role.upsert")
        .json(json!({
            "id": "viewer",
            "description": "Updated description",
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(
        second["created"], created_at,
        "created should be preserved on update"
    );
    assert_eq!(second["id"], "viewer");

    let get_resp = client
        .post("v1.admin.role.get")
        .json(json!({ "id": "viewer" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(get_resp["description"], "Updated description");

    Ok(())
}

#[tokio::test]
async fn test_admin_role_upsert_with_rules_and_policies() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.role.upsert")
        .json(json!({
            "id": "kv-reader",
            "description": "Can read KV",
            "rules": [
                {
                    "effect": "allow",
                    "resource": "kv:*:**",
                    "actions": ["Get", "List"],
                }
            ],
            "policies": ["base-policy"],
            "context": { "team": "platform" },
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let get_resp = client
        .post("v1.admin.role.get")
        .json(json!({ "id": "kv-reader" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let rules = get_resp["rules"].assert_array();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0]["effect"], "allow");
    assert_eq!(get_resp["policies"][0], "base-policy");
    assert_eq!(get_resp["context"]["team"], "platform");

    Ok(())
}

#[tokio::test]
async fn test_admin_role_list() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.role.upsert")
        .json(json!({ "id": "role-a", "description": "Role A" }))
        .await?
        .ensure(StatusCode::OK)?;

    client
        .post("v1.admin.role.upsert")
        .json(json!({ "id": "role-b", "description": "Role B" }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = client
        .post("v1.admin.role.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let data = resp["data"].assert_array();
    let ids: Vec<_> = data.iter().map(|r| r["id"].assert_str()).collect();
    assert!(ids.contains(&"role-a"), "role-a should be in list");
    assert!(ids.contains(&"role-b"), "role-b should be in list");

    Ok(())
}

#[tokio::test]
async fn test_admin_role_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.role.upsert")
        .json(json!({ "id": "to-delete", "description": "Temporary role" }))
        .await?
        .ensure(StatusCode::OK)?;

    let del_resp = client
        .post("v1.admin.role.delete")
        .json(json!({ "id": "to-delete" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(del_resp["success"], true);

    client
        .post("v1.admin.role.get")
        .json(json!({ "id": "to-delete" }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_admin_role_delete_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.role.delete")
        .json(json!({ "id": "does-not-exist" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["success"], false);

    Ok(())
}

#[tokio::test]
async fn test_admin_role_get_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.role.get")
        .json(json!({ "id": "no-such-role" }))
        .await?
        .ensure_not_found()?;

    Ok(())
}
