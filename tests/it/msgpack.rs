use serde::Serialize;
use test_utils::{StatusCode, TestResult};

#[derive(Serialize)]
struct CacheSetIn<'a> {
    key: &'a str,
    expire_in: u32,
    value: &'a str,
}

#[derive(Serialize)]
struct CacheGetIn<'a> {
    key: &'a str,
}

#[tokio::test]
async fn test_cache_set_and_get() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    client
        .post("cache/set")
        .msgpack(CacheSetIn {
            key: "test-key-1",
            expire_in: 60000,
            value: "test-value-123",
        })
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("cache/get")
        .msgpack(CacheGetIn { key: "test-key-1" })
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["key"], "test-key-1");
    assert_eq!(response["value"], "test-value-123");
    assert!(response["expires_at"].is_string());

    Ok(())
}
