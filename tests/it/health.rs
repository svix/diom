use std::time::Duration;

use diom_core::types::NonZeroDurationMs;
use test_utils::{JsonFastAndLoose as _, StatusCode, TestResult, server::TestServerBuilder};

#[tokio::test]
async fn test_health_ping() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .set_wait_for_initialization(false)
        .tap_cfg(|c| {
            c.bootstrap_delay = Some(NonZeroDurationMs::from_secs(1).unwrap());
        })
        .build()
        .await;
    let response = ctx.client.get("v1.health.ping").await?;
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.json();
    assert!(body["ok"].assert_bool());
    Ok(())
}

#[tokio::test]
async fn test_health_ready() -> TestResult {
    let ctx = TestServerBuilder::with_default_config()
        .set_wait_for_initialization(false)
        .tap_cfg(|c| {
            c.bootstrap_delay = Some(NonZeroDurationMs::from_secs(1).unwrap());
        })
        .build()
        .await;
    let response = ctx.client.get("v1.health.ready").await?;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    tokio::time::sleep(Duration::from_secs(2)).await;
    let response = ctx.client.get("v1.health.ready").await?;
    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}

#[tokio::test]
async fn test_error() -> TestResult {
    let ctx = TestServerBuilder::with_default_config().build().await;
    let response = ctx.client.post("v1.health.error").await?;
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = response.json();
    assert_eq!(body["code"].assert_str(), "internal");
    assert_eq!(
        body["detail"].assert_str(),
        "despite appearances, I am not an error"
    );
    assert_eq!(body["type"].assert_str(), "server-error");
    Ok(())
}
