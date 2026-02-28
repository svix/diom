use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns1" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns1:my-topic",
            "msgs": [
                { "value": "hello".as_bytes() },
                { "value": "world".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 2);

    // Each message should have a namespaced topic (with partition) and offset
    for m in msgs {
        assert!(m["topic"].as_str().unwrap().starts_with("ns1:my-topic~"));
        assert!(m["offset"].is_u64());
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-key" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-key:keyed-topic",
            "msgs": [
                { "value": "a".as_bytes(), "key": "user-123" },
                { "value": "b".as_bytes(), "key": "user-123" },
                { "value": "c".as_bytes(), "key": "user-123" },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All messages with the same key must land in the same partition topic
    let topic = msgs[0]["topic"].as_str().unwrap();
    assert!(topic.starts_with("ns-key:keyed-topic~"));
    for m in msgs {
        assert_eq!(m["topic"].as_str().unwrap(), topic);
    }

    // Offsets should be sequential within the partition
    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);
    assert_eq!(offsets[2], offsets[1] + 1);

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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-direct" }))
        .await?
        .expect(StatusCode::OK);

    // Configure the topic to have 4 partitions.
    client
        .post("msgs/topic/configure")
        .json(json!({ "topic": "ns-direct:my-topic", "partitions": 4 }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-direct:my-topic~2",
            "msgs": [
                { "value": "a".as_bytes() },
                { "value": "b".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 2);

    for m in msgs {
        assert_eq!(m["topic"].as_str().unwrap(), "ns-direct:my-topic~2");
    }

    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);

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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-range" }))
        .await?
        .expect(StatusCode::OK);

    // Default topic has 1 partition (index 0 only).
    // Publishing to partition 1 should fail.
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-range:my-topic~1",
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-bad" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-bad:my-topic~abc",
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
        .post("msgs/publish")
        .json(json!({
            "topic": "does-not-exist:topic",
            "msgs": [{ "value": "x".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::NOT_FOUND);

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
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-kl" }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("msgs/publish")
        .json(json!({
            "topic": "ns-kl:keyless-topic",
            "msgs": [
                { "value": "a".as_bytes() },
                { "value": "b".as_bytes() },
                { "value": "c".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All keyless messages in a single publish call land on the same partition topic
    let topic = msgs[0]["topic"].as_str().unwrap();
    assert!(topic.starts_with("ns-kl:keyless-topic~"));
    for m in msgs {
        assert_eq!(m["topic"].as_str().unwrap(), topic);
    }

    // Offsets should be sequential
    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);
    assert_eq!(offsets[2], offsets[1] + 1);

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
        .post("msgs/publish")
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

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 2);

    for m in msgs {
        let topic = m["topic"].as_str().unwrap();
        assert!(
            topic.starts_with("my-topic~"),
            "default namespace response topics should not have a namespace prefix: {topic}"
        );
        assert!(m["offset"].is_u64());
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
        .post("msgs/namespace/create")
        .json(json!({ "name": "other" }))
        .await?
        .expect(StatusCode::OK);

    // Publish to default namespace (no prefix)
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "shared-name",
            "msgs": [{ "value": "default-msg".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Publish to "other" namespace
    client
        .post("msgs/publish")
        .json(json!({
            "topic": "other:shared-name",
            "msgs": [{ "value": "other-msg".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive from default — should only see 1 message
    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let default_msgs = response["msgs"].as_array().unwrap();
    assert_eq!(default_msgs.len(), 1);
    assert!(
        default_msgs[0]["topic"]
            .as_str()
            .unwrap()
            .starts_with("shared-name~"),
        "default namespace response topics should not have a namespace prefix"
    );

    // Receive from "other" — should only see 1 message
    let response = client
        .post("msgs/stream/receive")
        .json(json!({
            "topic": "other:shared-name",
            "consumer_group": "cg1",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let other_msgs = response["msgs"].as_array().unwrap();
    assert_eq!(other_msgs.len(), 1);
    assert!(
        other_msgs[0]["topic"]
            .as_str()
            .unwrap()
            .starts_with("other:shared-name~"),
        "other namespace messages should have other: prefix"
    );

    Ok(())
}
