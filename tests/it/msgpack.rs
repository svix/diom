use http::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use test_utils::{StatusCode, TestResult};

use crate::TestContext;

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
async fn test_cache_set_and_get_msgpack_in() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = super::start_server().await;

    client
        .post("cache/set")
        .header("accept", "application/json")
        .msgpack(CacheSetIn {
            key: "test-key-1",
            expire_in: 60000,
            value: "test-value-123",
        })
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("cache/get")
        .header("accept", "application/json")
        .msgpack(CacheGetIn { key: "test-key-1" })
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["key"], "test-key-1");
    assert_eq!(response["value"], "test-value-123");
    assert!(response["expires_at"].is_string());

    Ok(())
}

#[derive(Deserialize)]
struct CacheGetOut {
    key: String,
    value: String,
    #[allow(unused)]
    expires_at: String,
}

#[tokio::test]
async fn test_cache_set_and_get_msgpack_out() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = super::start_server().await;

    client
        .post("cache/set")
        .json(json!({
            "key": "test-key-1",
            "expire_in": 60000,
            "value": "test-value-123",
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("cache/get")
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
    assert_eq!(response_body.key, "test-key-1");
    assert_eq!(response_body.value, "test-value-123");

    Ok(())
}
