use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn queue_receive_skips_expired_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({
            "name": "ns-retention-q",
            "retention": { "period_ms": 5000 }
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-retention-q",
            "topic": "t1",
            "msgs": [
                { "value": "old".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    time.fast_forward(Duration::from_secs(6));

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-retention-q",
            "topic": "t1",
            "msgs": [
                { "value": "new".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-retention-q",
            "topic": "t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 1, "should only get the non-expired message");
    assert_eq!(msgs[0]["value"], json!("new".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn stream_receive_skips_expired_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({
            "name": "ns-retention-s",
            "retention": { "period_ms": 5000 }
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-retention-s",
            "topic": "t1",
            "msgs": [
                { "value": "old".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    time.fast_forward(Duration::from_secs(6));

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-retention-s",
            "topic": "t1",
            "msgs": [
                { "value": "new".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "namespace": "ns-retention-s",
            "topic": "t1",
            "consumer_group": "cg",
            "default_starting_position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(msgs.len(), 1, "should only get the non-expired message");
    assert_eq!(msgs[0]["value"], json!("new".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn no_retention_returns_all_messages() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-no-retention" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-no-retention",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    time.fast_forward(Duration::from_secs(3600));

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-no-retention",
            "topic": "t1",
            "msgs": [
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-no-retention",
            "topic": "t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        2,
        "without retention, all messages should be returned"
    );

    Ok(())
}

#[tokio::test]
async fn retention_partial_expiry() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({
            "name": "ns-partial",
            "retention": { "period_ms": 10_000 }
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-partial",
            "topic": "t1",
            "msgs": [
                { "value": "early-1".as_bytes(), "key": "k1" },
                { "value": "early-2".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Advance 7 seconds (within retention)
    time.fast_forward(Duration::from_secs(7));

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-partial",
            "topic": "t1",
            "msgs": [
                { "value": "later".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Advance 5 more seconds -- first batch is now 12s old (expired), second is 5s old (valid)
    time.fast_forward(Duration::from_secs(5));

    let response = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "namespace": "ns-partial",
            "topic": "t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = response["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        1,
        "only the non-expired message should be returned"
    );
    assert_eq!(msgs[0]["value"], json!("later".as_bytes()));

    Ok(())
}
