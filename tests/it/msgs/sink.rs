#![allow(clippy::disallowed_types)]
use std::time::Duration;

use diom_core::types::NonZeroDurationMs;
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    retry::run_with_retries,
    server::{TestServerBuilder, start_server},
};
use testcontainers::{
    GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
};
use testcontainers_modules::{kafka::apache, testcontainers::runners::AsyncRunner};
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

// ---------------------------------------------------------------------------
// Svix sink
// ---------------------------------------------------------------------------

const SVIX_SINK_TOKEN: &str = "testsk_ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.eu";

#[tokio::test]
async fn test_svix_sink_configure() -> TestResult {
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
            "consumer_group": "svix_sink",
            "config": {
                "type": "svix",
                "data": {
                    "token": SVIX_SINK_TOKEN,
                    "app_id": "${headers.org_id}",
                    "event_type": "order.created",
                },
            },
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "orders");
    assert_eq!(response["consumer_group"], "svix_sink");

    Ok(())
}

#[tokio::test]
async fn test_svix_sink_list_obfuscates_token() -> TestResult {
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
            "consumer_group": "svix_sink",
            "config": {
                "type": "svix",
                "data": {
                    "token": SVIX_SINK_TOKEN,
                    "app_id": "app_1",
                    "event_type": "order.created",
                },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.sink.list")
        .json(json!({ "topic": "orders" }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let data = response["data"].as_array().expect("data array");
    assert_eq!(data.len(), 1);
    assert_eq!(data[0]["consumer_group"], "svix_sink");

    // The non-secret fields come back verbatim, but the token is obfuscated (first 12 and last 4
    // chars kept) so the secret is not recoverable from a list call.
    assert_eq!(data[0]["config"]["type"], "svix");
    assert_eq!(data[0]["config"]["data"]["app_id"], "app_1");
    assert_eq!(data[0]["config"]["data"]["token"], "testsk_ABCDE...9.eu");

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
async fn test_svix_sink_forwards_published_messages() -> TestResult {
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
            "consumer_group": "svix_sink",
            "default_starting_position": "earliest",
            "config": {
                "type": "svix",
                "data": {
                    "token": SVIX_SINK_TOKEN,
                    "server_url": mock.uri(),
                    "app_id": "${headers.org_id}",
                    "event_type": "order.${headers.kind}",
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
                {
                    "value": json!({ "id": 1 }).to_string().as_bytes(),
                    "headers": { "org_id": "acme", "kind": "created" },
                },
                {
                    "value": json!({ "id": 2 }).to_string().as_bytes(),
                    "headers": { "org_id": "acme", "kind": "created" },
                },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    run_with_retries(async || {
        let reqs = mock.received_requests().await.unwrap_or_default();
        let deliveries: Vec<_> = reqs
            .iter()
            .filter(|r| r.url.path() == "/api/v1/app/acme/msg/")
            .collect();
        anyhow::ensure!(
            deliveries.len() >= 2,
            "expected >= 2 deliveries to the Svix app path, got {}",
            deliveries.len()
        );

        anyhow::ensure!(
            deliveries.iter().all(|r| r.method == "POST"),
            "each delivery should be a POST"
        );
        anyhow::ensure!(
            deliveries.iter().all(|r| r
                .headers
                .get("authorization")
                .is_some_and(|v| v == &format!("Bearer {SVIX_SINK_TOKEN}"))),
            "each request should carry the bearer token"
        );
        anyhow::ensure!(
            deliveries.iter().all(|r| r
                .headers
                .get("content-type")
                .is_some_and(|v| v.as_bytes().starts_with(b"application/json"))),
            "each request should be sent as JSON"
        );

        // The body wraps the message value as `payload` under the templated `eventType`.
        for r in &deliveries {
            let body: serde_json::Value = serde_json::from_slice(&r.body)
                .map_err(|e| anyhow::anyhow!("body was not JSON: {e}"))?;
            anyhow::ensure!(
                body["eventType"] == "order.created",
                "unexpected eventType: {body}"
            );
            anyhow::ensure!(
                body["payload"]["id"].is_number(),
                "payload should carry the message value: {body}"
            );
        }
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

// ---------------------------------------------------------------------------
// Kafka sink
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_kafka_sink_configure() -> TestResult {
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
            "consumer_group": "kafka_sink",
            "config": {
                "type": "kafka",
                "data": {
                    "bootstrap_servers": "localhost:9092",
                    "topic": "orders-out",
                },
            },
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["topic"], "orders");
    assert_eq!(response["consumer_group"], "kafka_sink");

    Ok(())
}

#[tokio::test]
async fn test_kafka_sink_configure_rejects_invalid_auth() -> TestResult {
    let ctx = start_server().await;

    ctx.client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "default" }))
        .await?
        .expect(StatusCode::OK);

    // A SASL mechanism without credentials is incoherent and must be rejected up front.
    let response = ctx
        .client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "kafka_bad_auth",
            "config": {
                "type": "kafka",
                "data": {
                    "bootstrap_servers": "localhost:9092",
                    "topic": "orders-out",
                    "security": {
                        "security_protocol": "sasl-ssl",
                        "sasl_mechanism": "scram-sha256",
                    },
                },
            },
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY)
        .json();

    assert_eq!(response["type"], "invalid-input");
    assert_eq!(response["code"], "invalid-data");

    Ok(())
}

#[tokio::test]
async fn test_kafka_sink_list_masks_auth_secrets() -> TestResult {
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
            "consumer_group": "kafka_sink",
            "config": {
                "type": "kafka",
                "data": {
                    "bootstrap_servers": "localhost:9092",
                    "topic": "orders-out",
                    "security": {
                        "security_protocol": "sasl-ssl",
                        "sasl_mechanism": "scram-sha256",
                        "sasl_username": "user",
                        "sasl_password": "supersecretpassword123456",
                        "ssl_certificate_pem": "-----BEGIN CERTIFICATE-----abc-----END CERTIFICATE-----",
                        "ssl_key_pem": "-----BEGIN PRIVATE KEY-----abc-----END PRIVATE KEY-----",
                    },
                },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    let response = ctx
        .client
        .post("v1.msgs.sink.list")
        .json(json!({ "topic": "orders" }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let security = &response["data"][0]["config"]["data"]["security"];
    // The password is obfuscated (first 12 and last 4 chars kept), key material is fully redacted,
    // and the non-secret username comes back verbatim.
    assert_eq!(security["sasl_username"], "user");
    assert_eq!(security["sasl_password"], "supersecretp...3456");
    assert_eq!(security["ssl_key_pem"], "...");

    Ok(())
}

// Requires Docker. Boots a real Kafka broker in a container and asserts the sink produces the
// published messages to the target Kafka topic.
#[tokio::test]
async fn test_kafka_sink_forwards_published_messages() -> TestResult {
    let broker = apache::Kafka::default().start().await?;
    let bootstrap = format!(
        "{}:{}",
        broker.get_host().await?,
        broker.get_host_port_ipv4(apache::KAFKA_PORT).await?
    );

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

    // The sink forwards each message to `topic` on the target cluster, using the templated `key`.
    ctx.client
        .post("v1.msgs.sink.configure")
        .json(json!({
            "topic": "orders",
            "consumer_group": "kafka_sink",
            "default_starting_position": "earliest",
            "config": {
                "type": "kafka",
                "data": {
                    "bootstrap_servers": bootstrap,
                    "topic": "orders-out",
                    "key": "${headers.org_id}",
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

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap)
        .set("group.id", "kafka-sink-test-consumer")
        .set("auto.offset.reset", "earliest")
        .create()?;
    consumer.subscribe(&["orders-out"])?;

    let deadline = std::time::Instant::now() + Duration::from_secs(60);
    let mut received: Vec<(String, Option<String>)> = Vec::new();
    while received.len() < 2 && std::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), consumer.recv()).await {
            Ok(Ok(msg)) => {
                let value = String::from_utf8_lossy(msg.payload().unwrap_or_default()).into_owned();
                let key = msg.key().map(|k| String::from_utf8_lossy(k).into_owned());
                received.push((value, key));
            }
            Ok(Err(_)) => tokio::time::sleep(Duration::from_millis(250)).await,
            Err(_elapsed) => {}
        }
    }
    assert_eq!(received.len(), 2, "expected 2 records, got {received:?}");

    let values: Vec<&str> = received.iter().map(|(v, _)| v.as_str()).collect();
    assert!(values.contains(&"hello"), "missing hello: {values:?}");
    assert!(values.contains(&"world"), "missing world: {values:?}");
    assert!(
        received.iter().all(|(_, k)| k.as_deref() == Some("acme")),
        "each record should carry the templated key: {received:?}"
    );

    Ok(())
}

// Requires Docker. Boots a SASL-enabled Redpanda broker and asserts the sink authenticates with
// SCRAM-SHA-256 and delivers to the target topic. The `testcontainers-modules` kafka module is
// PLAINTEXT-only, so the broker is a hand-rolled GenericImage.
#[tokio::test]
async fn test_kafka_sink_forwards_with_sasl() -> TestResult {
    const SASL_USER: &str = "superuser";
    const SASL_PASS: &str = "secretpassword";
    // Fixed host port so the broker can advertise a host-reachable address up front.
    const HOST_PORT: u16 = 39092;

    let _broker = GenericImage::new("redpandadata/redpanda", "v24.2.7")
        .with_exposed_port(9092.tcp())
        .with_wait_for(WaitFor::message_on_stderr("Successfully started Redpanda!"))
        .with_mapped_port(HOST_PORT, 9092.tcp())
        .with_env_var("RP_BOOTSTRAP_USER", format!("{SASL_USER}:{SASL_PASS}"))
        .with_cmd(vec![
            "redpanda".to_string(),
            "start".to_string(),
            "--overprovisioned".to_string(),
            "--smp".to_string(),
            "1".to_string(),
            "--memory".to_string(),
            "1G".to_string(),
            "--reserve-memory".to_string(),
            "0M".to_string(),
            "--node-id".to_string(),
            "0".to_string(),
            "--check=false".to_string(),
            "--kafka-addr".to_string(),
            "PLAINTEXT://0.0.0.0:9092".to_string(),
            format!("--advertise-kafka-addr=PLAINTEXT://127.0.0.1:{HOST_PORT}"),
            "--set".to_string(),
            "redpanda.enable_sasl=true".to_string(),
            // The bootstrap user must be a superuser to be authorized, and the sink relies on topic
            // auto-creation when it first produces.
            "--set".to_string(),
            "redpanda.superusers=['superuser']".to_string(),
            "--set".to_string(),
            "redpanda.auto_create_topics_enabled=true".to_string(),
        ])
        .start()
        .await?;
    let bootstrap = format!("127.0.0.1:{HOST_PORT}");

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
            "consumer_group": "kafka_sink",
            "default_starting_position": "earliest",
            "config": {
                "type": "kafka",
                "data": {
                    "bootstrap_servers": bootstrap,
                    "topic": "orders-out",
                    "security": {
                        "security_protocol": "sasl-plaintext",
                        "sasl_mechanism": "scram-sha256",
                        "sasl_username": SASL_USER,
                        "sasl_password": SASL_PASS,
                    },
                },
            },
        }))
        .await?
        .expect(StatusCode::OK);

    ctx.client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "orders",
            "msgs": [ { "value": "authed".as_bytes() } ],
        }))
        .await?
        .expect(StatusCode::OK);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap)
        .set("group.id", "kafka-sasl-test-consumer")
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", "SASL_PLAINTEXT")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("sasl.username", SASL_USER)
        .set("sasl.password", SASL_PASS)
        .create()?;
    consumer.subscribe(&["orders-out"])?;

    // Kept under the nextest slow-timeout so a genuine failure asserts (and drops the container)
    // rather than being killed mid-run and leaking the fixed host port.
    let deadline = std::time::Instant::now() + Duration::from_secs(25);
    let mut received: Vec<String> = Vec::new();
    while received.is_empty() && std::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_secs(10), consumer.recv()).await {
            Ok(Ok(msg)) => {
                received
                    .push(String::from_utf8_lossy(msg.payload().unwrap_or_default()).into_owned());
            }
            Ok(Err(_)) => tokio::time::sleep(Duration::from_millis(250)).await,
            Err(_elapsed) => {}
        }
    }
    assert_eq!(received, vec!["authed".to_string()]);

    Ok(())
}
