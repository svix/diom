#![allow(clippy::disallowed_types)] // serde_json::Value ok for tests
use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn seek_earliest_replays_all_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-earliest" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive all messages
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 3);

    // Commit to advance past all messages
    let partition_topic = msgs[0]["topic"].assert_str();
    let last_offset = msgs[2]["offset"].assert_u64();
    client
        .post("v1.msgs.stream.commit")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Verify nothing left to consume
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r2["msgs"].assert_array().len(), 0);

    // Seek to earliest
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — should replay all 3 messages
    let r3 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-earliest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r3["msgs"].assert_array().len(),
        3,
        "seek to earliest should replay all messages"
    );

    Ok(())
}

#[tokio::test]
async fn seek_latest_skips_existing() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-latest" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages before any consumer exists
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-latest",
            "topic": "t1",
            "msgs": [
                { "value": "old-a".as_bytes(), "key": "k1" },
                { "value": "old-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to latest (creates consumer group at the end of the stream)
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-latest",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "latest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get nothing (all messages are before "latest")
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-latest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r1["msgs"].assert_array().len(),
        0,
        "seek to latest should skip existing messages"
    );

    // Publish new messages
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-latest",
            "topic": "t1",
            "msgs": [
                { "value": "new-a".as_bytes(), "key": "k1" },
                { "value": "new-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get only the new messages
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-latest",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        2,
        "should receive only messages published after seek to latest"
    );

    Ok(())
}

#[tokio::test]
async fn seek_to_specific_offset() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-offset" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish 5 messages
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": "t1",
            "msgs": (0..5)
                .map(|i| json!({ "value": format!("msg-{i}").as_bytes(), "key": "k1" }))
                .collect::<Vec<_>>(),
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive to discover the partition topic
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 5);
    let partition_topic = msgs[0]["topic"].assert_str();

    // Commit past all messages
    let last_offset = msgs[4]["offset"].assert_u64();
    client
        .post("v1.msgs.stream.commit")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to offset 2 (next-to-read semantics: will read from offset 2)
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": 2,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get messages starting at offset 2 (3 messages: 2, 3, 4)
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-offset",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].assert_array();
    assert_eq!(
        msgs2.len(),
        3,
        "should get 3 messages starting from offset 2"
    );
    assert_eq!(msgs2[0]["offset"].assert_u64(), 2);

    Ok(())
}

#[tokio::test]
async fn seek_offset_requires_partition_topic() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-pt" }))
        .await?
        .expect(StatusCode::OK);

    // Offset-based seek on a bare topic (no ~partition) should fail
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-pt",
            "topic": "t1",
            "consumer_group": "cg1",
            "offset": 0,
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn seek_position_works_on_topic_level() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-topic" }))
        .await?
        .expect(StatusCode::OK);

    // Configure multiple partitions to ensure we can seek on a per-partition basis
    client
        .post("v1.msgs.topic.configure")
        .json(json!({
            "namespace": "ns-seek-topic",
            "topic": "t1",
            "partitions": 16,
        }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-topic",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages to different partitions
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-topic",
            "topic": "t1",
            "msgs": [
                { "value": "a1".as_bytes(), "key": "k1" },
                { "value": "a2".as_bytes(), "key": "k1" },
                { "value": "b1".as_bytes(), "key": "k2" },
                { "value": "b2".as_bytes(), "key": "k2" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to earliest on the topic level (applies to all partitions)
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-topic",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get all messages from all partitions
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-topic",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 10,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert!(
        msgs.len() >= 2,
        "should receive messages from at least one partition after topic-level seek"
    );

    Ok(())
}

#[tokio::test]
async fn seek_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "does-not-exist",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .ensure_not_found()?;

    Ok(())
}

#[tokio::test]
async fn seek_clears_lease() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-lease" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — locks the partition
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].assert_array().len(), 2);

    // Verify partition is locked
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    // Seek to earliest — should clear the lease
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — should succeed (lease cleared) and replay messages
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-lease",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r2["msgs"].assert_array().len(),
        2,
        "seek should clear lease and allow re-receive"
    );

    Ok(())
}

#[tokio::test]
async fn seek_to_timestamp_skips_older_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-ts" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "msgs": [
                { "value": "old-a".as_bytes(), "key": "k1" },
                { "value": "old-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Need a time gap for seeking to make sense
    time.fast_forward(Duration::from_secs(10));

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "msgs": [
                { "value": "new-a".as_bytes(), "key": "k1" },
                { "value": "new-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive all messages to discover the timestamp boundary
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 4);
    // The third message (index 2) is the first "new" one
    let new_msg_ts = msgs[2]["timestamp"].assert_u64();

    let partition_topic = msgs[0]["topic"].assert_str();
    let last_offset = msgs[3]["offset"].assert_u64();
    client
        .post("v1.msgs.stream.commit")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to the timestamp of the new messages
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "consumer_group": "cg1",
            "timestamp": new_msg_ts,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should only get the 2 new messages
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].assert_array();
    assert_eq!(
        msgs2.len(),
        2,
        "seek to timestamp should skip older messages"
    );
    assert_eq!(msgs2[0]["value"], json!("new-a".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn seek_to_timestamp_before_all_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-ts-early" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive and commit past all
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    let partition_topic = msgs[0]["topic"].assert_str();
    let last_offset = msgs[1]["offset"].assert_u64();
    client
        .post("v1.msgs.stream.commit")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to timestamp 0 (before all messages)
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": "t1",
            "consumer_group": "cg1",
            "timestamp": 0,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should replay all messages
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-early",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r2["msgs"].assert_array().len(),
        2,
        "seeking to timestamp 0 should replay all messages"
    );

    Ok(())
}

#[tokio::test]
async fn seek_to_timestamp_after_all_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-ts-future" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages (creates the topic)
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts-future",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to a far-future timestamp
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-ts-future",
            "topic": "t1",
            "consumer_group": "cg1",
            "timestamp": 253402207200000_u64,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get nothing
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-future",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r1["msgs"].assert_array().len(),
        0,
        "seeking past all messages should return nothing"
    );

    Ok(())
}

#[tokio::test]
async fn seek_to_timestamp_on_specific_partition_only_replays_that_partition() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-seek-ts-one-part" }))
        .await?
        .expect(StatusCode::OK);

    // Configure 2 partitions
    client
        .post("v1.msgs.topic.configure")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "partitions": 2,
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek consumer group to earliest on all partitions
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish old messages to both partitions (different keys route to different partitions)
    // With 2 partitions, "k-p0" and "k-p1" should land on different partitions
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "msgs": [
                { "value": "old-p0".as_bytes(), "key": "k-p0" },
                { "value": "old-p1".as_bytes(), "key": "k-p1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    time.fast_forward(Duration::from_secs(10));

    // Publish new messages to both partitions
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "msgs": [
                { "value": "new-p0".as_bytes(), "key": "k-p0" },
                { "value": "new-p1".as_bytes(), "key": "k-p1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive all messages to discover partition topics and timestamps
    let r1 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 100,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let all_msgs = r1["msgs"].assert_array();
    assert!(all_msgs.len() >= 2, "should have messages from partitions");

    // Group messages by partition topic
    let mut partitions: std::collections::HashMap<String, Vec<&serde_json::Value>> =
        std::collections::HashMap::new();
    for m in all_msgs {
        let pt = m["topic"].assert_str().to_string();
        partitions.entry(pt).or_default().push(m);
    }

    // We need at least 2 partitions with messages for this test to be meaningful
    // If hash routing put everything on one partition, the test isn't useful
    if partitions.len() < 2 {
        unreachable!("we misconfigured the partition keys");
    }

    // Pick two different partition topics
    let mut part_iter = partitions.iter();
    let (target_pt, target_msgs) = part_iter.next().unwrap();
    let (other_pt, _) = part_iter.next().unwrap();

    // Find the timestamp of the "new" message on the target partition
    let target_new_ts = target_msgs
        .iter()
        .map(|m| m["timestamp"].assert_u64())
        .max()
        .unwrap();

    // Commit all messages on both partitions
    for (pt, msgs) in &partitions {
        let last_offset = msgs.iter().map(|m| m["offset"].assert_u64()).max().unwrap();
        client
            .post("v1.msgs.stream.commit")
            .json(json!({
                "namespace": "ns-seek-ts-one-part",
                "topic": pt,
                "consumer_group": "cg1",
                "offset": last_offset,
            }))
            .await?
            .expect(StatusCode::OK);
    }

    // Verify nothing left to consume
    let r_empty = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r_empty["msgs"].assert_array().len(), 0);

    // Seek ONLY the target partition to the new timestamp
    client
        .post("v1.msgs.stream.seek")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": target_pt,
            "consumer_group": "cg1",
            "timestamp": target_new_ts,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should only get the new message(s) from the target partition.
    // The other partition should remain fully consumed (no messages returned).
    let r2 = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-seek-ts-one-part",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 100,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let replayed = r2["msgs"].assert_array();
    assert!(
        !replayed.is_empty(),
        "target partition should have replayed messages"
    );
    for m in replayed {
        assert_eq!(
            m["topic"].assert_str(),
            target_pt,
            "only the targeted partition should have replayed messages, not {other_pt}"
        );
    }

    Ok(())
}
