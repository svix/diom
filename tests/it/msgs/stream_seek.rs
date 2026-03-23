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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-earliest" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
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
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
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
        .post("msgs/stream/commit")
        .json(json!({
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Verify nothing left to consume
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r2["msgs"].assert_array().len(), 0);

    // Seek to earliest
    client
        .post("msgs/stream/seek")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — should replay all 3 messages
    let r3 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-earliest:t1",
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-latest" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages before any consumer exists
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-latest:t1",
            "msgs": [
                { "value": "old-a".as_bytes(), "key": "k1" },
                { "value": "old-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to latest (creates consumer group at the end of the stream)
    client
        .post("msgs/stream/seek")
        .json(json!({
            "topic": "ns-seek-latest:t1",
            "consumer_group": "cg1",
            "position": "latest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get nothing (all messages are before "latest")
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-latest:t1",
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
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-latest:t1",
            "msgs": [
                { "value": "new-a".as_bytes(), "key": "k1" },
                { "value": "new-b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get only the new messages
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-latest:t1",
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-offset" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-offset:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish 5 messages
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-offset:t1",
            "msgs": (0..5)
                .map(|i| json!({ "value": format!("msg-{i}").as_bytes(), "key": "k1" }))
                .collect::<Vec<_>>(),
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive to discover the partition topic
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-offset:t1",
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
        .post("msgs/stream/commit")
        .json(json!({
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": last_offset,
        }))
        .await?
        .expect(StatusCode::OK);

    // Seek to offset 2 (next-to-read semantics: will read from offset 2)
    client
        .post("msgs/stream/seek")
        .json(json!({
            "topic": partition_topic,
            "consumer_group": "cg1",
            "offset": 2,
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get messages starting at offset 2 (3 messages: 2, 3, 4)
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-offset:t1",
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-pt" }))
        .await?
        .expect(StatusCode::OK);

    // Offset-based seek on a bare topic (no ~partition) should fail
    client
        .post("msgs/stream/seek")
        .json(json!({
            "topic": "ns-seek-pt:t1",
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-topic" }))
        .await?
        .expect(StatusCode::OK);

    // Configure multiple partitions to ensure we can seek on a per-partition basis
    client
        .post("msgs/topic/configure")
        .json(json!({
            "topic": "ns-seek-topic:t1",
            "partitions": 16,
        }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-topic:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages to different partitions
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-topic:t1",
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
        .post("msgs/stream/seek")
        .json(json!({
            "topic": "ns-seek-topic:t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get all messages from all partitions
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-topic:t1",
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
        .post("msgs/stream/seek")
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-seek-lease" }))
        .await?
        .expect(StatusCode::OK);

    // Register consumer group
    client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-lease:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-seek-lease:t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — locks the partition
    let r1 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-lease:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].assert_array().len(), 2);

    // Verify partition is locked
    client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-lease:t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    // Seek to earliest — should clear the lease
    client
        .post("msgs/stream/seek")
        .json(json!({
            "topic": "ns-seek-lease:t1",
            "consumer_group": "cg1",
            "position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive again — should succeed (lease cleared) and replay messages
    let r2 = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "ns-seek-lease:t1",
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
