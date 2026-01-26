use serde_json::json;
use test_utils::{StatusCode, TestResult};

#[tokio::test]
async fn test_create_stream() -> TestResult {
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
    let id = &response["id"];

    assert_eq!(
        response,
        json!({
            "id": id,
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
            "id": id,
            "name": "test-stream",
            "maxByteSize": null,
            "retentionPeriodSeconds": null,
            "createdAt": ts,
            "updatedAt": &update["updatedAt"],
        })
    );

    client
        .post("stream/append")
        .json(json!({
            "streamId": id,
            "msgs": [
                {"payload": [1, 2, 3], "headers": {}},
                {"payload": [4, 5, 6], "headers": {"key": "value"}}
            ]
        }))
        .await?
        .expect(StatusCode::OK);

    client
        .post("stream/append")
        .json(json!({
            "streamId": id,
            "msgs": [
                {"payload": [7, 8, 9], "headers": {}}
            ]
        }))
        .await?
        .expect(StatusCode::OK);

    Ok(())
}
