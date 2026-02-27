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
            "name": "ns1",
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

    // Each message should have a partition and offset
    for m in msgs {
        assert!(m["partition"].is_u64());
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
            "name": "ns-key",
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

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All messages with the same key must land in the same partition
    let partition = msgs[0]["partition"].as_u64().unwrap();
    for m in msgs {
        assert_eq!(m["partition"].as_u64().unwrap(), partition);
    }

    // Offsets should be sequential within the partition
    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);
    assert_eq!(offsets[2], offsets[1] + 1);

    Ok(())
}

#[tokio::test]
async fn publish_rejects_partition_topic() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("msgs/namespace/create")
        .json(json!({ "name": "ns-part" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("msgs/publish")
        .json(json!({
            "name": "ns-part",
            "topic": "mytopic~3",
            "msgs": [{ "value": "a".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

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
            "name": "does-not-exist",
            "topic": "topic",
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
            "name": "ns-kl",
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

    let msgs = response["msgs"].as_array().unwrap();
    assert_eq!(msgs.len(), 3);

    // All keyless messages in a single publish call land on the same partition
    let partition = msgs[0]["partition"].as_u64().unwrap();
    for m in msgs {
        assert_eq!(m["partition"].as_u64().unwrap(), partition);
    }

    // Offsets should be sequential
    let offsets: Vec<u64> = msgs.iter().map(|m| m["offset"].as_u64().unwrap()).collect();
    assert_eq!(offsets[1], offsets[0] + 1);
    assert_eq!(offsets[2], offsets[1] + 1);

    Ok(())
}
