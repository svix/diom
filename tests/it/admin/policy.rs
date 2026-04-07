use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn test_admin_access_policy_upsert_and_get() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({
            "id": "read-only",
            "description": "Allows reading everything",
            "rules": [],
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["id"], "read-only");
    assert!(resp["created"].is_i64());
    assert!(resp["updated"].is_i64());

    let get_resp = client
        .post("v1.admin.auth-policy.get")
        .json(json!({ "id": "read-only" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(get_resp["id"], "read-only");
    assert_eq!(get_resp["description"], "Allows reading everything");

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_upsert_preserves_created() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let first = client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({
            "id": "base-policy",
            "description": "Initial description",
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let created_at = first["created"].assert_u64();

    let second = client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({
            "id": "base-policy",
            "description": "Updated description",
        }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(
        second["created"], created_at,
        "created should be preserved on update"
    );
    assert_eq!(second["id"], "base-policy");

    let get_resp = client
        .post("v1.admin.auth-policy.get")
        .json(json!({ "id": "base-policy" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(get_resp["description"], "Updated description");

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_upsert_with_rules() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({
            "id": "kv-policy",
            "description": "KV access policy",
            "rules": [
                {
                    "effect": "allow",
                    "resource": "kv:*:*",
                    "actions": ["get", "set", "delete"],
                },
                {
                    "effect": "deny",
                    "resource": "kv:restricted:*",
                    "actions": ["set", "delete"],
                },
            ],
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let get_resp = client
        .post("v1.admin.auth-policy.get")
        .json(json!({ "id": "kv-policy" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let rules = get_resp["rules"].assert_array();
    assert_eq!(rules.len(), 2);
    assert_eq!(rules[0]["effect"], "allow");
    assert_eq!(rules[1]["effect"], "deny");

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_list() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({ "id": "policy-a", "description": "Policy A" }))
        .await?
        .ensure(StatusCode::OK)?;

    client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({ "id": "policy-b", "description": "Policy B" }))
        .await?
        .ensure(StatusCode::OK)?;

    let resp = client
        .post("v1.admin.auth-policy.list")
        .json(json!({}))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let data = resp["data"].assert_array();
    let ids: Vec<_> = data.iter().map(|p| p["id"].assert_str()).collect();
    assert!(
        ids.contains(&"policy-a"),
        "policy-a should be in list {ids:?}"
    );
    assert!(
        ids.contains(&"policy-b"),
        "policy-b should be in list {ids:?}"
    );

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_list_pagination() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // Create 3 policies with IDs that sort lexicographically: policy-aa < policy-ab < policy-ac
    for (id, desc) in [
        ("policy-aa", "Policy AA"),
        ("policy-ab", "Policy AB"),
        ("policy-ac", "Policy AC"),
    ] {
        client
            .post("v1.admin.auth-policy.upsert")
            .json(json!({ "id": id, "description": desc }))
            .await?
            .ensure(StatusCode::OK)?;
    }

    // First page: limit=2, no iterator
    let resp = client
        .post("v1.admin.auth-policy.list")
        .json(json!({ "limit": 2 }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let data = resp["data"].assert_array();
    assert_eq!(data.len(), 2, "first page should have 2 items");
    assert_eq!(resp["done"], false, "should not be done after first page");
    let iterator = resp["iterator"].assert_str().to_owned();
    assert_eq!(
        iterator, "policy-ab",
        "iterator should be id of last returned item"
    );

    // Second page: limit=2, pass iterator from first page
    let resp2 = client
        .post("v1.admin.auth-policy.list")
        .json(json!({ "limit": 2, "iterator": iterator }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    let data2 = resp2["data"].assert_array();
    assert_eq!(data2.len(), 1, "second page should have 1 item");
    assert_eq!(data2[0]["id"], "policy-ac");
    assert_eq!(resp2["done"], true, "should be done on last page");

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_delete() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({ "id": "to-delete", "description": "Temporary policy" }))
        .await?
        .ensure(StatusCode::OK)?;

    let del_resp = client
        .post("v1.admin.auth-policy.delete")
        .json(json!({ "id": "to-delete" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(del_resp["success"], true);

    client
        .post("v1.admin.auth-policy.get")
        .json(json!({ "id": "to-delete" }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_delete_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-policy.delete")
        .json(json!({ "id": "does-not-exist" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();

    assert_eq!(resp["success"], false);

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_get_nonexistent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.admin.auth-policy.get")
        .json(json!({ "id": "no-such-policy" }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn test_admin_access_policy_upsert_internal_ns() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let resp = client
        .post("v1.admin.auth-policy.upsert")
        .json(json!({
            "id": "canihazinternal",
            "description": "Trying to access _internal stuff",
            "rules": [
                {
                    "effect": "allow",
                    "resource": "kv:_internal:*",
                    "actions": ["get", "set", "delete"],
                },
            ],
        }))
        .await?
        .ensure(StatusCode::UNPROCESSABLE_ENTITY)?
        .json();

    assert_eq!(
        resp,
        json!({
            "detail": [{
                "type": "value_error",
                "loc": ["body", "rules"],
                "msg": "access rule 1 refers to a reserved namespace",
            }],
        })
    );

    Ok(())
}
