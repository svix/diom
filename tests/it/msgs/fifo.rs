use std::time::Duration;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn fifo_blocks_same_key_while_head_leased() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-hol" }))
        .await?
        .expect(StatusCode::OK);

    // Two messages sharing key "k1" land in the same partition, in order.
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-hol",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // First receive (batch_size 1) leases the head "a" and locks key "k1".
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-hol",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs1 = r1["msgs"].assert_array();
    assert_eq!(msgs1.len(), 1);
    assert_eq!(msgs1[0]["value"], json!("a".as_bytes()));
    assert_eq!(msgs1[0]["key"], json!("k1"), "the message key is surfaced");
    let head_id = msgs1[0]["msg_id"].assert_str().to_owned();

    // A second, concurrent receive gets nothing for "k1" while "a" is in-flight — even though "b"
    // is unleased and available. (This is exactly where queue mode would return "b".)
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-hol",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "key k1 is leased; no further k1 messages until the head is acked"
    );

    // Acking the head releases the key; "b" becomes deliverable.
    client
        .post("v1.msgs.fifo.ack")
        .json(json!({
            "namespace": "ns-fifo-hol",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [head_id],
        }))
        .await?
        .expect(StatusCode::OK);

    let r3 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-hol",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs3 = r3["msgs"].assert_array();
    assert_eq!(msgs3.len(), 1, "the key unlocks after its head is acked");
    assert_eq!(msgs3[0]["value"], json!("b".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn fifo_single_receive_returns_same_key_run_in_order() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-run" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-run",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
                { "value": "c".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // A generous batch pulls the entire k1 run at once, in offset order.
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-run",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 10,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    let msgs = r1["msgs"].assert_array();
    assert_eq!(msgs.len(), 3, "the whole same-key run is delivered in one call");
    assert_eq!(msgs[0]["value"], json!("a".as_bytes()));
    assert_eq!(msgs[1]["value"], json!("b".as_bytes()));
    assert_eq!(msgs[2]["value"], json!("c".as_bytes()));
    for m in msgs {
        assert_eq!(m["key"], json!("k1"));
    }

    // Everything is now leased to the first caller, so a concurrent receive gets nothing.
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-run",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 10,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r2["msgs"].assert_array().len(), 0);

    Ok(())
}

#[tokio::test]
async fn fifo_locks_keys_independently() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-indep" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-indep",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k2" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let req = json!({
        "namespace": "ns-fifo-indep",
        "topic": "t1",
        "consumer_group": "test-cg",
        "batch_size": 1,
    });

    // First receive leases k1's head; k1 is now locked.
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let m1 = r1["msgs"].assert_array();
    assert_eq!(m1.len(), 1);
    assert_eq!(m1[0]["key"], json!("k1"));

    // k2 is a different key and still free, so the next receive returns it (not empty).
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let m2 = r2["msgs"].assert_array();
    assert_eq!(m2.len(), 1, "an unrelated key is unaffected by k1's lock");
    assert_eq!(m2[0]["key"], json!("k2"));

    // Both keys are now locked, so the third receive is empty.
    let r3 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r3["msgs"].assert_array().len(), 0);

    Ok(())
}

#[tokio::test]
async fn fifo_nack_keeps_key_locked_until_retry_elapses() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-nack" }))
        .await?
        .expect(StatusCode::OK);

    // Retry the first failure after 1s (no DLQ topic).
    client
        .post("v1.msgs.fifo.configure")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [1000],
            "dlq_topic": null,
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Lease the head with a long lease, then nack it.
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
            "lease_duration_ms": 60000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    let head_id = r1["msgs"].assert_array()[0]["msg_id"].assert_str().to_owned();

    client
        .post("v1.msgs.fifo.nack")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [head_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // While the head is retry-scheduled, the key stays locked: "b" is not delivered.
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "a retry-scheduled head keeps its key locked"
    );

    // After the retry delay, the head becomes available again and is re-delivered (still ahead of b).
    time.fast_forward(Duration::from_millis(1500));
    let r3 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-nack",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    let m3 = r3["msgs"].assert_array();
    assert_eq!(m3.len(), 1);
    assert_eq!(m3[0]["value"], json!("a".as_bytes()), "the head retries before its successor");

    Ok(())
}

#[tokio::test]
async fn fifo_dlq_unblocks_key_and_redrive_redelivers() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-dlq" }))
        .await?
        .expect(StatusCode::OK);

    // Empty retry schedule + no dlq_topic → a nack immediately marks the message DLQ in place.
    client
        .post("v1.msgs.fifo.configure")
        .json(json!({
            "namespace": "ns-fifo-dlq",
            "topic": "t1",
            "consumer_group": "test-cg",
            "retry_schedule": [],
            "dlq_topic": null,
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-dlq",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let req = json!({
        "namespace": "ns-fifo-dlq",
        "topic": "t1",
        "consumer_group": "test-cg",
        "batch_size": 1,
        "lease_duration_ms": 60000,
    });

    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let head_id = r1["msgs"].assert_array()[0]["msg_id"].assert_str().to_owned();
    
    // nacking sends it straight to the DLQ.
    client
        .post("v1.msgs.fifo.nack")
        .json(json!({
            "namespace": "ns-fifo-dlq",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [head_id],
        }))
        .await?
        .expect(StatusCode::OK);

    // The DLQ'd head no longer locks the key, so its successor "b" is delivered.
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let m2 = r2["msgs"].assert_array();
    assert_eq!(m2.len(), 1, "a DLQ'd head unblocks the key");
    assert_eq!(m2[0]["value"], json!("b".as_bytes()));

    // Redriving the DLQ makes the original head available again and re-delivers it.
    client
        .post("v1.msgs.fifo.redrive-dlq")
        .json(json!({
            "namespace": "ns-fifo-dlq",
            "topic": "t1",
            "consumer_group": "test-cg",
        }))
        .await?
        .expect(StatusCode::OK);

    let r3 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let m3 = r3["msgs"].assert_array();
    assert_eq!(m3.len(), 1, "the redriven message is delivered again");
    assert_eq!(m3[0]["value"], json!("a".as_bytes()));

    Ok(())
}

#[tokio::test]
async fn fifo_keyless_messages_never_block_each_other() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-keyless" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-keyless",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes() },
                { "value": "b".as_bytes() },
                { "value": "c".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let req = json!({
        "namespace": "ns-fifo-keyless",
        "topic": "t1",
        "consumer_group": "test-cg",
        "batch_size": 1,
        "lease_duration_ms": 60000,
    });

    let mut values = Vec::new();
    for _ in 0..3 {
        let r = client
            .post("v1.msgs.fifo.receive")
            .json(req.clone())
            .await?
            .expect(StatusCode::OK)
            .json();
        let m = r["msgs"].assert_array();
        assert_eq!(m.len(), 1, "each keyless message is deliverable despite others in-flight");
        assert!(m[0]["key"].is_null(), "keyless messages carry no key");
        values.push(m[0]["value"].clone());
    }

    assert_eq!(
        values,
        vec![json!("a".as_bytes()), json!("b".as_bytes()), json!("c".as_bytes())],
        "keyless messages are delivered in offset order"
    );

    Ok(())
}

#[tokio::test]
async fn fifo_expired_lease_redelivers_the_head() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-expiry" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-expiry",
            "topic": "t1",
            "msgs": [
                { "value": "a".as_bytes(), "key": "k1" },
                { "value": "b".as_bytes(), "key": "k1" },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    let req = json!({
        "namespace": "ns-fifo-expiry",
        "topic": "t1",
        "consumer_group": "test-cg",
        "batch_size": 1,
        "lease_duration_ms": 1000,
    });

    // Lease the head, let the lease expire without acking.
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(r1["msgs"].assert_array()[0]["value"], json!("a".as_bytes()));

    time.fast_forward(Duration::from_millis(1500));

    // The head is re-delivered — not its successor.
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(req.clone())
        .await?
        .expect(StatusCode::OK)
        .json();
    let m2 = r2["msgs"].assert_array();
    assert_eq!(m2.len(), 1);
    assert_eq!(
        m2[0]["value"],
        json!("a".as_bytes()),
        "the expired head is re-delivered ahead of its successor"
    );

    Ok(())
}

#[tokio::test]
async fn fifo_extend_lease_prevents_redelivery() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        time,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns-fifo-extend" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns-fifo-extend",
            "topic": "t1",
            "msgs": [ { "value": "a".as_bytes(), "key": "k1" } ],
        }))
        .await?
        .expect(StatusCode::OK);

    // Lease with a short duration, then extend it well past the original expiry.
    let r1 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-extend",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
            "lease_duration_ms": 1000,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    let msg_id = r1["msgs"].assert_array()[0]["msg_id"].assert_str().to_owned();

    client
        .post("v1.msgs.fifo.extend-lease")
        .json(json!({
            "namespace": "ns-fifo-extend",
            "topic": "t1",
            "consumer_group": "test-cg",
            "msg_ids": [msg_id],
            "lease_duration_ms": 60000,
        }))
        .await?
        .expect(StatusCode::OK);

    // Past the original 1s lease, but the extension still holds it.
    time.fast_forward(Duration::from_millis(1500));
    let r2 = client
        .post("v1.msgs.fifo.receive")
        .json(json!({
            "namespace": "ns-fifo-extend",
            "topic": "t1",
            "consumer_group": "test-cg",
            "batch_size": 1,
        }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        r2["msgs"].assert_array().len(),
        0,
        "an extended lease is not re-delivered at the original expiry"
    );

    Ok(())
}
