use std::time::Duration;

use serde_json::json;
use test_utils::{StatusCode, TestResult};

#[tokio::test]
async fn create_stream_upserts() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    let response = client
        .post("stream/create")
        .json(json!({
            "name": "test-stream",
            "maxByteSize": 1024,
            "retentionPeriodSeconds": 9999
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let ts = &response["createdAt"];

    assert_eq!(
        response,
        json!({
            "name": "test-stream",
            "maxByteSize": 1024,
            "retentionPeriodSeconds": 9999,
            "createdAt": ts,
            "updatedAt": ts,
        })
    );

    let update = client
        .post("stream/create")
        .json(json!({
            "name": "test-stream",
            "maxByteSize": null,
            "retentionPeriodSeconds": null
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        update,
        json!({
            "name": "test-stream",
            "maxByteSize": null,
            "retentionPeriodSeconds": null,
            "createdAt": ts,
            "updatedAt": &update["updatedAt"],
        })
    );

    Ok(())
}

#[tokio::test]
async fn stream_append_and_locking_consumption() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    let _stream = client
        .post("stream/create")
        .json(json!({
            "name": "test-stream",
            "maxByteSize": 1024,
            "retentionPeriodSeconds": 9999
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    client
        .post("stream/append")
        .json(json!({
            "name": "test-stream",
            "msgs": [
                {"payload": [1, 2], "headers": {"msg": "1"}},
                {"payload": [3, 4], "headers": {"msg": "2"}},
                {"payload": [5, 6], "headers": {"msg": "3"}},
            ]
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("stream/append")
        .json(json!({
            "name": "test-stream",
            "msgs": [
                {"payload": [7, 8], "headers": {"msg": "4"}},
                {"payload": [9, 10], "headers": {"msg": "5"}},
            ]
        }))
        .await?
        .expect(StatusCode::OK);

    // Fetch first batch of 3 messages
    let fetch1 = client
        .post("stream/fetch-locking")
        .json(json!({
            "name": "test-stream",
            "consumerGroup": "test-group",
            "batchSize": 3,
            "visibilityTimeoutSeconds": 3600
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs1 = fetch1["msgs"].as_array().unwrap();
    assert_eq!(msgs1.len(), 3);
    assert_eq!(
        msgs1[0],
        json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs1[0]["timestamp"]})
    );
    assert_eq!(
        msgs1[1],
        json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs1[1]["timestamp"]})
    );
    assert_eq!(
        msgs1[2],
        json!({"id": 2, "payload": [5, 6], "headers": {"msg": "3"}, "timestamp": msgs1[2]["timestamp"]})
    );

    // Ack the first batch
    let max_msg_id = &msgs1[2]["id"];
    client
        .post("stream/ack")
        .json(json!({
            "name": "test-stream",
            "consumerGroup": "test-group",
            "maxMsgId": max_msg_id
        }))
        .await?
        .expect(StatusCode::OK);

    // Fetch second batch - should get remaining 2 messages
    let fetch2 = client
        .post("stream/fetch-locking")
        .json(json!({
            "name": "test-stream",
            "consumerGroup": "test-group",
            "batchSize": 3,
            "visibilityTimeoutSeconds": 3600
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = fetch2["msgs"].as_array().unwrap();
    assert_eq!(msgs2.len(), 2);
    assert_eq!(
        msgs2[0],
        json!({"id": 3, "payload": [7, 8], "headers": {"msg": "4"}, "timestamp": msgs2[0]["timestamp"]})
    );
    assert_eq!(
        msgs2[1],
        json!({"id": 4, "payload": [9, 10], "headers": {"msg": "5"}, "timestamp": msgs2[1]["timestamp"]})
    );

    Ok(())
}

#[tokio::test]
async fn stream_visibility_timeout() -> TestResult {
    let (client, _server_handle) = super::start_server().await;

    let _stream = client
        .post("stream/create")
        .json(json!({
            "name": "test-stream",
            "maxByteSize": 1024,
            "retentionPeriodSeconds": 9999
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    client
        .post("stream/append")
        .json(json!({
            "name": "test-stream",
            "msgs": [
                {"payload": [1, 2], "headers": {"msg": "1"}},
                {"payload": [3, 4], "headers": {"msg": "2"}},
            ]
        }))
        .await?
        .expect(StatusCode::OK);

    // Fetch messages with a short visibility timeout (1 second)
    let fetch1 = client
        .post("stream/fetch-locking")
        .json(json!({
            "name": "test-stream",
            "consumerGroup": "test-group",
            "batchSize": 3,
            "visibilityTimeoutSeconds": 1
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs1 = fetch1["msgs"].as_array().unwrap();
    assert_eq!(msgs1.len(), 2);
    assert_eq!(
        msgs1[0],
        json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs1[0]["timestamp"]})
    );
    assert_eq!(
        msgs1[1],
        json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs1[1]["timestamp"]})
    );

    // Wait for the visibility timeout to expire
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Fetch again - should get the same messages since they weren't acked
    let fetch2 = client
        .post("stream/fetch-locking")
        .json(json!({
            "name": "test-stream",
            "consumerGroup": "test-group",
            "batchSize": 3,
            "visibilityTimeoutSeconds": 1
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = fetch2["msgs"].as_array().unwrap();
    assert_eq!(msgs2.len(), 2);
    assert_eq!(
        msgs2[0],
        json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs2[0]["timestamp"]})
    );
    assert_eq!(
        msgs2[1],
        json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs2[1]["timestamp"]})
    );

    Ok(())
}
