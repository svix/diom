use std::collections::HashSet;

use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn default_is_one_partition() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-def-part" }))
        .await?
        .expect(StatusCode::OK);

    // Publish messages with different keys — with 1 partition they all land on the same one.
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-def-part",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "alpha" },
                { "value": "b".as_bytes(), "key": "beta" },
                { "value": "c".as_bytes(), "key": "gamma" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-def-part",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All messages should be on the same single partition
    let topics: HashSet<&str> = msgs.iter().map(|m| m["topic"].as_str().unwrap()).collect();
    assert_eq!(topics.len(), 1, "all messages should be on one partition");
    assert!(topics.contains("t1~0"), "single partition should be t1~0");

    Ok(())
}

#[tokio::test]
async fn configure_topic_partitions() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-conf" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-conf",
            "topic": "t1",
            "partitions": 4,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["partitions"].as_u64().unwrap(), 4);

    // Publish messages with different keys to spread across partitions
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-conf",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "alpha" },
                { "value": "b".as_bytes(), "key": "beta" },
                { "value": "c".as_bytes(), "key": "gamma" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-conf",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All partition-level topics should be within t1~0..t1~3
    for m in msgs {
        let topic = m["topic"].as_str().unwrap();
        assert!(
            topic.starts_with("t1~"),
            "expected partition-level topic: {topic}"
        );
        let partition: u16 = topic.strip_prefix("t1~").unwrap().parse().unwrap();
        assert!(partition < 4, "partition {partition} should be < 4");
    }

    Ok(())
}

#[tokio::test]
async fn cannot_decrease_partitions() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-dec" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-dec",
            "topic": "t1",
            "partitions": 4,
        }))
        .await?
        .expect(StatusCode::OK);

    // Attempt to decrease — should be rejected
    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-dec",
            "topic": "t1",
            "partitions": 2,
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn configure_rejects_zero() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-zero" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-zero",
            "topic": "t1",
            "partitions": 0,
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn configure_rejects_over_max() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-max" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-max",
            "topic": "t1",
            "partitions": 65,
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn configure_nonexistent_namespace() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "does-not-exist",
            "topic": "t1",
            "partitions": 4,
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn receive_respects_configured_partitions() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-recv-conf" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/topic/configure")
        .json(json!({
            "name": "ns-recv-conf",
            "topic": "t1",
            "partitions": 16,
        }))
        .await?
        .expect(StatusCode::OK);

    // "k1" and "k2" hash to different partitions with 16 partitions
    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-recv-conf",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k2" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "name": "ns-recv-conf",
            "topic": "t1",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 2);

    // Messages should come from different partition-level topics
    let topics: HashSet<&str> = msgs.iter().map(|m| m["topic"].as_str().unwrap()).collect();
    assert_eq!(
        topics.len(),
        2,
        "k1 and k2 should route to different partitions"
    );

    for topic in &topics {
        assert!(
            topic.starts_with("t1~"),
            "expected partition-level topic: {topic}"
        );
        let partition: u16 = topic.strip_prefix("t1~").unwrap().parse().unwrap();
        assert!(partition < 16, "partition {partition} should be < 16");
    }

    Ok(())
}
