use http::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[derive(Serialize)]
struct CacheSetIn<'a> {
    key: &'a str,
    ttl_ms: u32,
    value: &'a [u8],
}

#[derive(Serialize)]
struct CacheGetIn<'a> {
    key: &'a str,
}

#[tokio::test]
async fn test_cache_set_and_get_msgpack_in() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.cache.set")
        .header("accept", "application/json")
        .msgpack(CacheSetIn {
            key: "test-key-1",
            ttl_ms: 60000,
            value: b"test-value-123",
        })
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.cache.get")
        .header("accept", "application/json")
        .msgpack(CacheGetIn { key: "test-key-1" })
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["value"], "test-value-123");
    assert!(response["expiry"].is_string());

    Ok(())
}

#[derive(Deserialize)]
struct CacheGetOut {
    value: Option<Vec<u8>>,
    #[allow(unused)]
    expiry: Option<String>,
}

#[tokio::test]
async fn test_cache_set_and_get_msgpack_out() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.cache.set")
        .json(json!({
            "key": "test-key-1",
            "ttl_ms": 60000,
            "value": "test-value-123".as_bytes(),
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.cache.get")
        .header("accept", "application/msgpack")
        .json(json!({ "key": "test-key-1" }))
        .await?
        .expect(StatusCode::OK);

    let response_content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .expect("response must have a content-type");
    assert_eq!(response_content_type, "application/msgpack");

    let response_body: CacheGetOut = rmp_serde::from_slice(response.body())?;
    assert_eq!(
        response_body.value.as_deref(),
        Some(b"test-value-123" as &[u8])
    );

    Ok(())
}
