use std::time::Duration;

use serde_json::json;
use test_utils::{
    StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn call_limit_token_bucket(
    client: &TestClient,
    key: &str,
    units: u64,
    capacity: u64,
    refill_amount: u64,
    refill_interval_seconds: u64,
) -> TestResult<serde_json::Value> {
    let response = client
        .post("v1.rate-limit.limit")
        .json(json!({
            "key": key,
            "tokens": units,
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval_ms": refill_interval_seconds * 1000
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

#[tokio::test]
async fn test_rate_limiter_limit_token_bucket() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;
    let refill_interval = 1;
    let refill_amount = 5;
    let capacity = 5;

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        1,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["allowed"], true);
    assert_eq!(response["remaining"], 4);
    assert_eq!(response["retry_after_ms"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        2,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["allowed"], true);
    assert_eq!(response["remaining"], 2);
    assert_eq!(response["retry_after_ms"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        2,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["allowed"], true);
    assert_eq!(response["remaining"], 0);
    assert_eq!(response["retry_after_ms"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        1,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["allowed"], false);
    assert_eq!(response["remaining"], 0);
    assert!(response["retry_after_ms"].is_number());

    time.fast_forward(Duration::from_secs(1));

    let response = client
        .post("v1.rate-limit.get-remaining")
        .json(json!({
            "key": "rl-key-1",
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval_ms": refill_interval * 1000
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["remaining"], capacity);
    assert_eq!(response["retry_after_ms"], json!(null));

    Ok(())
}

#[tokio::test]
async fn test_rate_limiter_refill_interval() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;
    let refill_interval = 5;
    let refill_amount = 10;

    let capacity = 6;

    call_limit_token_bucket(
        &client,
        "rl-key-1",
        2,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;

    time.fast_forward(Duration::from_secs(2));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        1,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["remaining"], 3); // Does not refill between intervals

    time.fast_forward(Duration::from_secs(3));

    let response = client
        .post("v1.rate-limit.get-remaining")
        .json(json!({
                "key": "rl-key-1",
                "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval_ms": refill_interval * 1000
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["remaining"], 6); // fill up to capacity
    assert_eq!(response["retry_after_ms"], json!(null));

    Ok(())
}
