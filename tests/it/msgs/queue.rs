use std::time::Duration;

use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    server::{TestContext, start_server},
};
use tokio::time::sleep;

#[tokio::test]
async fn queue_receive_returns_published_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-queue" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-queue:my-topic",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-queue:my-topic",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-lease" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-lease:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // First receive gets message "a"
    let r1 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-lease:t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs1 = r1["msgs"].as_array().unwrap();
    assert_eq!(msgs1.len(), 1);
    assert_eq!(msgs1[0]["value"], json!("a".as_bytes()));

    // Second receive gets message "b" (message "a" is leased, skipped)
    let r2 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-lease:t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].as_array().unwrap();
    assert_eq!(msgs2.len(), 1);
    assert_eq!(msgs2[0]["value"], json!("b".as_bytes()));

    // Third receive — all messages leased, should be empty
    let r3 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-lease:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r3["msgs"].as_array().unwrap().len(),
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
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-ack" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-ack:t1",
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
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-ack:t1",
            "consumer_group": "test-cg",
            "lease_duration_millis": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // Ack all messages
    let msg_ids: Vec<&str> = msgs.iter().map(|m| m["msg_id"].as_str().unwrap()).collect();
    client
        .post("msgs/queue/ack")
        .json(json!({
            "topic": "ns-q-ack:t1",
            "consumer_group": "test-cg",
            "msg_ids": msg_ids,
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for lease to expire
    sleep(Duration::from_millis(1500)).await;

    // Receive again — all messages were acked, should get nothing
    let r2 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-ack:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].as_array().unwrap().len(),
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
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-redeliver" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-redeliver:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive with a very short lease
    let r1 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-redeliver:t1",
            "consumer_group": "test-cg",
            "lease_duration_millis": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].as_array().unwrap().len(), 2);

    // Don't ack — wait for lease to expire
    sleep(Duration::from_millis(1500)).await;

    // Receive again — should get the same messages
    let r2 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-redeliver:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r2["msgs"].as_array().unwrap();
    assert_eq!(
        msgs.len(),
        2,
        "un-acked messages should be re-delivered after lease expiry"
    );

    // Should have the same msg_ids as before
    let ids_r1: Vec<&str> = r1["msgs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["msg_id"].as_str().unwrap())
        .collect();
    let ids_r2: Vec<&str> = msgs.iter().map(|m| m["msg_id"].as_str().unwrap()).collect();
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
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "does-not-exist:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-earliest" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages BEFORE any queue consumer exists
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-earliest:t1",
            "msgs": (0..5)
                .map(|i| json!({ "value": format!("msg-{i}").as_bytes(), "key": "k1" }))
                .collect::<Vec<_>>(),
        }))
        .await?
        .expect(StatusCode::OK);

    // First queue.receive should get ALL existing messages (unlike stream which starts from latest)
    let response = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-earliest:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
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
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-partial" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-partial:t1",
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
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-partial:t1",
            "consumer_group": "test-cg",
            "lease_duration_millis": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // Ack only the first and last messages (skip the middle one)
    let first_id = msgs[0]["msg_id"].as_str().unwrap();
    let last_id = msgs[2]["msg_id"].as_str().unwrap();
    client
        .post("msgs/queue/ack")
        .json(json!({
            "topic": "ns-q-partial:t1",
            "consumer_group": "test-cg",
            "msg_ids": [first_id, last_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Wait for lease to expire
    sleep(Duration::from_millis(1500)).await;

    // Receive again — only the un-acked middle message should be returned
    let r2 = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-partial:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].as_array().unwrap();
    assert_eq!(
        msgs2.len(),
        1,
        "only un-acked messages should be re-delivered"
    );

    // The re-delivered message should be the middle one (value "b")
    let middle_id = msgs[1]["msg_id"].as_str().unwrap();
    assert_eq!(msgs2[0]["msg_id"].as_str().unwrap(), middle_id);
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-concurrent" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-concurrent:t1",
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
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-concurrent:t1",
            "consumer_group": "test-cg",
            "batch_size": 3,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].as_array().unwrap();
    assert_eq!(msgs_a.len(), 3);

    // Consumer B receives remaining 3 messages (first 3 are leased)
    let r_b = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-concurrent:t1",
            "consumer_group": "test-cg",
            "batch_size": 3,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_b = r_b["msgs"].as_array().unwrap();
    assert_eq!(msgs_b.len(), 3);

    // Verify no overlap in msg_ids between the two consumers
    let ids_a: std::collections::HashSet<&str> = msgs_a
        .iter()
        .map(|m| m["msg_id"].as_str().unwrap())
        .collect();
    let ids_b: std::collections::HashSet<&str> = msgs_b
        .iter()
        .map(|m| m["msg_id"].as_str().unwrap())
        .collect();
    assert!(
        ids_a.is_disjoint(&ids_b),
        "consumers must receive different messages"
    );

    // Consumer C — all messages leased, should be empty
    let r_c = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-concurrent:t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r_c["msgs"].as_array().unwrap().len(),
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-q-cg" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-q-cg:t1",
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
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-cg:t1",
            "consumer_group": "group-a",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].as_array().unwrap();
    assert_eq!(msgs_a.len(), 3);

    // Ack all messages for group A
    let msg_ids_a: Vec<&str> = msgs_a
        .iter()
        .map(|m| m["msg_id"].as_str().unwrap())
        .collect();
    client
        .post("msgs/queue/ack")
        .json(json!({
            "topic": "ns-q-cg:t1",
            "consumer_group": "group-a",
            "msg_ids": msg_ids_a,
        }))
        .await?
        .expect(StatusCode::OK);

    // Group B should independently receive all the same messages
    let r_b = client
        .post("msgs/queue/receive")
        .json(json!({
            "topic": "ns-q-cg:t1",
            "consumer_group": "group-b",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_b = r_b["msgs"].as_array().unwrap();
    assert_eq!(
        msgs_b.len(),
        3,
        "different consumer group should get all messages independently"
    );

    Ok(())
}
