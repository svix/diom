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

    // Second receive with the same CG should get nothing (messages are leased)
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
