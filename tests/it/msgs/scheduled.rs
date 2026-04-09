use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

/// scheduled message delivered after delay elapses but not before
#[tokio::test]
async fn scheduled_msg_delivered_after_delay() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-queue-deliver" }))
        .await?
        .expect(StatusCode::OK);

    // Publish a message with a 5-second delay
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-queue-deliver:t1",
            "msgs": [{ "value": "delayed".as_bytes(), "delay_ms": 5_000u64 }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Immediately — should get nothing from the queue
    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-hold:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r["msgs"].assert_array().len(),
        0,
        "scheduled message should not be delivered before delay elapses"
    );

    // Fast-forward but before the delay
    time.fast_forward(Duration::from_secs(3));
    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-hold:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r["msgs"].assert_array().len(),
        0,
        "scheduled message should not be delivered before delay elapses"
    );

    // Fast-forward past the delay
    time.fast_forward(Duration::from_secs(3));

    // Should now be deliverable
    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-deliver:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        1,
        "scheduled message should be delivered after delay"
    );
    assert_eq!(msgs[0]["value"], "delayed");

    Ok(())
}

/// mix of immediate and scheduled messages — immediate delivered first
#[tokio::test]
async fn immediate_msg_delivered_while_scheduled_msg_held() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-queue-mix" }))
        .await?
        .expect(StatusCode::OK);

    // Publish one immediate and one scheduled message (same partition key so same partition)
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-queue-mix:t1",
            "msgs": [
                { "value": "immediate1".as_bytes(), "key": "k1" },
                { "value": "delayed".as_bytes(), "key": "k1", "delay_ms": 10_000u64 },
                { "value": "immediate2".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Receive — should get only the immediate message
    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-mix:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r["msgs"].assert_array();
    assert_eq!(msgs.len(), 2, "only immediates message should be delivered");
    assert_eq!(msgs[0]["value"], "immediate1");
    assert_eq!(msgs[1]["value"], "immediate2");

    // Fast-forward and ack the immediate message to advance the cursor past it
    let msg_id = msgs[0]["msg_id"].assert_str();
    client
        .post("v1.msgs.queue.ack")
        .json(json!({
            "topic": "ns-sched-queue-mix:t1",
            "consumer_group": "cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);
    let msg_id = msgs[1]["msg_id"].assert_str();
    client
        .post("v1.msgs.queue.ack")
        .json(json!({
            "topic": "ns-sched-queue-mix:t1",
            "consumer_group": "cg",
            "msg_ids": [msg_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // Fast-forward past the delay
    time.fast_forward(Duration::from_secs(11));

    // Now the delayed message should be available
    let r2 = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-mix:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs2 = r2["msgs"].assert_array();
    assert_eq!(
        msgs2.len(),
        1,
        "delayed message should be delivered after delay"
    );
    assert_eq!(msgs2[0]["value"], "delayed");

    Ok(())
}

/// delay_ms=0 is treated as immediate
#[tokio::test]
async fn zero_delay_ms_is_immediate() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-zero-delay" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-zero-delay:t1",
            "msgs": [{ "value": "immediate".as_bytes(), "delay_ms": 0u64 }],
        }))
        .await?
        .expect(StatusCode::OK);

    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-zero-delay:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(
        r["msgs"].assert_array().len(),
        1,
        "delay_ms=0 should be delivered immediately"
    );

    Ok(())
}

/// Stream: scheduled message visible immediately with scheduled_at set
#[tokio::test]
async fn stream_sees_scheduled_msg_immediately() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-stream-vis" }))
        .await?
        .expect(StatusCode::OK);

    // Publish a message with a delay
    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-stream-vis:t1",
            "msgs": [{ "value": "future".as_bytes(), "delay_ms": 60_000u64 }],
        }))
        .await?
        .expect(StatusCode::OK);

    // Stream receive from earliest — should see the message immediately
    let r = client
        .post("v1.msgs.stream.receive")
        .json(json!({
            "topic": "ns-sched-stream-vis:t1",
            "consumer_group": "cg",
            "default_starting_position": "earliest",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r["msgs"].assert_array();
    assert_eq!(
        msgs.len(),
        1,
        "stream should see scheduled message immediately"
    );
    assert!(
        !msgs[0]["scheduled_at"].is_null(),
        "scheduled_at should be set on stream message"
    );

    Ok(())
}

/// scheduled_at is present on delivered message
#[tokio::test]
async fn queue_msg_has_scheduled_at_after_delivery() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-queue-field" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-queue-field:t1",
            "msgs": [{ "value": "x".as_bytes(), "delay_ms": 1_000u64 }],
        }))
        .await?
        .expect(StatusCode::OK);

    time.fast_forward(Duration::from_secs(2));

    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-queue-field:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r["msgs"].assert_array();
    assert_eq!(msgs.len(), 1);
    assert!(
        !msgs[0]["scheduled_at"].is_null(),
        "scheduled_at should be present on delivered queue message"
    );

    Ok(())
}

/// no scheduled_at on message published without delay
#[tokio::test]
async fn no_scheduled_at_on_immediate_message() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.create")
        .json(json!({ "name": "ns-sched-no-field" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "topic": "ns-sched-no-field:t1",
            "msgs": [{ "value": "x".as_bytes() }],
        }))
        .await?
        .expect(StatusCode::OK);

    let r = client
        .post("v1.msgs.queue.receive")
        .json(json!({
            "topic": "ns-sched-no-field:t1",
            "consumer_group": "cg",
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r["msgs"].assert_array();
    assert_eq!(msgs.len(), 1);
    assert!(
        msgs[0]["scheduled_at"].is_null(),
        "scheduled_at should be absent for messages published without delay"
    );

    Ok(())
}
