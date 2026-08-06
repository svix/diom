use diom_core::types::NonZeroDurationMs;
use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    retry::run_with_retries,
    server::{TEST_ADMIN_TOKEN, TestServerBuilder},
};

#[tokio::test]
async fn test_cluster_id_roundtrips_log_purge() -> TestResult {
    // start up one server
    let s1_context = TestServerBuilder::with_default_config()
        .token(TEST_ADMIN_TOKEN.to_owned())
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
            cfg.cluster.minimum_snapshot_interval = NonZeroDurationMs::from_secs(3).unwrap();
            cfg.cluster.snapshot_after_writes = 10.into();
        })
        .build()
        .await;

    let client = &s1_context.client;

    // do a bunch of writes to it to force a snapshot
    for i in 0..11u32 {
        client
            .post("v1.kv.set")
            .json(json!({
                "key": format!("key{i}"),
                "value": i.to_be_bytes(),
                "behavior": "upsert"
            }))
            .await?
            .ensure(StatusCode::OK)?;
    }

    // now wait for it to snapshot (and purge)
    run_with_retries(async || {
        let response = client
            .get("v1.cluster-admin.status")
            .await?
            .ensure(StatusCode::OK)?;
        let body = response.json();
        assert_eq!(
            body["cluster_id"].assert_str(),
            s1_context.cluster_id.to_string()
        );
        more_asserts::assert_ge!(
            body["nodes"].assert_array()[0]["last_committed_log_index"].assert_u64(),
            10
        );
        anyhow::ensure!(body["this_node_last_purged_log_index"].is_u64());
        Ok(())
    })
    .await?;

    // now start up another node; it should have to snapshot
    let s2_context = TestServerBuilder::with_default_config()
        .token(TEST_ADMIN_TOKEN.to_owned())
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
            cfg.cluster.minimum_snapshot_interval = NonZeroDurationMs::from_secs(3).unwrap();
            cfg.cluster.snapshot_after_writes = 10.into();
            cfg.cluster.seed_nodes = vec![s1_context.repl_addr.into()]
        })
        .build()
        .await;

    let response = s2_context
        .client
        .get("v1.cluster-admin.status")
        .await?
        .ensure(StatusCode::OK)?;
    let body = response.json();
    assert_eq!(
        body["cluster_id"].assert_str(),
        s1_context.cluster_id.to_string()
    );

    Ok(())
}
