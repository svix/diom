use http::StatusCode;
use serde_json::json;
use test_utils::{
    TestResult,
    server::{TestServerBuilder, default_server_config},
};

#[tokio::test]
async fn test_bootstrap() -> TestResult {
    let workdir = tempfile::tempdir().unwrap();
    let mut cfg = default_server_config(workdir.path());
    cfg.bootstrap_cfg_path = Some("./tests/it/static/bootstrap.test.yaml".to_string());

    let test_server = TestServerBuilder::new().cfg(cfg).build().await;
    let client = test_server.client;

    let default_kv_namespace = client
        .post("kv/get-namespace")
        .json(json!({
            "name": "default",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(default_kv_namespace["max_storage_bytes"], 1000);
    assert_eq!(default_kv_namespace["storage_type"], "Ephemeral");

    let default_cache_namespace = client
        .post("cache/get-namespace")
        .json(json!({
            "name": "default",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(default_cache_namespace["name"], "default");
    assert_eq!(default_cache_namespace["eviction_policy"], "NoEviction");

    let default_idempotency_namespace = client
        .post("idempotency/get-namespace")
        .json(json!({
            "name": "default",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(default_idempotency_namespace["name"], "default");

    let kv1 = client
        .post("kv/get-namespace")
        .json(json!({
            "name": "kv1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv1["name"], "kv1");
    assert_eq!(kv1["max_storage_bytes"], 2000);
    assert_eq!(kv1["storage_type"], "Ephemeral");

    let kv2 = client
        .post("kv/get-namespace")
        .json(json!({
            "name": "kv2",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(kv2["name"], "kv2");
    assert_eq!(kv2["max_storage_bytes"], 3000);

    let cache1 = client
        .post("cache/get-namespace")
        .json(json!({
            "name": "cache1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(cache1["name"], "cache1");
    assert_eq!(cache1["eviction_policy"], "LeastRecentlyUsed");
    assert_eq!(cache1["storage_type"], "Persistent");

    let stream2 = client
        .post("msgs/namespace/get")
        .json(json!({
            "name": "stream2",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(stream2["name"], "stream2");
    assert!(stream2["retention"].is_object());
    assert_eq!(stream2["storage_type"], "Persistent");

    Ok(())
}
