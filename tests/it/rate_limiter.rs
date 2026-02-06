use std::time::Duration;

use serde_json::{Value, json};
use test_utils::{
    StatusCode, TestClient, TestResult,
    server::{TestContext, start_server},
};

async fn call_limit_token_bucket(
    client: &TestClient,
    key: &str,
    units: u64,
    capacity: u64,
    refill_amount: u64,
    refill_interval_seconds: u64,
) -> TestResult<Value> {
    let response = client
        .post("rate-limiter/limit")
        .json(json!({
            "key": key,
            "units": units,
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval_seconds": refill_interval_seconds
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
    assert_eq!(response["result"], "OK");
    assert_eq!(response["remaining"], 4);
    assert_eq!(response["retry_after"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        2,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["result"], "OK");
    assert_eq!(response["remaining"], 2);
    assert_eq!(response["retry_after"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        2,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["result"], "OK");
    assert_eq!(response["remaining"], 0);
    assert_eq!(response["retry_after"], json!(null));

    let response = call_limit_token_bucket(
        &client,
        "rl-key-1",
        1,
        capacity,
        refill_amount,
        refill_interval,
    )
    .await?;
    assert_eq!(response["result"], "BLOCK");
    assert_eq!(response["remaining"], 0);
    assert!(response["retry_after"].is_number());

    tokio::time::sleep(Duration::from_secs(1)).await;

    let response = client
        .post("rate-limiter/get-remaining")
        .json(json!({
            "key": "rl-key-1",
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval_seconds": refill_interval
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["remaining"], capacity);
    assert_eq!(response["retry_after"], json!(null));

    Ok(())
}
