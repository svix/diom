use serde_json::json;
use test_utils::{
    StatusCode, TestResult,
    server::{TestContext, start_server},
};

#[tokio::test]
async fn test_execute_identity() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = client
        .post("v1.transformations.execute")
        .json(json!({
            "input": r#"{"x": 42}"#,
            "script": "function handler(input) { return input; }"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["output"], json!(r#"{"x":42}"#));

    Ok(())
}

#[tokio::test]
async fn test_execute_transforms_input() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    let response = client
        .post("v1.transformations.execute")
        .json(json!({
            "input": r#"{"x": 1}"#,
            "script": "function handler(input) { return { result: input.x + 41 }; }"
        }))
        .await?
        .expect(StatusCode::OK)
        .json();

    assert_eq!(response["output"], json!(r#"{"result":42}"#));

    Ok(())
}

#[tokio::test]
async fn test_execute_script_exception_is_bad_request() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.transformations.execute")
        .json(json!({
            "input": "null",
            "script": r#"function handler(input) { throw new Error("intentional failure"); }"#
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn test_execute_timeout_is_bad_request() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.transformations.execute")
        .json(json!({
            "input": "null",
            "script": r#"function handler(input) { for (var i = 0; i < 10000000000; ++i) {} }"#
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}

#[tokio::test]
async fn test_execute_missing_handler_is_bad_request() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;

    client
        .post("v1.transformations.execute")
        .json(json!({
            "input": "null",
            "script": "var x = 1;"
        }))
        .await?
        .expect(StatusCode::BAD_REQUEST);

    Ok(())
}
