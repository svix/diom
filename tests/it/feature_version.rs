use std::{net::SocketAddr, time::Duration};

use diom_backend::cfg::VersionRange;
use diom_core::types::NonZeroDurationMs;
use diom_operations::FeatureVersion;
use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestClient, TestResult,
    retry::run_with_retries,
    server::{TEST_ADMIN_TOKEN, TestServerBuilder, start_cluster_with},
};

/// A short advance interval so the leader's advance worker fires promptly under test.
fn fast_advance_interval() -> NonZeroDurationMs {
    NonZeroDurationMs::from_millis(100).unwrap()
}

async fn feature_version(client: &TestClient) -> anyhow::Result<FeatureVersion> {
    let body = client
        .get("v1.cluster-admin.status")
        .await?
        .ensure(StatusCode::OK)?
        .json();
    Ok(body["feature_version"].assert_u64() as FeatureVersion)
}

#[tokio::test]
async fn feature_version_advances_when_all_voters_support_it() -> TestResult {
    // Every node supports up to v1, so the leader should raise the committed feature version to 1.
    let cluster = start_cluster_with(3, |_idx, cfg| {
        cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, 1));
        cfg.cluster.feature_version_advance_interval = fast_advance_interval();
    })
    .await;

    let client = cluster.leader_client().await;
    run_with_retries(async || {
        anyhow::ensure!(feature_version(client).await? == 1);
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn feature_version_does_not_advance_with_a_lagging_voter() -> TestResult {
    // Model a mid-upgrade cluster: one voter is stuck at v0 while the others support v1. As long as
    // that v0 voter stays in the committed membership it caps the target, so the cluster never
    // advances past 0. Keeping it as the initializing node means the cluster is never briefly at v1,
    // so every node can join.
    let cluster = start_cluster_with(3, |idx, cfg| {
        let max = if idx == 0 { 0 } else { 1 };
        cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, max));
        cfg.cluster.feature_version_advance_interval = fast_advance_interval();
    })
    .await;

    let client = cluster.leader_client().await;
    // Sample across many advance intervals. If the gate were wrong it would have advanced by now.
    for _ in 0..20 {
        assert_eq!(
            feature_version(client).await?,
            0,
            "feature version advanced despite a lagging voter"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

#[tokio::test]
async fn incompatible_node_is_refused() -> TestResult {
    // Every node supports v1, so the cluster advances its committed feature version to 1.
    let cluster = start_cluster_with(3, |_idx, cfg| {
        cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, 1));
        cfg.cluster.feature_version_advance_interval = fast_advance_interval();
    })
    .await;

    let leader = cluster.leader_client().await;
    run_with_retries(async || {
        anyhow::ensure!(feature_version(leader).await? == 1);
        Ok(())
    })
    .await?;

    // A node that only supports v0 must not join a cluster committed to v1.
    let cluster_peers: Vec<SocketAddr> = cluster
        .handles
        .values()
        .map(|node| node.repl_addr)
        .collect();
    let _newcomer = TestServerBuilder::with_default_config()
        .token(TEST_ADMIN_TOKEN.to_owned())
        .set_wait_for_initialization(false)
        .tap_cfg(move |cfg| {
            cfg.cluster.auto_initialize = false;
            cfg.cluster.seed_nodes = cluster_peers.iter().copied().map(Into::into).collect();
            cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, 0));
        })
        .build_uninitialized()
        .await;

    // The newcomer retries discovery and is refused each time, so the committed membership never
    // grows beyond the original three voters.
    for _ in 0..20 {
        let body = leader
            .get("v1.cluster-admin.status")
            .await?
            .ensure(StatusCode::OK)?
            .json();
        assert_eq!(
            body["nodes"].assert_array().len(),
            3,
            "an incompatible node joined the cluster"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

#[tokio::test]
async fn feature_version_survives_snapshot_install() -> TestResult {
    // A single node that supports v1 advances to feature version 1, then snapshots and purges its
    // logs so a later joiner can only catch up by installing that snapshot.
    let s1 = TestServerBuilder::with_default_config()
        .token(TEST_ADMIN_TOKEN.to_owned())
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
            cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, 1));
            cfg.cluster.feature_version_advance_interval = fast_advance_interval();
            cfg.cluster.minimum_snapshot_interval = NonZeroDurationMs::from_secs(3).unwrap();
            cfg.cluster.snapshot_after_writes = 10.into();
        })
        .build()
        .await;

    let client = &s1.client;
    run_with_retries(async || {
        anyhow::ensure!(feature_version(client).await? == 1);
        Ok(())
    })
    .await?;

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

    run_with_retries(async || {
        let body = client
            .get("v1.cluster-admin.status")
            .await?
            .ensure(StatusCode::OK)?
            .json();
        anyhow::ensure!(body["this_node_last_purged_log_index"].is_u64());
        Ok(())
    })
    .await?;

    // A fresh node joins and can only catch up from the snapshot, so it must adopt the committed
    // feature version carried in that snapshot.
    let s2 = TestServerBuilder::with_default_config()
        .token(TEST_ADMIN_TOKEN.to_owned())
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
            cfg.cluster.supported_feature_versions = Some(VersionRange::new(0, 1));
            cfg.cluster.feature_version_advance_interval = fast_advance_interval();
            cfg.cluster.seed_nodes = vec![s1.repl_addr.into()];
        })
        .build()
        .await;

    run_with_retries(async || {
        anyhow::ensure!(feature_version(&s2.client).await? == 1);
        Ok(())
    })
    .await?;

    // The snapshot must also have carried the actual data, so the key/value pairs are readable from
    // the freshly synced node.
    let body = s2
        .client
        .post("v1.kv.get")
        .json(json!({ "key": "key7" }))
        .await?
        .ensure(StatusCode::OK)?
        .json();
    assert_eq!(body["value"].assert_bytes(), 7u32.to_be_bytes().to_vec());

    Ok(())
}
