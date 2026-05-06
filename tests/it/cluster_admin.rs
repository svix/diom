use diom_backend::core::cluster::NodeId;
use serde_json::json;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, TestServerBuilder, start_cluster},
};

#[tokio::test]
async fn test_cluster_status() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        node_id,
        cluster_id,
        repl_addr,
        ..
    } = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.cluster.name = "example cluster".to_owned();
        })
        .build()
        .await;

    let cluster_status = client
        .get("v1.cluster-admin.status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(
        cluster_status["cluster_id"].assert_str(),
        cluster_id.to_string()
    );
    assert_eq!(
        cluster_status["cluster_name"].assert_str(),
        "example cluster"
    );
    assert_eq!(
        cluster_status["this_node_id"].assert_str(),
        node_id.to_string()
    );
    assert_eq!(cluster_status["this_node_state"].assert_str(), "leader");

    let nodes = cluster_status["nodes"].assert_array();
    assert_eq!(nodes.len(), 1);
    let node = &nodes[0];
    assert_eq!(node["address"].assert_str(), format!("http://{repl_addr}"));

    Ok(())
}

#[tokio::test]
async fn test_cluster_remove() -> TestResult {
    let context = start_cluster(2).await;

    let leader_id = context.get_leader_id().await;
    let follower_id = context.get_follower_id().await;
    let leader_client = &context.handles[&leader_id].client;

    // now remove the second node
    let resp = leader_client
        .post("v1.cluster-admin.remove-node")
        .json(json!({"node_id": follower_id}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(resp["node_id"], follower_id.to_string());

    let cluster_status = leader_client
        .get("v1.cluster-admin.status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cluster_status["nodes"].assert_array().len(), 1);
    assert_eq!(cluster_status["nodes"][0]["node_id"], leader_id.to_string());

    Ok(())
}

#[tokio::test]
async fn test_cluster_force_snapshot() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        ..
    } = TestServerBuilder::with_default_config().build().await;

    // don't race with the startup processes
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let cluster_status = client
        .get("v1.cluster-admin.status")
        .await?
        .expect(StatusCode::OK)
        .json();
    // this is usually null but it's possible a a snapshot will be triggered here
    let previous_snapshot = &cluster_status["this_node_last_snapshot_id"];

    // do some write so that the txn ID increases
    client
        .post("v1.kv.set")
        .json(json!({
            "key": "foo",
            "ttl": 900,
            "value": b"bar"
        }))
        .await?
        .ensure(StatusCode::OK)?;

    let response = client
        .post("v1.cluster-admin.force-snapshot")
        .json(json!({}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert!(response["snapshot_log_index"].is_number());
    assert!(response["snapshot_time"].is_i64());

    let later_cluster_status = client
        .get("v1.cluster-admin.status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_ne!(
        &later_cluster_status["this_node_last_snapshot_id"],
        previous_snapshot
    );

    Ok(())
}

#[tokio::test]
async fn test_cluster_force_election() -> TestResult {
    let context = start_cluster(2).await;
    let leader = context.get_leader_id().await;
    let leader_client = context.leader_client().await;
    let follower_client = context.follower_client().await;

    // make sure at least one write occurs
    leader_client
        .post("v1.kv.set")
        .json(json!({"key": "foo", "value" :"bar"}))
        .await?
        .expect(StatusCode::OK);

    // now trigger an election
    let resp = follower_client
        .post("v1.cluster-admin.force-election")
        .json(json!({}))
        .await?
        .expect(StatusCode::OK)
        .json();
    // technically, an election isn't guaranteed to change the leader
    let previous_leader: NodeId = resp["previous_leader_id"].assert_str().parse().unwrap();
    assert_eq!(previous_leader, leader);
    let new_leader: NodeId = resp["new_leader_id"].assert_str().parse().unwrap();
    assert!(context.node_ids().contains(&new_leader));

    Ok(())
}
