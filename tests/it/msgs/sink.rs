use std::time::Duration;

use diom_core::types::NonZeroDurationMs;
use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    retry::run_with_retries,
    server::{TestServerBuilder, start_server},
};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

#[tokio::test]
async fn test_sink_configure() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "sink_123",
            "config": {
                "type": "http",
                "data": { "url": "https://example.test/ingest" },
            },
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "orders");
    assert_eq!(response["consumer_group"], "sink_123");

    Ok(())
}

#[tokio::test]
async fn test_sink_configure_rejects_invalid_template() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "sink_bad",
            "config": {
                "type": "http",
                "data": { "url": "https://example.test/${org_id/ingest" },
            },
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY)
        .json();

    assert_eq!(response["type"], "invalid-input");
    assert_eq!(response["code"], "invalid-data");
    let detail = response["detail"].as_str().expect("detail string");
    assert!(
        detail.contains("unterminated `${` placeholder"),
        "error should explain the template parse failure, got: {detail}"
    );

    Ok(())
}

#[tokio::test]
async fn test_sink_configure_and_delete() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "sink_del",
            "config": {
                "type": "http",
                "data": { "url": "https://example.test/ingest" },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.sink.delete")
        .json(json!({
            "topic": "orders",
            "consumer_group": "sink_del",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "orders");
    assert_eq!(response["consumer_group"], "sink_del");
    assert_eq!(response["success"], true);

    Ok(())
}

#[tokio::test]
async fn test_sink_list() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    for consumer_group in ["sink_a", "sink_b", "sink_c"] {
        ctx.client
            .post("v1.msgs.sink.configure")
            .json(json!({
                "topic": "orders",
                "consumer_group": consumer_group,
                "config": {
                    "type": "http",
                    "data": {
                        "url": "https://example.test/ingest",
                        "headers": { "Authorization": "Bearer super-secret-token" },
                    },
                },
            }))
            .await?
            .expect(StatusCode::OK);
    }

    let response = ctx
        .client
        .post("v1.msgs.sink.list")
        .json(json!({ "topic": "orders" }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = response["data"].as_array().expect("data array");
    assert_eq!(data.len(), 3);
    // Ordered by consumer group.
    assert_eq!(data[0]["consumer_group"], "sink_a");
    assert_eq!(data[2]["consumer_group"], "sink_c");
    assert_eq!(data[0]["topic"], "orders");
    assert_eq!(response["done"], true);

    // The config is returned verbatim, header values included.
    assert_eq!(
        data[0]["config"]["data"]["headers"]["Authorization"],
        "Bearer super-secret-token"
    );

    // The delivery knobs are flattened alongside the config, populated with their defaults.
    assert_eq!(data[0]["default_starting_position"], "earliest");

    Ok(())
}

#[tokio::test]
async fn test_sink_list_paginates() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    for consumer_group in ["sink_a", "sink_b", "sink_c"] {
        ctx.client
            .post("v1.msgs.sink.configure")
            .json(json!({
                "topic": "orders",
                "consumer_group": consumer_group,
                "config": {
                    "type": "http",
                    "data": { "url": "https://example.test/ingest" },
                },
            }))
            .await?
            .expect(StatusCode::OK);
    }

    let page = ctx
        .client
        .post("v1.msgs.sink.list")
        .json(json!({ "topic": "orders", "limit": 2 }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = page["data"].as_array().expect("data array");
    assert_eq!(data.len(), 2);
    assert_eq!(data[0]["consumer_group"], "sink_a");
    assert_eq!(data[1]["consumer_group"], "sink_b");
    assert_eq!(page["done"], false);

    let iterator = page["iterator"].as_str().expect("iterator cursor");
    let next = ctx
        .client
        .post("v1.msgs.sink.list")
        .json(json!({ "topic": "orders", "limit": 2, "iterator": iterator }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = next["data"].as_array().expect("data array");
    assert_eq!(data.len(), 1);
    assert_eq!(data[0]["consumer_group"], "sink_c");
    assert_eq!(next["done"], true);

    Ok(())
}

// ---------------------------------------------------------------------------
// End-to-end forwarding
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_sink_forwards_published_messages() -> TestResult {
    let mock = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock)
        .await;

    // Short poll interval so the background sink worker forwards promptly.
    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.background_cleanup_interval = NonZeroDurationMs::from_secs(1).unwrap();
        })
        .build()
        .await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "webhook",
            "default_starting_position": "earliest",
            "config": {
                "type": "http",
                "data": {
                    // `${headers.org_id}` should be filled in by `header["org_id"]`
                    // from the msg.
                    "url": format!("{}/ingest/${{headers.org_id}}", mock.uri()),
                    "method": "post",

                    // Just like headers, we can access the other message fields
                    // directly (`offset`, `value`, `topic`, ...).
                    "headers": { "X-Offset": "${offset}" },
                    "body": "value=${value};topic=${topic}",
                },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [
                { "value": "hello".as_bytes(), "headers": { "org_id": "acme" } },
                { "value": "world".as_bytes(), "headers": { "org_id": "acme" } },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    run_with_retries(async || {
        let reqs = mock.received_requests().await.unwrap_or_default();
        let acme: Vec<_> = reqs
            .iter()
            .filter(|r| r.url.path() == "/ingest/acme")
            .collect();
        anyhow::ensure!(
            acme.len() >= 2,
            "expected >= 2 deliveries, got {}",
            acme.len()
        );

        anyhow::ensure!(
            acme.iter().all(|r| r.method == "POST"),
            "each delivery should be a POST"
        );
        anyhow::ensure!(
            acme.iter().all(|r| r
                .headers
                .keys()
                .any(|k| k.as_str().eq_ignore_ascii_case("x-offset"))),
            "each request should carry the templated X-Offset header"
        );

        let bodies: String = acme
            .iter()
            .map(|r| String::from_utf8_lossy(&r.body).into_owned())
            .collect::<Vec<_>>()
            .join(" | ");
        anyhow::ensure!(
            bodies.contains("value=hello"),
            "missing hello value: {bodies}"
        );
        anyhow::ensure!(
            bodies.contains("value=world"),
            "missing world value: {bodies}"
        );
        anyhow::ensure!(
            bodies.contains("topic=orders"),
            "missing topic var: {bodies}"
        );
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_sink_delete_stops_forwarding() -> TestResult {
    let mock = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock)
        .await;

    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.background_cleanup_interval = NonZeroDurationMs::from_secs(1).unwrap();
        })
        .build()
        .await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "webhook",
            "default_starting_position": "earliest",
            "config": {
                "type": "http",
                "data": {
                    "url": format!("{}/ingest", mock.uri()),
                },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    // First message is delivered.
    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [ { "value": "first".as_bytes() } ],
        }))
        .await?
        .expect(StatusCode::OK);

    run_with_retries(async || {
        let n = mock.received_requests().await.unwrap_or_default().len();
        anyhow::ensure!(n >= 1, "expected first message to be delivered, got {n}");
        Ok(())
    })
    .await?;

    // Delete the sink, then publish again — the new message must not be delivered.
    ctx.client
        .post("v1.msgs.sink.delete")
        .json(json!({ "topic": "orders", "consumer_group": "webhook" }))
        .await?
        .expect(StatusCode::OK);

    let before = mock.received_requests().await.unwrap_or_default().len();

    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [ { "value": "second".as_bytes() } ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Give the worker several poll cycles to (not) deliver.
    tokio::time::sleep(Duration::from_secs(3)).await;

    let after = mock.received_requests().await.unwrap_or_default().len();
    assert_eq!(
        before, after,
        "no messages should be delivered after the sink is deleted"
    );

    Ok(())
}

#[tokio::test]
async fn test_sink_retries_until_delivered() -> TestResult {
    let mock = MockServer::start().await;
    // Fail the first two attempts, then succeed: the worker should keep retrying until it lands,
    // rather than giving up.
    Mock::given(any())
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(2)
        .with_priority(1)
        .mount(&mock)
        .await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .with_priority(2)
        .mount(&mock)
        .await;

    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.background_cleanup_interval = NonZeroDurationMs::from_secs(1).unwrap();
        })
        .build()
        .await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "webhook",
            "config": {
                "type": "http",
                "data": { "url": format!("{}/ingest", mock.uri()) },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [ { "value": "hello".as_bytes() } ],
        }))
        .await?
        .expect(StatusCode::OK);

    // The two failing attempts plus the eventual success should all land; the third request only
    // ever gets a 200, so reaching three attempts proves the message was delivered.
    run_with_retries(async || {
        let attempts = mock.received_requests().await.unwrap_or_default().len();
        anyhow::ensure!(
            attempts >= 3,
            "expected the message to be retried until delivered, got {attempts} attempts"
        );
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_sink_delete_clears_cursor() -> TestResult {
    let mock = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock)
        .await;

    let ctx = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.background_cleanup_interval = NonZeroDurationMs::from_secs(1).unwrap();
        })
        .build()
        .await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    let sink_config = json!({
        "topic": "orders",
        "consumer_group": "webhook",
        "config": { "type": "http", "data": { "url": format!("{}/ingest", mock.uri()) } },
    });

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(sink_config.clone())
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [ { "value": "hello".as_bytes() } ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for the first delivery, then give the worker a moment to commit the cursor.
    run_with_retries(async || {
        let n = mock.received_requests().await.unwrap_or_default().len();
        anyhow::ensure!(n >= 1, "expected first delivery, got {n}");
        Ok(())
    })
    .await?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    let before = mock.received_requests().await.unwrap_or_default().len();

    // Delete the sink (which should clear its cursor) and re-create it with the same consumer group.
    ctx.client
        .post("v1.msgs.sink.delete")
        .json(json!({ "topic": "orders", "consumer_group": "webhook" }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.sink.configure")
        .json(sink_config)
        .await?
        .expect(StatusCode::OK);

    // With the cursor cleared, the fresh sink starts from earliest and redelivers the message. Had
    // the cursor leaked, it would resume past the message and never deliver again.
    run_with_retries(async || {
        let n = mock.received_requests().await.unwrap_or_default().len();
        anyhow::ensure!(n > before, "expected redelivery after re-create, still {n}");
        Ok(())
    })
    .await?;

    Ok(())
}
