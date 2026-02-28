use std::collections::HashSet;

use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn stream_receive_returns_published_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-recv" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-recv",
            "topic": "my-topic",
            "msgs": [
                { "value": "a".as_bytes(), "key": "user-1" },
                { "value": "b".as_bytes(), "key": "user-1" },
                { "value": "c".as_bytes(), "key": "user-1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-recv",
            "topic": "my-topic",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    for m in msgs {
        assert!(m["offset"].is_u64());
        let topic = m["topic"].as_str().unwrap();
        assert!(
            topic.starts_with("my-topic~"),
            "topic should be partition-level: {topic}"
        );
        assert!(!m["value"].is_null());
        assert!(!m["timestamp"].is_null());
    }

    // Offsets should be sequential within the same partition
    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);
    assert_eq!(offsets[2], offsets[1] + 1);

    Ok(())
}

#[tokio::test]
async fn stream_receive_no_duplicates_within_lease() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-nodup" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-nodup",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // First receive gets the messages
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-nodup",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].as_array().unwrap().len(), 2);

    // Second receive with the same CG — partition is locked, returns empty
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-nodup",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r2["msgs"].as_array().unwrap().len(), 0);

    // Commit the first batch to unlock the partition
    let msgs = r1["msgs"].as_array().unwrap();
    let partition_topic = msgs[0]["topic"].as_str().unwrap().to_owned();
    let last_offset = msgs[1]["offset"].as_u64().unwrap();

    client
        .post("msgs/stream/commit")
        .json(json!({
            "name": "ns-nodup",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish more messages
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-nodup",
            "topic": "t1",
            "msgs": [
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Third receive gets only the new message
    let r3 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-nodup",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r3["msgs"].as_array().unwrap().len(), 1);

    Ok(())
}

#[tokio::test]
async fn different_consumer_groups_get_same_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-cg" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-cg",
            "topic": "t1",
            "msgs": [
                { "value": "x".as_bytes(), "key": "k1" },
                { "value": "y".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let r_a = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-cg",
            "topic": "t1",
            "consumer_group": "group-a",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let r_b = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-cg",
            "topic": "t1",
            "consumer_group": "group-b",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].as_array().unwrap();
    let msgs_b = r_b["msgs"].as_array().unwrap();

    assert_eq!(msgs_a.len(), 2);
    assert_eq!(msgs_b.len(), 2);

    // Both groups should get the same offsets
    let offsets_a: Vec<u64> = msgs_a
        .iter()
        .map(|m| m["offset"].as_u64().unwrap())
        .collect();
    let offsets_b: Vec<u64> = msgs_b
        .iter()
        .map(|m| m["offset"].as_u64().unwrap())
        .collect();
    assert_eq!(offsets_a, offsets_b);

    Ok(())
}

#[tokio::test]
async fn stream_receive_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "does-not-exist",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn stream_receive_with_defaults() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-def" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-def",
            "topic": "t1",
            "msgs": [{ "value": "hello".as_bytes(), "key": "k1" }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Call without specifying batch_size or lease_duration_millis
    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-def",
            "topic": "t1",
            "consumer_group": "cg-default",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 1);

    Ok(())
}

#[tokio::test]
async fn partition_locked_until_lease_expired_or_committed() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-lock" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-lock",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
                { "value": "d".as_bytes(), "key": "k1" },
                { "value": "e".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Consumer A receives a small batch — leases the partition
    let r_a = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-lock",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 2,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r_a["msgs"].as_array().unwrap().len(), 2);

    // Consumer B (same CG) — partition is locked, returns empty
    let r_b_locked = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-lock",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r_b_locked["msgs"].as_array().unwrap().len(), 0);

    // Consumer A commits — unlocks the partition
    let msgs_a = r_a["msgs"].as_array().unwrap();
    let partition_topic = msgs_a[0]["topic"].as_str().unwrap().to_owned();
    let last_offset = msgs_a[1]["offset"].as_u64().unwrap();

    client
        .post("msgs/stream/commit")
        .json(json!({
            "name": "ns-lock",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Now consumer B can receive the remaining messages
    let r_b = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-lock",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r_b["msgs"].as_array().unwrap().len(),
        3,
        "after commit, remaining messages should be available"
    );

    Ok(())
}

#[tokio::test]
async fn concurrent_consumers_receive_from_different_partitions() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-concurrent" }))
        .await?
        .expect(StatusCode::OK);

    // Default is 1 partition; configure more so "k1" and "k2" hash to different ones.
    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-concurrent",
            "topic": "t1",
            "partitions": 16,
        }))
        .await?
        .expect(StatusCode::OK);

    // "k1" and "k2" hash to different partitions via djb2
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-concurrent",
            "topic": "t1",
            "msgs": [
                { "value": "a1".as_bytes(), "key": "k1" },
                { "value": "a2".as_bytes(), "key": "k1" },
                { "value": "a3".as_bytes(), "key": "k1" },
                { "value": "a4".as_bytes(), "key": "k1" },
                { "value": "a5".as_bytes(), "key": "k1" },
                { "value": "b1".as_bytes(), "key": "k2" },
                { "value": "b2".as_bytes(), "key": "k2" },
                { "value": "b3".as_bytes(), "key": "k2" },
                { "value": "b4".as_bytes(), "key": "k2" },
                { "value": "b5".as_bytes(), "key": "k2" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Consumer A: receives with batch_size=5, should get one partition's worth
    let r_a = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-concurrent",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 5,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_a = r_a["msgs"].as_array().unwrap();
    assert_eq!(msgs_a.len(), 5);

    // Consumer B: same CG, should get the other partition
    let r_b = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-concurrent",
            "topic": "t1",
            "consumer_group": "cg1",
            "batch_size": 10,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs_b = r_b["msgs"].as_array().unwrap();
    assert_eq!(msgs_b.len(), 5);

    // Each consumer should have messages from a single (different) partition
    let topics_a: HashSet<&str> = msgs_a
        .iter()
        .map(|m| m["topic"].as_str().unwrap())
        .collect();
    let topics_b: HashSet<&str> = msgs_b
        .iter()
        .map(|m| m["topic"].as_str().unwrap())
        .collect();

    assert_eq!(
        topics_a.len(),
        1,
        "consumer A should read from one partition"
    );
    assert_eq!(
        topics_b.len(),
        1,
        "consumer B should read from one partition"
    );
    assert!(
        topics_a.is_disjoint(&topics_b),
        "consumers must read from different partitions"
    );

    Ok(())
}

#[tokio::test]
async fn commit_then_receive_no_duplicates() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-commit" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-commit",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive all 3 messages
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-commit",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // Extract partition-level topic and last offset in the batch
    let partition_topic = msgs[0]["topic"].as_str().unwrap().to_owned();
    let last_offset = msgs[2]["offset"].as_u64().unwrap();

    // Commit the last offset we processed
    client
        .post("msgs/stream/commit")
        .json(json!({
            "name": "ns-commit",
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — should get nothing (all committed past leases)
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-commit",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r2["msgs"].as_array().unwrap().len(), 0);

    // Publish more
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-commit",
            "topic": "t1",
            "msgs": [
                { "value": "d".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — only the new message
    let r3 = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-commit",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r3["msgs"].as_array().unwrap().len(), 1);

    Ok(())
}

#[tokio::test]
async fn commit_requires_partition_topic() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-commit-pt" }))
        .await?
        .expect(StatusCode::OK);

    // Base topic (no ~partition suffix) should be rejected at deserialization
    client
        .post("msgs/stream/commit")
        .json(json!({
            "name": "ns-commit-pt",
            "topic": "t1",
            "consumer_group": "cg1",
            "offset": 0,
        }))
        .await?
        .expect(StatusCode::UNPROCESSABLE_ENTITY);

    Ok(())
}

#[tokio::test]
async fn commit_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/stream/commit")
        .json(json!({
            "name": "does-not-exist",
            "topic": "t1~0",
            "consumer_group": "cg1",
            "offset": 0,
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn concurrent_receives_same_cg_no_overlap() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-race" }))
        .await?
        .expect(StatusCode::OK);

    // Single partition (default) — all messages land on partition 0.
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-race",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Fire 20 concurrent receives for the same consumer group.
    let mut handles = Vec::new();
    for _ in 0..20 {
        let c = client.clone();
        handles.push(tokio::spawn(async move {
            c.post("msgs/stream/receive")
                .json(json!({
                    "name": "ns-race",
                    "topic": "t1",
                    "consumer_group": "cg1",
                }))
                .await
                .unwrap()
        }));
    }

    let mut total_msgs = 0usize;
    let mut ok_count = 0usize;
    for handle in handles {
        let resp = handle.await.unwrap();
        match resp.status() {
            StatusCode::OK => {
                let body = resp.json();
                let msgs = body["msgs"].as_array().unwrap();
                total_msgs += msgs.len();
                ok_count += 1;
            }
            StatusCode::BAD_REQUEST => {
                // Partition locked — expected for losers of the race.
            }
            other => panic!("unexpected status: {other}"),
        }
    }

    // Exactly one consumer should have received the 3 messages.
    assert_eq!(
        total_msgs, 3,
        "messages must not be duplicated across consumers"
    );
    assert_eq!(
        ok_count, 1,
        "exactly one consumer should win the single partition"
    );

    Ok(())
}
