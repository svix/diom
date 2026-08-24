use anyhow::Context;
use serde_json::json;
use std::io::Write;
use tap::Pipe;
use test_utils::{
    JsonFastAndLoose, StatusCode, TestClient, TestResponse, TestResult,
    retry::run_with_retries,
    server::{TestContext, start_cluster, start_server},
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

async fn kv_set(
    client: &TestClient,
    key: &str,
    value: &str,
    behavior: &str,
) -> anyhow::Result<TestResponse> {
    client
        .post("v1.kv.set")
        .msgpack(json!({
            "key": key,
            "value": value.as_bytes(),
            "behavior": behavior
        }))
        .await
        .map_err(Into::into)
}

#[allow(clippy::disallowed_types)] // serde_json::Value okay for tests
async fn kv_get(client: &TestClient, key: &str) -> anyhow::Result<TestResponse> {
    client
        .post("v1.kv.get")
        .json(json!({
            "key": key
        }))
        .await?
        .ensure(StatusCode::OK)
}

async fn test_mutation_header(client: &TestClient) -> TestResult {
    let response = kv_set(client, "foo", "var", "upsert").await?;
    let value = response
        .headers()
        .get("Diom-Mutation-Version")
        .expect("header should be set");
    let initial_index: u64 = value
        .to_str()
        .expect("value should start out a a str")
        .parse()
        .expect("value should be a u64");
    tracing::debug!("initial index is {initial_index}");

    let response = kv_set(client, "foo", "bar", "upsert").await?;
    let new_index: u64 = response
        .headers()
        .get("Diom-Mutation-Version")
        .expect("header should still be set on second request")
        .to_str()
        .expect("value should start out a str")
        .parse()
        .expect("value should be a u64");
    tracing::debug!("second request index is {new_index}");
    assert!(new_index > initial_index);

    let response = kv_get(client, "foo").await?;
    assert!(
        response.headers().get("Diom-Mutation-Version").is_none(),
        "Diom-Mutation-Version should not be set on get requests"
    );

    Ok(())
}

#[tokio::test]
async fn test_diom_mutation_header_single_node() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = start_server().await;
    test_mutation_header(&client).await
}

#[tokio::test]
async fn test_diom_mutation_header_cluster() -> TestResult {
    let context = start_cluster(2).await;

    test_mutation_header(context.leader_client().await).await?;
    test_mutation_header(context.follower_client().await).await?;

    Ok(())
}

#[tokio::test]
async fn test_replication_end_to_end() -> TestResult {
    let context = start_cluster(3).await;

    let key = format!("key{}", uuid::Uuid::new_v4().simple());
    let value = "leader-value";
    let leader_client = context.leader_client().await;
    let follower_client = context.follower_client().await;

    async fn ensure_visible(
        client: &TestClient,
        key: &str,
        expected_value: &str,
    ) -> anyhow::Result<()> {
        let found_value = kv_get(client, key)
            .await?
            .json()
            .get("value")
            .context("no val in response")?
            .assert_bytes()
            .pipe(String::from_utf8)
            .unwrap();
        anyhow::ensure!(found_value == expected_value);
        Ok(())
    }

    // insert a key on the leader
    kv_set(leader_client, &key, value, "upsert").await?;
    // it should immediately be visible on the leader
    ensure_visible(leader_client, &key, value).await?;
    // it should eventually be visible on a follower
    run_with_retries(async || ensure_visible(follower_client, &key, value).await).await?;

    // insert it from the follower (which forwards it)
    let value = "follower-value";
    kv_set(follower_client, &key, value, "upsert").await?;
    // it should immediately be visible on the leader
    ensure_visible(leader_client, &key, value).await?;
    // it should eventually be visible on a follower
    run_with_retries(async || ensure_visible(follower_client, &key, value).await).await?;

    Ok(())
}

async fn make_http11_kv_set(ctx: &TestContext, value: &str) -> TestResult<String> {
    let url = url::Url::parse(&ctx.client.base_uri).unwrap();
    let host = url.host_str().unwrap();
    let port = url.port().unwrap_or(ctx.cfg.listen_address.port());
    let mut conn = tokio::net::TcpStream::connect((host, port)).await?;
    let (read, mut write) = conn.split();
    let body = serde_json::json!({
        "key": "key",
        "value": value,
        "behavior": "upsert",
    });
    let header_lines: &[&[u8]] = &[
        b"POST /api/v1.kv.set HTTP/1.1",
        b"Connection: keep-alive",
        b"Transfer-Encoding: Chunked",
        b"Content-Type: application/msgpack",
        b"Accept: application/json",
    ];
    let mut http_request: Vec<u8> = vec![];
    for line in header_lines {
        http_request.extend(*line);
        http_request.extend(b"\r\n");
    }
    if let Some(auth) = &ctx.client.auth_header {
        write!(&mut http_request, "Authorization: {auth}\r\n")?;
    };
    http_request.extend(b"\r\n");
    // send off the headers and wait for them to be flushed
    write.write_all(&http_request).await?;
    write.flush().await?;
    let mut msgpack_body = rmp_serde::to_vec_named(&body)?;
    let mut http_request_body = vec![];
    // the chunk length
    write!(&mut http_request_body, "{:x}\r\n", msgpack_body.len())?;
    // the body
    http_request_body.append(&mut msgpack_body);
    // an empty chunk for the end of the stream
    write!(&mut http_request_body, "\r\n0\r\n\r\n")?;
    // now send the body (but ignore if we get an EPIPE)
    if let Err(err) = write.write_all(&http_request_body).await {
        tracing::debug!(?err, "got an error writing body; ignoring");
    } else {
        let _ = write.flush().await;
    }
    // read the response line
    let mut buf = String::new();
    let mut read = tokio::io::BufReader::new(read);
    read.read_line(&mut buf).await?;
    Ok(buf)
}

#[tokio::test]
async fn test_body_limit() -> TestResult {
    let ctx = TestContext::builder()
        .tap_cfg(|cfg| {
            cfg.max_body_size = 10000;
        })
        .build()
        .await;
    // a small request should succeed
    kv_set(&ctx.client, "key", "a", "upsert")
        .await?
        .expect(StatusCode::OK);
    // a large (normal) request should 413
    let large_value = std::iter::repeat_n('n', 10001).collect::<String>();
    kv_set(&ctx.client, "key", &large_value, "upsert")
        .await?
        .expect(StatusCode::PAYLOAD_TOO_LARGE);
    // now make a chunked http1.1 request (no content-length)
    let response_line = make_http11_kv_set(&ctx, &large_value).await?;
    assert_eq!(response_line, "HTTP/1.1 200 OK\r\n");
    // a very very large http/1.1 request should still fail
    let huge_value = std::iter::repeat_n('n', 50001).collect::<String>();
    let response_line = make_http11_kv_set(&ctx, &huge_value).await?;
    assert_eq!(response_line, "HTTP/1.1 413 Payload Too Large\r\n");

    Ok(())
}

#[tokio::test]
async fn test_body_limit_allows_bigger_than_the_default() -> TestResult {
    let ctx = TestContext::builder()
        .tap_cfg(|cfg| {
            cfg.max_body_size = 3_000_000;
        })
        .build()
        .await;
    // a larger than 2MB but smaller than max_body_size request should succeed
    let huge_value = std::iter::repeat_n('n', 2_097_153).collect::<String>();
    kv_set(&ctx.client, "key", &huge_value, "upsert")
        .await?
        .expect(StatusCode::OK);
    // a larger than 3MB request should fail
    let huger_value = std::iter::repeat_n('n', 3_000_001).collect::<String>();
    kv_set(&ctx.client, "key", &huger_value, "upsert")
        .await?
        .expect(StatusCode::PAYLOAD_TOO_LARGE);
    // as an HTTP/1.1 request, it should succeed
    let response_line = make_http11_kv_set(&ctx, &huger_value).await?;
    assert_eq!(response_line, "HTTP/1.1 200 OK\r\n");
    let hugest_value = std::iter::repeat_n('n', 12_000_012).collect::<String>();
    let response_line = make_http11_kv_set(&ctx, &hugest_value).await?;
    assert_eq!(response_line, "HTTP/1.1 413 Payload Too Large\r\n");

    // It would be good to test here a request *much* larger that hits that 4x
    // body-reading limit and gets an EPIPE; however, that's totally dependent on
    // OS scheduling (basically, whether we can write data faster than the server
    // can do the SHUT_RD), so there's not a good way to test it. Maybe some kind
    // of custom IO stream implementation?
    Ok(())
}
