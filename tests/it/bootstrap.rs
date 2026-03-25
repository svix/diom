use http::StatusCode;
use serde_json::json;
use test_utils::{TestClient, TestResult, server::TestServerBuilder};

async fn assert_bootstrap_namespaces(client: &TestClient) -> TestResult {
    let default_kv = client
        .post("v1.kv.namespace.get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_kv["max_storage_bytes"], 1000);

    let default_cache = client
        .post("v1.cache.namespace.get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_cache["name"], "default");
    assert_eq!(default_cache["eviction_policy"], "NoEviction");

    let default_idempotency = client
        .post("v1.idempotency.namespace.get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_idempotency["name"], "default");

    let kv1 = client
        .post("v1.kv.namespace.get")
        .json(json!({"name": "kv1"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv1["name"], "kv1");
    assert_eq!(kv1["max_storage_bytes"], 2000);

    let kv2 = client
        .post("v1.kv.namespace.get")
        .json(json!({"name": "kv2"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv2["name"], "kv2");
    assert_eq!(kv2["max_storage_bytes"], 3000);

    let cache1 = client
        .post("v1.cache.namespace.get")
        .json(json!({"name": "cache1"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cache1["name"], "cache1");
    assert_eq!(cache1["eviction_policy"], "LeastRecentlyUsed");

    let msgs2 = client
        .post("v1.msgs.namespace.get")
        .json(json!({"name": "msgs2"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(msgs2["name"], "msgs2");
    assert!(msgs2["retention"].is_object());

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_file_based() -> TestResult {
    let test_server = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| cfg.bootstrap_cfg_path = Some("tests/it/static/bootstrap.test".to_string()))
        .build()
        .await;
    assert_bootstrap_namespaces(&test_server.client).await
}

#[tokio::test]
async fn test_bootstrap_env_var_based() -> TestResult {
    let content = include_str!("static/bootstrap.test").to_string();
    let test_server = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| cfg.bootstrap_cfg = Some(content))
        .build()
        .await;
    assert_bootstrap_namespaces(&test_server.client).await
}
