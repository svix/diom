use http::StatusCode;
use serde_json::json;
use test_utils::{
    TestClient, TestResult,
    server::{TestServerBuilder, default_server_config},
};

async fn assert_bootstrap_namespaces(client: &TestClient) -> TestResult {
    let default_kv = client
        .post("kv/namespace/get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_kv["max_storage_bytes"], 1000);
    assert_eq!(default_kv["storage_type"], "Ephemeral");

    let default_cache = client
        .post("cache/namespace/get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_cache["name"], "default");
    assert_eq!(default_cache["eviction_policy"], "NoEviction");

    let default_idempotency = client
        .post("idempotency/namespace/get")
        .json(json!({"name": "default"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_idempotency["name"], "default");

    let kv1 = client
        .post("kv/namespace/get")
        .json(json!({"name": "kv1"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv1["name"], "kv1");
    assert_eq!(kv1["max_storage_bytes"], 2000);
    assert_eq!(kv1["storage_type"], "Ephemeral");

    let kv2 = client
        .post("kv/namespace/get")
        .json(json!({"name": "kv2"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv2["name"], "kv2");
    assert_eq!(kv2["max_storage_bytes"], 3000);

    let cache1 = client
        .post("cache/namespace/get")
        .json(json!({"name": "cache1"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cache1["name"], "cache1");
    assert_eq!(cache1["eviction_policy"], "LeastRecentlyUsed");
    assert_eq!(cache1["storage_type"], "Persistent");

    let stream2 = client
        .post("msgs/namespace/get")
        .json(json!({"name": "stream2"}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(stream2["name"], "stream2");
    assert!(stream2["retention"].is_object());
    assert_eq!(stream2["storage_type"], "Persistent");

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_file_based() -> TestResult {
    let workdir = tempfile::tempdir()?;
    let mut cfg = default_server_config(workdir.path());
    cfg.bootstrap_cfg_path = Some("./tests/it/static/bootstrap.test.yaml".to_string());

    let test_server = TestServerBuilder::new().cfg(cfg).build().await;
    assert_bootstrap_namespaces(&test_server.client).await
}

#[tokio::test]
async fn test_bootstrap_env_var_based() -> TestResult {
    let workdir = tempfile::tempdir()?;
    let mut cfg = default_server_config(workdir.path());
    cfg.bootstrap_cfg = Some(include_str!("static/bootstrap.test.yaml").to_string());

    let test_server = TestServerBuilder::new().cfg(cfg).build().await;
    assert_bootstrap_namespaces(&test_server.client).await
}
