use anyhow::{Context as _, Result};
use diom::{DiomClient, DiomOptions};
use http::HeaderValue;
use serde_json::json;
use std::time::Duration;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

struct Context {
    mock_server: MockServer,
    diom: DiomClient,
}

impl Context {
    async fn run_test<O, F: AsyncFnOnce(Self) -> Result<O>>(f: F) -> Result<O> {
        let mock_server = MockServer::start().await;
        let options = DiomOptions {
            server_url: Some(mock_server.uri()),
            timeout: Some(Duration::from_millis(500)),
            retry_schedule: Some(vec![Duration::from_millis(10), Duration::from_millis(10)]),
            ..Default::default()
        };
        let diom = DiomClient::new("not-a-token".to_string(), Some(options));
        let this = Self { mock_server, diom };
        f(this).await.context("running test")
    }
}

#[tokio::test]
async fn test_basic_json() -> Result<()> {
    Context::run_test(async move |ctx| {
        ctx.mock_server
            .register(
                Mock::given(method("GET"))
                    .and(path("/api/v1.health.ping"))
                    .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true }))),
            )
            .await;
        let res = ctx.diom.health().ping().await?;
        assert!(res.ok);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_basic_msgpack() -> Result<()> {
    const APPLICATION_MSGPACK: HeaderValue = HeaderValue::from_static("application/msgpack");

    let ping_ok = rmp_serde::to_vec_named(&diom::models::PingOut { ok: true })?;

    Context::run_test(async move |ctx| {
        ctx.mock_server
            .register(
                Mock::given(method("GET"))
                    .and(path("/api/v1.health.ping"))
                    .respond_with(
                        ResponseTemplate::new(200)
                            .insert_header(http::header::CONTENT_TYPE, APPLICATION_MSGPACK)
                            .set_body_bytes(ping_ok),
                    ),
            )
            .await;
        let res = ctx.diom.health().ping().await?;
        assert!(res.ok);
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_retries_on_5xx() -> Result<()> {
    Context::run_test(async move |ctx| {
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .with_priority(1)
            .mount(&ctx.mock_server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true })))
            .mount(&ctx.mock_server)
            .await;
        let res = ctx.diom.health().ping().await?;
        assert!(res.ok);
        assert_eq!(
            ctx.mock_server
                .received_requests()
                .await
                .expect("should have received requests")
                .len(),
            2
        );
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_retries_exhaustion() -> Result<()> {
    Context::run_test(async move |ctx| {
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(3)
            .with_priority(1)
            .mount(&ctx.mock_server)
            .await;
        let res = ctx.diom.health().ping().await.expect_err("should fail");
        assert!(res.is_server_error());
        assert_eq!(
            ctx.mock_server
                .received_requests()
                .await
                .expect("should have received requests")
                .len(),
            3
        );
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_not_retries_on_4xx() -> Result<()> {
    Context::run_test(async move |ctx| {
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(ResponseTemplate::new(422).set_body_json(json!({
                "type": "invalid-input",
                "code": "BAD_REQUEST",
                "detail": "whatever",
                "location": null
            })))
            .mount(&ctx.mock_server)
            .await;
        let res = ctx.diom.health().ping().await.expect_err("should fail");
        assert!(res.is_invalid_input());
        assert_eq!(
            ctx.mock_server
                .received_requests()
                .await
                .expect("should have received requests")
                .len(),
            1
        );
        Ok(())
    })
    .await
}

#[tokio::test]
async fn test_retries_on_timeout() -> Result<()> {
    Context::run_test(async move |ctx| {
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({ "ok": true }))
                    .set_delay(Duration::from_secs(3)),
            )
            .up_to_n_times(1)
            .with_priority(1)
            .mount(&ctx.mock_server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1.health.ping"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true })))
            .mount(&ctx.mock_server)
            .await;
        let res = ctx.diom.health().ping().await?;
        assert!(res.ok);
        assert_eq!(
            ctx.mock_server
                .received_requests()
                .await
                .expect("should have received requests")
                .len(),
            2
        );
        Ok(())
    })
    .await
}
