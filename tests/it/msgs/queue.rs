use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};
use tokio::task::yield_now;

#[tokio::test]
async fn queue_receive_returns_published_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-queue" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-queue",
            "topic": "my-topic",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-queue",
            "topic": "my-topic",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 3);

    for m in msgs {
        assert!(m["msg_id"].as_str().is_some(), "msg_id should be present");
        assert!(!m["value"].is_null());
        assert!(!m["timestamp"].is_null());
    }

    Ok(())
}

#[tokio::test]
async fn queue_receive_leases_individual_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-lease" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-lease",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // First receive gets message "a"
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-lease",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs1 = r1["msgs"].assert_array();
    assert_eq!(msgs1.len(), 1);
    assert_eq!(msgs1[0]["value"], json!("a".as_bytes()));

    // Second receive gets message "b" (message "a" is leased, skipped)
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-lease",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].assert_array();
    assert_eq!(msgs2.len(), 1);
    assert_eq!(msgs2[0]["value"], json!("b".as_bytes()));

    // Third receive — all messages leased, should be empty
    let r3 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-lease",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r3["msgs"].assert_array().len(),
        0,
        "no messages available when all are leased"
    );

    Ok(())
}

#[tokio::test]
async fn queue_ack_prevents_redelivery() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-ack" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-ack",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive with a short lease
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-ack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 3);

    // Ack all messages
    let msg_ids: Vec<&str> = msgs.iter().map(|m| m["msg_id"].assert_str()).collect();
    client
        .post("v1.msgs.queue.ack")
        .json(json!({
            "namespace": "ns-q-ack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": msg_ids,
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for lease to expire
    time.fast_forward(Duration::from_millis(1500));

    // Receive again — all messages were acked, should get nothing
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-ack",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "acked messages should not be re-delivered"
    );

    Ok(())
}

#[tokio::test]
async fn unacked_messages_redelivered_after_lease_expiry() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-redeliver" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-redeliver",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive with a very short lease
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-redeliver",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].assert_array().len(), 2);

    // Don't ack — wait for lease to expire
    time.fast_forward(Duration::from_millis(1500));

    // Receive again — should get the same messages
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-redeliver",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r2["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        2,
        "un-acked messages should be re-delivered after lease expiry"
    );

    // Should have the same msg_ids as before
    let ids_r1: Vec<&str> = r1["msgs"]
        .assert_array()
        .iter()
        .map(|m| m["msg_id"].assert_str())
        .collect();
    let ids_r2: Vec<&str> = msgs.iter().map(|m| m["msg_id"].assert_str()).collect();
    assert_eq!(ids_r1, ids_r2);

    Ok(())
}

#[tokio::test]
async fn queue_receive_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "does-not-exist",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn queue_starts_from_earliest() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-earliest" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages BEFORE any queue consumer exists
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-earliest",
            "topic": "t1",
            "msgs": (0..5)
                .map(|i| json!({ "value": format!("msg-{i}").as_bytes(), "key": "k1" }))
                .collect::<Vec<_>>(),
        }))
        .await?
        .expect(StatusCode::OK);

    // First queue.receive should get ALL existing messages (unlike stream which starts from latest)
    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-earliest",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        5,
        "queue should deliver all existing messages (starts from earliest)"
    );

    Ok(())
}

#[tokio::test]
async fn partial_ack_redelivers_unacked() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-partial" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-partial",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive with short lease
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-partial",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 3);

    // Ack only the first and last messages (skip the middle one)
    let first_id = msgs[0]["msg_id"].assert_str();
    let last_id = msgs[2]["msg_id"].assert_str();
    client
        .post("v1.msgs.queue.ack")
        .json(json!({
            "namespace": "ns-q-partial",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [first_id, last_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for lease to expire
    time.fast_forward(Duration::from_millis(1500));

    // Receive again — only the un-acked middle message should be returned
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-partial",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].assert_array();
    assert_eq!(
        msgs2.len(),
        1,
        "only un-acked messages should be re-delivered"
    );

    // The re-delivered message should be the middle one (value "b")
    let middle_id = msgs[1]["msg_id"].assert_str();
    assert_eq!(msgs2[0]["msg_id"].assert_str(), middle_id);
    assert_eq!(msgs2[0]["value"], json!("b".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn concurrent_queue_consumers_no_overlap() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-concurrent" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-concurrent",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
                { "value": "d".as_bytes(), "key": "k1" },
                { "value": "e".as_bytes(), "key": "k1" },
                { "value": "f".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Consumer A receives 3 messages
    let r_a = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-concurrent",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 3,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].assert_array();
    assert_eq!(msgs_a.len(), 3);

    // Consumer B receives remaining 3 messages (first 3 are leased)
    let r_b = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-concurrent",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 3,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_b = r_b["msgs"].assert_array();
    assert_eq!(msgs_b.len(), 3);

    // Verify no overlap in msg_ids between the two consumers
    let ids_a: std::collections::HashSet<&str> =
        msgs_a.iter().map(|m| m["msg_id"].assert_str()).collect();
    let ids_b: std::collections::HashSet<&str> =
        msgs_b.iter().map(|m| m["msg_id"].assert_str()).collect();
    assert!(
        ids_a.is_disjoint(&ids_b),
        "consumers must receive different messages"
    );

    // Consumer C — all messages leased, should be empty
    let r_c = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-concurrent",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r_c["msgs"].assert_array().len(),
        0,
        "no messages available when all are leased"
    );

    Ok(())
}

#[tokio::test]
async fn queue_consumer_groups_independent() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-cg" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-cg",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Group A receives all messages
    let r_a = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-cg",
            "topic": "t1",
            "consumer_group": "group-a",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].assert_array();
    assert_eq!(msgs_a.len(), 3);

    // Ack all messages for group A
    let msg_ids_a: Vec<&str> = msgs_a.iter().map(|m| m["msg_id"].assert_str()).collect();
    client
        .post("v1.msgs.queue.ack")
        .json(json!({
            "namespace": "ns-q-cg",
            "topic": "t1",
            "consumer_group": "group-a",
            "msg_ids": msg_ids_a,
        }))
        .await?
        .expect(StatusCode::OK);

    // Group B should independently receive all the same messages
    let r_b = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-cg",
            "topic": "t1",
            "consumer_group": "group-b",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_b = r_b["msgs"].assert_array();
    assert_eq!(
        msgs_b.len(),
        3,
        "different consumer group should get all messages independently"
    );

    Ok(())
}

#[tokio::test]
async fn nack_sends_to_dlq() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-nack" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-nack",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive messages
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 2);

    // Nack all messages
    let msg_ids: Vec<&str> = msgs.iter().map(|m| m["msg_id"].assert_str()).collect();
    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": msg_ids,
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for original lease to expire
    time.fast_forward(Duration::from_millis(1500));

    // Receive again — nacked messages are in DLQ, should get nothing
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "nacked messages should be in DLQ and not re-delivered"
    );

    Ok(())
}

#[tokio::test]
async fn nack_then_redrive_makes_available() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-redrive" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-redrive",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive and nack
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-redrive",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msg_ids: Vec<&str> = r1["msgs"]
        .assert_array()
        .iter()
        .map(|m| m["msg_id"].assert_str())
        .collect();

    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-redrive",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": msg_ids,
        }))
        .await?
        .expect(StatusCode::OK);

    // Redrive DLQ
    client
        .post("v1.msgs.queue.redrive-dlq")
        .json(json!({
            "namespace": "ns-q-redrive",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — redriven messages should be available
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-redrive",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r2["msgs"].assert_array().len(),
        2,
        "redriven messages should be available again"
    );

    Ok(())
}

#[tokio::test]
async fn nack_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "does-not-exist",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": ["0:0"],
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn redrive_dlq_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.queue.redrive-dlq")
        .json(json!({
            "namespace": "does-not-exist",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn redrive_dlq_no_dlq_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-redrive-noop" }))
        .await?
        .expect(StatusCode::OK);

    // Publish a message to create the topic
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-redrive-noop",
            "topic": "t1",
            "msgs": [{ "value": "a".as_bytes(), "key": "k1" }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Redrive with no DLQ messages should succeed as a no-op
    client
        .post("v1.msgs.queue.redrive-dlq")
        .json(json!({
            "namespace": "ns-q-redrive-noop",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn configure_retry_schedule() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-cfg" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.queue.configure")
        .json(json!({
            "namespace": "ns-q-cfg",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [1000, 5000, 10000],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["retry_schedule"], json!([1000, 5000, 10000]));
    assert!(response["dlq_topic"].is_null());

    // Updating the config should overwrite
    let response2 = client
        .post("v1.msgs.queue.configure")
        .json(json!({
            "namespace": "ns-q-cfg",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [2000],
            "dlq_topic": "t1-dlq",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response2["retry_schedule"], json!([2000]));
    assert_eq!(response2["dlq_topic"], json!("t1-dlq"));

    Ok(())
}

#[tokio::test]
async fn nack_retries_before_dlq() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-retry" }))
        .await?
        .expect(StatusCode::OK);

    // Configure a single retry with 1s delay
    client
        .post("v1.msgs.queue.configure")
        .json(json!({
                "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [1000],
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "msgs": [{ "value": "a".as_bytes(), "key": "k1" }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive the message
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 500,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 1);
    let msg_id = msgs[0]["msg_id"].assert_str();

    // Nack — should schedule for retry, NOT DLQ
    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Immediately after nack, message should NOT be available (retry delay not elapsed)
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "message should be delayed, not immediately available"
    );

    // Wait for the retry delay to elapse
    time.fast_forward(Duration::from_millis(1500));

    // Message should now be available again (retried, not DLQ'd)
    let r3 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 500,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs3 = r3["msgs"].assert_array();
    assert_eq!(
        msgs3.len(),
        1,
        "message should be redelivered after retry delay"
    );
    assert_eq!(msgs3[0]["msg_id"].assert_str(), msg_id);

    // Nack again — retries exhausted, should go to DLQ
    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for any delay
    time.fast_forward(Duration::from_millis(1500));

    // Message should now be in DLQ — not available
    let r4 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-retry",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r4["msgs"].assert_array().len(),
        0,
        "message should be in DLQ after exhausting retries"
    );

    Ok(())
}

#[tokio::test]
async fn nack_with_dlq_topic_forwards() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-q-dlqfwd" }))
        .await?
        .expect(StatusCode::OK);

    // Configure with retry schedule and DLQ topic
    client
        .post("v1.msgs.queue.configure")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [1000],
            "dlq_topic": "t1-dlq",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "msgs": [{ "value": "a".as_bytes(), "key": "k1" }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive and nack (first retry)
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 500,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msg_id = r1["msgs"].assert_array()[0]["msg_id"].assert_str();

    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for retry delay
    time.fast_forward(Duration::from_millis(1500));

    // Receive the retried message and nack again (exhausts retries → forwards to DLQ topic)
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
            "lease_duration_ms": 500,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(r2["msgs"].assert_array().len(), 1);

    client
        .post("v1.msgs.queue.nack")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Original topic should have no messages available
    time.fast_forward(Duration::from_millis(1500));
    let r3 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r3["msgs"].assert_array().len(), 0);

    // The DLQ topic should have the message
    let r_dlq = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-q-dlqfwd",
            "topic": "t1-dlq",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let dlq_msgs = r_dlq["msgs"].assert_array();
    assert_eq!(dlq_msgs.len(), 1, "message should appear in DLQ topic");
    assert_eq!(dlq_msgs[0]["value"], json!("a".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn queue_receive_max_wait_returns_when_batch_filled() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-qwait-full" }))
        .await?
        .expect(StatusCode::OK);

    // Publish exactly 3 messages before receiving (queue starts from earliest)
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-qwait-full:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive with max_wait — batch is already full, should return immediately
    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-qwait-full:t1",
            "consumer_group": "test-cg",
            "batch_size": 3,
            "batch_wait_ms": 5000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 3, "should return all 3 messages");

    Ok(())
}

#[tokio::test]
async fn queue_receive_max_wait_returns_partial_batch_on_timeout() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-qwait-partial" }))
        .await?
        .expect(StatusCode::OK);

    // Publish only 2 messages — less than batch_size
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-qwait-partial:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Start a receive wanting batch_size=5 with a short wait
    let recv_client = client.clone();
    let recv_handle = tokio::spawn(async move {
        recv_client
            .post("v1.msgs.queue.receive")
            .json(json!({
                "topic": "ns-qwait-partial:t1",
                "consumer_group": "test-cg",
                "batch_size": 5,
                "batch_wait_ms": 500,
            }))
            .await
    });

    // Let the receive request reach the server
    yield_now().await;

    // Advance past the max_wait deadline
    time.fast_forward(Duration::from_millis(500));

    let response = recv_handle.await??.expect(StatusCode::OK).json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        2,
        "should return the 2 available messages after timeout"
    );

    Ok(())
}

#[tokio::test]
async fn queue_receive_max_wait_times_out_with_no_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-qtimeout" }))
        .await?
        .expect(StatusCode::OK);

    // Receive with a short max_wait — no messages will arrive
    let recv_client = client.clone();
    let recv_handle = tokio::spawn(async move {
        recv_client
            .post("v1.msgs.queue.receive")
            .json(json!({
                "topic": "ns-qtimeout:t1",
                "consumer_group": "test-cg",
                "batch_wait_ms": 500,
            }))
            .await
    });

    // Let the receive request reach the server
    yield_now().await;

    // Advance past the max_wait deadline
    time.fast_forward(Duration::from_millis(500));

    let response = recv_handle.await??.expect(StatusCode::OK).json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 0, "should return empty after timeout");

    Ok(())
}

/// Queue partitions are not locked by active leases (unlike stream). The max_wait
/// estimate must not skip partitions that have an existing cursor, otherwise consumers
/// would unnecessarily wait even when messages are available.
#[tokio::test]
async fn queue_receive_max_wait_does_not_skip_partitions_with_existing_cursor() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-qwait-cursor" }))
        .await?
        .expect(StatusCode::OK);

    // Publish 3 messages
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-qwait-cursor:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Consumer A leases the first 2 messages, creating a cursor on the partition
    let r1 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-qwait-cursor:t1",
            "consumer_group": "cg1",
            "batch_size": 2,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].assert_array().len(), 2);

    // Publish 3 more messages
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-qwait-cursor:t1",
            "msgs": [
                { "value": "d".as_bytes(), "key": "k1" },
                { "value": "e".as_bytes(), "key": "k1" },
                { "value": "f".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Consumer B calls receive with max_wait. The partition already has a cursor from
    // consumer A's receive, but queue partitions aren't locked — new messages should
    // be available immediately without waiting.
    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-qwait-cursor:t1",
            "consumer_group": "cg1",
            "batch_size": 3,
            "batch_wait_ms": 5000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    // Should get the 1 unleased message from the first batch + 3 new messages = 4
    // (or at least > 0 — the point is it didn't wait 5 seconds)
    let msgs = response["msgs"].assert_array();
    assert!(
        !msgs.is_empty(),
        "should return messages immediately, not wait for max_wait"
    );

    Ok(())
}
