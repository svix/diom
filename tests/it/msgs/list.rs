use std::collections::BTreeSet;

use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn test_list_topics() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns1" }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("v1.msgs.namespace.configure")
        .json(json!({ "name": "ns2" }))
        .await?
        .expect(StatusCode::OK);

    // post some messages to auto-create a topic
    client
        .post("v1.msgs.publish")
        .json(json!({
            "namespace": "ns1",
            "topic": "t1",
            "msgs": [
                { "value": "hello".as_bytes() },
                { "value": "world".as_bytes() },
            ],
        }))
        .await?
        .expect(StatusCode::OK);

    // explicitly create a few more topics
    for i in 2..=5 {
        client
            .post("v1.msgs.topic.configure")
            .json(json!({
                "namespace": "ns1",
                "topic": format!("t{i}"),
                "partitions": i,
            }))
            .await?
            .expect(StatusCode::OK);
    }

    // and create one in another namespace to make sure we don't see it
    client
        .post("v1.msgs.topic.configure")
        .json(json!({
            "namespace": "ns2",
            "topic": "t6",
            "partitions": 1,
        }))
        .await?
        .expect(StatusCode::OK);

    // now actually list them
    let response = client
        .post("v1.msgs.topic.list")
        .json(json!({"namespace": "ns1", "limit": 3}))
        .await?
        .expect(StatusCode::OK)
        .json();

    let keys = response
        .as_object()
        .expect("response should be an object")
        .keys()
        .map(|s| s.as_str())
        .collect::<BTreeSet<&str>>();
    assert_eq!(
        keys,
        maplit::btreeset! { "data", "iterator", "prev_iterator", "done" }
    );
    assert!(!response["done"].assert_bool());
    let data = response["data"]
        .as_array()
        .expect("data should be an array");
    assert_eq!(
        data.iter()
            .map(|r| r["name"].assert_str())
            .collect::<BTreeSet<&str>>(),
        maplit::btreeset! { "t1", "t2", "t3" }
    );

    let response = client
        .post("v1.msgs.topic.list")
        .json(json!({"namespace": "ns1", "limit": 3, "iterator": response["iterator"] }))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert!(response["done"].assert_bool());
    let data = response["data"]
        .as_array()
        .expect("data should be an array");
    assert_eq!(
        data.iter()
            .map(|r| r["name"].assert_str())
            .collect::<BTreeSet<&str>>(),
        maplit::btreeset! { "t4", "t5" }
    );

    Ok(())
}
