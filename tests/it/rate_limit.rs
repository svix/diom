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
        .post("rate-limiter/limit")
        .json(json!({
            "key": key,
            "tokens": units,
            "method": "token_bucket",
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval": refill_interval_seconds
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    Ok(response)
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn call_limit_fixed_window(
    client: &TestClient,
    key: &str,
    units: u64,
    max_requests: u64,
    window_size_seconds: u64,
) -> TestResult<serde_json::Value> {
    let response = client
        .post("rate-limiter/limit")
        .json(json!({
            "key": key,
            "tokens": units,
            "method": "fixed_window",
            "config": {
                "max_requests": max_requests,
                "window_size": window_size_seconds
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
    assert_eq!(response["status"], "ok");
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
    assert_eq!(response["status"], "ok");
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
    assert_eq!(response["status"], "ok");
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
    assert_eq!(response["status"], "block");
    assert_eq!(response["remaining"], 0);
    assert!(response["retry_after"].is_number());

    tokio::time::sleep(Duration::from_secs(1)).await;

    let response = client
        .post("rate-limiter/get-remaining")
        .json(json!({
            "key": "rl-key-1",
            "method": "token_bucket",
            "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval": refill_interval
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["remaining"], capacity);
    assert_eq!(response["retry_after"], json!(null));

    Ok(())
}

#[tokio::test]
async fn test_rate_limiter_limit_fixed_window() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;
    let max_requests = 5;
    let window_size_seconds = 3;
    let key = "rl-key-fixed-window";

    let response =
        call_limit_fixed_window(&client, key, 1, max_requests, window_size_seconds).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 4);
    assert_eq!(response["retry_after"], json!(null));

    let response =
        call_limit_fixed_window(&client, key, 2, max_requests, window_size_seconds).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 2);
    assert_eq!(response["retry_after"], json!(null));

    let response =
        call_limit_fixed_window(&client, key, 2, max_requests, window_size_seconds).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 0);
    assert_eq!(response["retry_after"], json!(null));

    let response =
        call_limit_fixed_window(&client, key, 1, max_requests, window_size_seconds).await?;
    assert_eq!(response["status"], "block");
    assert_eq!(response["remaining"], 0);
    assert!(response["retry_after"].is_number());

    tokio::time::sleep(Duration::from_secs(3)).await;

    let response = client
        .post("rate-limiter/get-remaining")
        .json(json!({
            "key": key,
            "method": "fixed_window",
            "config": {
                "max_requests": max_requests,
                "window_size": window_size_seconds
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(response["remaining"], max_requests);
    assert_eq!(response["retry_after"], json!(null));

    Ok(())
}

#[tokio::test]
async fn test_rate_limiter_change_algorithm() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;
    let max_requests = 5;
    let window_size_seconds = 1;

    let response =
        call_limit_fixed_window(&client, "rl-key-1", 1, max_requests, window_size_seconds).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 4);
    assert_eq!(response["retry_after"], json!(null));

    // Calling limit with the same key but a different algorithm behaves as a different key
    let response = call_limit_token_bucket(&client, "rl-key-1", 1, 5, 1, 1).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 4);
    assert_eq!(response["retry_after"], json!(null));

    // Change algorithm parameters
    let response = call_limit_token_bucket(&client, "rl-key-1", 1, 10, 4, 2).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 3);
    assert_eq!(response["retry_after"], json!(null));

    tokio::time::sleep(Duration::from_secs(2)).await;

    let response = call_limit_token_bucket(&client, "rl-key-1", 1, 10, 4, 2).await?;
    assert_eq!(response["status"], "ok");
    assert_eq!(response["remaining"], 6); // 3 (previous) + 4 (refill) - 1 (consumed)
    assert_eq!(response["retry_after"], json!(null));

    Ok(())
}

#[tokio::test]
async fn test_rate_limiter_refill_interval() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
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

    tokio::time::sleep(Duration::from_secs(2)).await;

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

    tokio::time::sleep(Duration::from_secs(3)).await;

    let response = client
        .post("rate-limiter/get-remaining")
        .json(json!({
                "key": "rl-key-1",
                "method": "token_bucket",
                "config": {
                "capacity": capacity,
                "refill_amount": refill_amount,
                "refill_interval": refill_interval
            }
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["remaining"], 6); // fill up to capacity
    assert_eq!(response["retry_after"], json!(null));

    Ok(())
}
