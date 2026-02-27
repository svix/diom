// use std::time::Duration;

// use serde_json::json;

// use test_utils::{
//     StatusCode, TestResult,
//     server::{TestContext, start_server},
// };

// #[tokio::test]
// async fn create_stream_upserts() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let response = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 },
//             "storage_type": "Ephemeral"
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let ts = &response["created"];

//     assert_eq!(
//         response,
//         json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 },
//             "storage_type": "Ephemeral",
//             "created": ts,
//             "updated": ts,
//         })
//     );

//     let update = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream"
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     assert_eq!(
//         update,
//         json!({
//             "name": "test-stream",
//             "retention": &update["retention"],
//             "storage_type": "Persistent",
//             "created": ts,
//             "updated": &update["updated"],
//         })
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn stream_append_and_locking_consumption() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();
//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [1, 2], "headers": {"msg": "1"}},
//                 {"payload": [3, 4], "headers": {"msg": "2"}},
//                 {"payload": [5, 6], "headers": {"msg": "3"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [7, 8], "headers": {"msg": "4"}},
//                 {"payload": [9, 10], "headers": {"msg": "5"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch first batch of 3 messages
//     let fetch1 = client
//         .post("stream/fetch-locking")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 3);
//     assert_eq!(
//         msgs1[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs1[0]["timestamp"]})
//     );
//     assert_eq!(
//         msgs1[1],
//         json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs1[1]["timestamp"]})
//     );
//     assert_eq!(
//         msgs1[2],
//         json!({"id": 2, "payload": [5, 6], "headers": {"msg": "3"}, "timestamp": msgs1[2]["timestamp"]})
//     );

//     // Ack the first batch
//     let max_msg_id = &msgs1[2]["id"];
//     client
//         .post("stream/ack-range")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "max_msg_id": max_msg_id
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch second batch - should get remaining 2 messages
//     let fetch2 = client
//         .post("stream/fetch-locking")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 2);
//     assert_eq!(
//         msgs2[0],
//         json!({"id": 3, "payload": [7, 8], "headers": {"msg": "4"}, "timestamp": msgs2[0]["timestamp"]})
//     );
//     assert_eq!(
//         msgs2[1],
//         json!({"id": 4, "payload": [9, 10], "headers": {"msg": "5"}, "timestamp": msgs2[1]["timestamp"]})
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn stream_visibility_timeout() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [1, 2], "headers": {"msg": "1"}},
//                 {"payload": [3, 4], "headers": {"msg": "2"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch messages with a short visibility timeout (1 second)
//     let fetch1 = client
//         .post("stream/fetch-locking")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 1
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 2);
//     assert_eq!(
//         msgs1[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs1[0]["timestamp"]})
//     );
//     assert_eq!(
//         msgs1[1],
//         json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs1[1]["timestamp"]})
//     );

//     // Wait for the visibility timeout to expire
//     tokio::time::sleep(Duration::from_secs(2)).await;

//     // Fetch again - should get the same messages since they weren't acked
//     let fetch2 = client
//         .post("stream/fetch-locking")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 1
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 2);
//     assert_eq!(
//         msgs2[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs2[0]["timestamp"]})
//     );
//     assert_eq!(
//         msgs2[1],
//         json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs2[1]["timestamp"]})
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn queue_fetch_with_queue_semantics() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [1, 2], "headers": {"msg": "1"}},
//                 {"payload": [3, 4], "headers": {"msg": "2"}},
//                 {"payload": [5, 6], "headers": {"msg": "3"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch first batch of 2 messages
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 2,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 2);
//     assert_eq!(
//         msgs1[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "1"}, "timestamp": msgs1[0]["timestamp"]})
//     );
//     assert_eq!(
//         msgs1[1],
//         json!({"id": 1, "payload": [3, 4], "headers": {"msg": "2"}, "timestamp": msgs1[1]["timestamp"]})
//     );

//     // Ack the first batch
//     let max_msg_id = &msgs1[1]["id"];
//     client
//         .post("stream/ack-range")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "max_msg_id": max_msg_id
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch second batch - should get remaining message
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 2,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 1);
//     assert_eq!(
//         msgs2[0],
//         json!({"id": 2, "payload": [5, 6], "headers": {"msg": "3"}, "timestamp": msgs2[0]["timestamp"]})
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn queue_fetch_concurrent_consumers() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [1, 2], "headers": {"msg": "A"}},
//                 {"payload": [3, 4], "headers": {"msg": "B"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch message A with a short visibility timeout
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 1,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 1);
//     assert_eq!(
//         msgs1[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "A"}, "timestamp": msgs1[0]["timestamp"]})
//     );

//     // Fetch again - should get message B (not blocked by A's lock)
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 1,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 1);
//     assert_eq!(
//         msgs2[0],
//         json!({"id": 1, "payload": [3, 4], "headers": {"msg": "B"}, "timestamp": msgs2[0]["timestamp"]})
//     );

//     // Wait for A's visibility timeout to expire
//     tokio::time::sleep(Duration::from_secs(2)).await;

//     // Fetch again - should get message A back (its lock expired)
//     let fetch3 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 1,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs3 = fetch3["msgs"].as_array().unwrap();
//     assert_eq!(msgs3.len(), 1);
//     assert_eq!(
//         msgs3[0],
//         json!({"id": 0, "payload": [1, 2], "headers": {"msg": "A"}, "timestamp": msgs3[0]["timestamp"]})
//     );

//     Ok(())
// }

// #[tokio::test]
// async fn queue_fetch_mixed_visibility_timeouts() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [0], "headers": {"msg": "A"}},
//                 {"payload": [1], "headers": {"msg": "B"}},
//                 {"payload": [2], "headers": {"msg": "C"}},
//                 {"payload": [3], "headers": {"msg": "D"}},
//                 {"payload": [4], "headers": {"msg": "E"}},
//                 {"payload": [5], "headers": {"msg": "F"}},
//                 {"payload": [6], "headers": {"msg": "G"}},
//                 {"payload": [7], "headers": {"msg": "H"}},
//                 {"payload": [8], "headers": {"msg": "I"}},
//                 {"payload": [9], "headers": {"msg": "J"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch A + B with short visibility timeout
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 2,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 2);
//     assert_eq!(msgs1[0]["headers"]["msg"], "A");
//     assert_eq!(msgs1[1]["headers"]["msg"], "B");

//     // Fetch C + D with short visibility timeout
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 2,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 2);
//     assert_eq!(msgs2[0]["headers"]["msg"], "C");
//     assert_eq!(msgs2[1]["headers"]["msg"], "D");

//     // Fetch E + F with LONG visibility timeout
//     let fetch3 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 2,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs3 = fetch3["msgs"].as_array().unwrap();
//     assert_eq!(msgs3.len(), 2);
//     assert_eq!(msgs3[0]["headers"]["msg"], "E");
//     assert_eq!(msgs3[1]["headers"]["msg"], "F");

//     // Ack C + D
//     client
//         .post("stream/ack-range")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "min_msg_id": msgs2[0]["id"],
//             "max_msg_id": msgs2[1]["id"]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Wait for A+B visibility timeout to expire (but not E+F)
//     tokio::time::sleep(Duration::from_secs(3)).await;

//     // Fetch remaining - should get A, B (expired), G, H, I, J (never fetched)
//     // Should NOT get C, D (acked) or E, F (still locked)
//     let fetch4 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs4 = fetch4["msgs"].as_array().unwrap();
//     assert_eq!(msgs4.len(), 6);
//     assert_eq!(msgs4[0]["headers"]["msg"], "A");
//     assert_eq!(msgs4[1]["headers"]["msg"], "B");
//     assert_eq!(msgs4[2]["headers"]["msg"], "G");
//     assert_eq!(msgs4[3]["headers"]["msg"], "H");
//     assert_eq!(msgs4[4]["headers"]["msg"], "I");
//     assert_eq!(msgs4[5]["headers"]["msg"], "J");

//     Ok(())
// }

// #[tokio::test]
// async fn queue_fetch_partial_ack_across_blocks() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [0], "headers": {"msg": "A"}},
//                 {"payload": [1], "headers": {"msg": "B"}},
//                 {"payload": [2], "headers": {"msg": "C"}},
//                 {"payload": [3], "headers": {"msg": "D"}},
//                 {"payload": [4], "headers": {"msg": "E"}},
//                 {"payload": [5], "headers": {"msg": "F"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch first block: A, B, C (ids 0, 1, 2)
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 3);
//     assert_eq!(msgs1[0]["headers"]["msg"], "A");
//     assert_eq!(msgs1[1]["headers"]["msg"], "B");
//     assert_eq!(msgs1[2]["headers"]["msg"], "C");

//     // Fetch second block: D, E, F (ids 3, 4, 5)
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 3);
//     assert_eq!(msgs2[0]["headers"]["msg"], "D");
//     assert_eq!(msgs2[1]["headers"]["msg"], "E");
//     assert_eq!(msgs2[2]["headers"]["msg"], "F");

//     // Ack a range that overlaps both blocks but doesn't fully cover either:
//     // minMsgId=1, maxMsgId=4 acks B, C, D, E (but NOT A or F)
//     client
//         .post("stream/ack-range")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "min_msg_id": 1,
//             "max_msg_id": 4
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Wait for visibility timeout to expire
//     tokio::time::sleep(Duration::from_secs(3)).await;

//     // Fetch again - should get A and F back (the unacked messages)
//     let fetch3 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs3 = fetch3["msgs"].as_array().unwrap();
//     assert_eq!(msgs3.len(), 2);
//     assert_eq!(msgs3[0]["headers"]["msg"], "A");
//     assert_eq!(msgs3[1]["headers"]["msg"], "F");

//     Ok(())
// }

// #[tokio::test]
// async fn queue_fetch_single_ack() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [0], "headers": {"msg": "A"}},
//                 {"payload": [1], "headers": {"msg": "B"}},
//                 {"payload": [2], "headers": {"msg": "C"}},
//                 {"payload": [3], "headers": {"msg": "D"}},
//                 {"payload": [4], "headers": {"msg": "E"}},
//                 {"payload": [5], "headers": {"msg": "F"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 6,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 6);
//     assert_eq!(msgs1[0]["headers"]["msg"], "A");
//     assert_eq!(msgs1[1]["headers"]["msg"], "B");
//     assert_eq!(msgs1[2]["headers"]["msg"], "C");
//     assert_eq!(msgs1[3]["headers"]["msg"], "D");
//     assert_eq!(msgs1[4]["headers"]["msg"], "E");
//     assert_eq!(msgs1[5]["headers"]["msg"], "F");

//     // Ack single messages, rather than an entire block
//     for msg_id in [2, 4] {
//         client
//             .post("stream/ack")
//             .json(json!({
//                 "name": "test-stream",
//                 "consumer_group": "test-group",
//                 "msg_id": msg_id
//             }))
//             .await?
//             .expect(StatusCode::OK);
//     }

//     // Wait for visibility timeout to expire
//     tokio::time::sleep(Duration::from_secs(2)).await;

//     // Fetch again - should get A, B, D, F back (the unacked messages)
//     let fetch3 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs3 = fetch3["msgs"].as_array().unwrap();
//     assert_eq!(msgs3.len(), 4);
//     assert_eq!(msgs3[0]["headers"]["msg"], "A");
//     assert_eq!(msgs3[1]["headers"]["msg"], "B");
//     assert_eq!(msgs3[2]["headers"]["msg"], "D");
//     assert_eq!(msgs3[3]["headers"]["msg"], "F");

//     Ok(())
// }

// #[tokio::test]
// async fn queue_dlq_and_redrive() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [0], "headers": {"msg": "A"}},
//                 {"payload": [1], "headers": {"msg": "B"}},
//                 {"payload": [2], "headers": {"msg": "C"}},
//                 {"payload": [3], "headers": {"msg": "D"}},
//                 {"payload": [4], "headers": {"msg": "E"}},
//                 {"payload": [5], "headers": {"msg": "F"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch all messages
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 6,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 6);

//     // DLQ messages B and D (ids 1 and 3)
//     for msg_id in [1, 3] {
//         client
//             .post("stream/dlq")
//             .json(json!({
//                 "name": "test-stream",
//                 "consumer_group": "test-group",
//                 "msg_id": msg_id
//             }))
//             .await?
//             .expect(StatusCode::OK);
//     }

//     // Wait for visibility timeout to expire (from fetch1's lease covering 0-5)
//     tokio::time::sleep(Duration::from_secs(3)).await;

//     // Fetch - should get A, C, E, F (B and D are in DLQ)
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 2
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 4);
//     assert_eq!(msgs2[0]["headers"]["msg"], "A");
//     assert_eq!(msgs2[1]["headers"]["msg"], "C");
//     assert_eq!(msgs2[2]["headers"]["msg"], "E");
//     assert_eq!(msgs2[3]["headers"]["msg"], "F");

//     // Ack the fetched messages (A, C, E, F) individually so B and D aren't acked
//     for msg_id in [0, 2, 4, 5] {
//         client
//             .post("stream/ack")
//             .json(json!({
//                 "name": "test-stream",
//                 "consumer_group": "test-group",
//                 "msg_id": msg_id
//             }))
//             .await?
//             .expect(StatusCode::OK);
//     }

//     // Redrive the DLQ
//     client
//         .post("stream/redrive-dlq")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group"
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Wait for visibility timeout to expire (from fetch2's lease)
//     tokio::time::sleep(Duration::from_secs(3)).await;

//     // Fetch again - should get B and D (redriven from DLQ, others acked)
//     let fetch3 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs3 = fetch3["msgs"].as_array().unwrap();
//     assert_eq!(msgs3.len(), 2);
//     assert_eq!(msgs3[0]["headers"]["msg"], "B");
//     assert_eq!(msgs3[1]["headers"]["msg"], "D");

//     Ok(())
// }

// #[tokio::test]
// async fn queue_dlq_with_partial_ack_and_redrive() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [0], "headers": {"msg": "A"}},
//                 {"payload": [1], "headers": {"msg": "B"}},
//                 {"payload": [2], "headers": {"msg": "C"}},
//                 {"payload": [3], "headers": {"msg": "D"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch first 3 messages
//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 3);
//     assert_eq!(msgs1[0]["headers"]["msg"], "A");
//     assert_eq!(msgs1[1]["headers"]["msg"], "B");
//     assert_eq!(msgs1[2]["headers"]["msg"], "C");

//     // DLQ message B (id 1)
//     client
//         .post("stream/dlq")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "msg_id": 1
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Ack messages A and C (ids 0 and 2)
//     for msg_id in [0, 2] {
//         client
//             .post("stream/ack")
//             .json(json!({
//                 "name": "test-stream",
//                 "consumer_group": "test-group",
//                 "msg_id": msg_id
//             }))
//             .await?
//             .expect(StatusCode::OK);
//     }

//     // Redrive the DLQ
//     client
//         .post("stream/redrive-dlq")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group"
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch again - should get B (redriven from DLQ) and D (never fetched)
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 3,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 2);
//     assert_eq!(msgs2[0]["headers"]["msg"], "B");
//     assert_eq!(msgs2[1]["headers"]["msg"], "D");

//     Ok(())
// }

// #[tokio::test]
// async fn queue_ack_dlqd_message_prevents_redrive() -> TestResult {
//     let TestContext {
//         client,
//         handle: _handle,
//         ..
//     } = start_server().await;

//     let _stream = client
//         .post("msgs/namespace/create")
//         .json(json!({
//             "name": "test-stream",
//             "retention": { "bytes": 1024, "millis": 9999 }
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     client
//         .post("stream/append")
//         .json(json!({
//             "name": "test-stream",
//             "msgs": [
//                 {"payload": [1, 2], "headers": {"msg": "A"}},
//             ]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     let fetch1 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 1,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs1 = fetch1["msgs"].as_array().unwrap();
//     assert_eq!(msgs1.len(), 1);
//     assert_eq!(msgs1[0]["headers"]["msg"], "A");

//     client
//         .post("stream/dlq")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "msg_id": msgs1[0]["id"]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     client
//         .post("stream/ack")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "msg_id": msgs1[0]["id"]
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     client
//         .post("stream/redrive-dlq")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group"
//         }))
//         .await?
//         .expect(StatusCode::OK);

//     // Fetch again, even though the message was DLQd, since the message
//     // was also ACK'd, it shouldn't be redriven.
//     // This is to try to approximate exactly-once semantics as much as is
//     // possible.
//     let fetch2 = client
//         .post("stream/fetch")
//         .json(json!({
//             "name": "test-stream",
//             "consumer_group": "test-group",
//             "batch_size": 10,
//             "visibility_timeout_seconds": 3600
//         }))
//         .await?
//         .expect(StatusCode::OK)
//         .json();

//     let msgs2 = fetch2["msgs"].as_array().unwrap();
//     assert_eq!(msgs2.len(), 0);

//     Ok(())
// }
