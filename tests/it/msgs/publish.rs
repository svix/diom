use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn publish_to_topic() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns1" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns1",
            "topic": "my-topic",
            "msgs": [
                { "value": "hello".as_bytes() },
                { "value": "world".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let topics = response["topics"].assert_array();
    assert_eq!(topics.len(), 1);
    let topic = &topics[0];
    assert_eq!(
        topic["offset"].assert_u64() - topic["start_offset"].assert_u64(),
        2
    );

    // Each topic should have a partition
    for topic in topics {
        assert!(topic["topic"].assert_str().starts_with("my-topic~"));
    }

    Ok(())
}

#[tokio::test]
async fn publish_with_partition_key() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-key" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-key",
            "topic": "keyed-topic",
            "msgs": [
                { "value": "a".as_bytes(), "key": "user-123" },
                { "value": "b".as_bytes(), "key": "user-123" },
                { "value": "c".as_bytes(), "key": "user-123" },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let topics = response["topics"].assert_array();
    assert_eq!(topics.len(), 1);
    let topic = &topics[0];
    assert_eq!(
        topic["offset"].assert_u64() - topic["start_offset"].assert_u64(),
        3
    );

    // Each topic should have a partition
    for topic in topics {
        assert!(topic["topic"].assert_str().starts_with("keyed-topic~"));
    }

    Ok(())
}

#[tokio::test]
async fn publish_directly_to_partition() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-direct" }))
        .await?
        .expect(StatusCode::OK);

    // Configure the topic to have 4 partitions.
    client
        .post("v1.msgs.topic.configure")
        .json(json!({ "namespace": "ns-direct", "topic": "my-topic", "partitions": 4 }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-direct",
            "topic": "my-topic~2",
            "msgs": [
                { "value": "a".as_bytes() },
                { "value": "b".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let topics = response["topics"].assert_array();
    assert_eq!(topics.len(), 1);
    let topic = &topics[0];
    assert_eq!(
        topic["offset"].assert_u64() - topic["start_offset"].assert_u64(),
        2
    );

    // Each topic should have a partition
    for topic in topics {
        assert!(topic["topic"].assert_str().starts_with("my-topic~2"));
    }

    Ok(())
}

#[tokio::test]
async fn publish_rejects_out_of_range_partition() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-range" }))
        .await?
        .expect(StatusCode::OK);

    // Default topic has 1 partition (index 0 only).
    // Publishing to partition 1 should fail.
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-range",
            "topic": "my-topic~1",
            "msgs": [{ "value": "a".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn publish_rejects_malformed_partition_topic() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-bad" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-bad",
            "topic": "my-topic~abc",
            "msgs": [{ "value": "a".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn publish_to_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "does-not-exist",
            "topic": "topic",
            "msgs": [{ "value": "x".as_bytes() }],
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn publish_keyless_same_partition() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-kl" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-kl",
            "topic": "keyless-topic",
            "msgs": [
                { "value": "a".as_bytes() },
                { "value": "b".as_bytes() },
                { "value": "c".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let topics = response["topics"].assert_array();
    assert_eq!(topics.len(), 1);
    let topic = &topics[0];
    assert_eq!(
        topic["offset"].assert_u64() - topic["start_offset"].assert_u64(),
        3
    );

    // Each topic should have a partition
    for topic in topics {
        assert!(topic["topic"].assert_str().starts_with("keyless-topic~"));
    }

    Ok(())
}

#[tokio::test]
async fn publish_rejects_reused_idempotency_key() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-idem" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic",
            "idempotency_key": "same-request",
            "msgs": [{ "value": "first".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic",
            "idempotency_key": "same-request",
            "msgs": [{ "value": "second".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::CONFLICT);

    // Same idempotency key, different topic should succeed
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic2",
            "idempotency_key": "same-request",
            "msgs": [{ "value": "from-topic-2".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0]["value"], json!("first".as_bytes()));

    time.fast_forward(Duration::from_hours(1) + Duration::from_secs(1));

    // key resets after TTL
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-idem",
            "topic": "topic",
            "idempotency_key": "same-request",
            "msgs": [{ "value": "third".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn publish_with_idempotency_key_concurrent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-idem-conc" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-idem-conc",
            "topic": "topic",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK);

    // Fire 10 concurrent publishes with the same idempotency key.
    let mut handles = Vec::new();
    for _ in 0..10 {
        let c = client.clone();
        handles.push(tokio::spawn(async move {
            c.post("v1.msgs.publish")
                .json(json!({
                    "namespace": "ns-idem-conc",
                    "topic": "topic",
                    "idempotency_key": "same-concurrent-key",
                    "msgs": [{ "value": "once".as_bytes() }],
                }))
                .await
        }));
    }

    let mut ok = 0;
    for handle in handles {
        let resp = handle.await??;
        if matches!(resp.status(), StatusCode::OK) {
            ok += 1;
        } else {
            assert!(matches!(resp.status(), StatusCode::CONFLICT));
        }
    }

    assert_eq!(ok, 1);

    let response = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-idem-conc",
            "topic": "topic",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 1);

    Ok(())
}

#[tokio::test]
async fn publish_to_default_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    // No namespace creation — default namespace is auto-created.
    let response = client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "my-topic",
            "msgs": [
                { "value": "hello".as_bytes() },
                { "value": "world".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let topics = response["topics"].assert_array();
    assert_eq!(topics.len(), 1);
    let topic = &topics[0];
    assert_eq!(
        topic["offset"].assert_u64() - topic["start_offset"].assert_u64(),
        2
    );

    // Each topic should have a partition
    for topic in topics {
        assert!(topic["topic"].assert_str().starts_with("my-topic~"));
    }

    Ok(())
}

#[tokio::test]
async fn default_namespace_isolated_from_named() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "other" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer groups before publishing
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "topic": "shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "other",
            "topic": "shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish to default namespace (no prefix)
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "shared-name",
            "msgs": [{ "value": "default-msg".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish to "other" namespace
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "other",
            "topic": "shared-name",
            "msgs": [{ "value": "other-msg".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive from default — should only see 1 message
    let response = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "topic": "shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let default_msgs = response["msgs"].assert_array();
    assert_eq!(default_msgs.len(), 1);
    assert!(
        default_msgs[0]["topic"]
            .assert_str()
            .starts_with("shared-name~"),
        "default namespace response topics should not have a namespace prefix"
    );

    // Receive from "other" — should only see 1 message
    let response = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "other",
            "topic": "shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let other_msgs = response["msgs"].assert_array();
    assert_eq!(other_msgs.len(), 1);
    assert!(
        other_msgs[0]["topic"]
            .assert_str()
            .starts_with("shared-name~"),
        "other namespace messages should have other: prefix"
    );

    Ok(())
}
