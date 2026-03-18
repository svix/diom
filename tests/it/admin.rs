use diom::cfg::PeerAddr;
use maplit::btreeset;
use serde_json::json;
use std::collections::BTreeSet;
use test_utils::{
    JsonFastAndLoose as _, StatusCode, TestResult,
    server::{TestContext, TestServerBuilder},
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
        .get("admin/cluster/status")
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
    assert_eq!(cluster_status["this_node_state"].assert_str(), "Leader",);

    let nodes = cluster_status["nodes"].assert_array();
    assert_eq!(nodes.len(), 1);
    let node = &nodes[0];
    assert_eq!(node["address"].assert_str(), format!("http://{repl_addr}"));

    Ok(())
}

#[tokio::test]
async fn test_cluster_remove() -> TestResult {
    let TestContext {
        client,
        handle: _handle,
        node_id,
        repl_addr,
        ..
    } = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.cluster.auto_initialize = true;
            cfg.cluster.shut_down_on_go_away = false;
        })
        .build()
        .await;

    let TestContext {
        handle: _second_handle,
        node_id: second_node_id,
        ..
    } = TestServerBuilder::with_default_config()
        .tap_cfg(|cfg| {
            cfg.cluster.seed_nodes = vec![PeerAddr::from(repl_addr)];
            cfg.cluster.auto_initialize = false;
            cfg.cluster.shut_down_on_go_away = false;
        })
        .build()
        .await;

    // make sure both nodes are present
    let cluster_status = client
        .get("admin/cluster/status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cluster_status["nodes"].assert_array().len(), 2);
    let node_ids = cluster_status["nodes"]
        .assert_array()
        .iter()
        .map(|n| n["node_id"].assert_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        node_ids,
        btreeset! { node_id.to_string(), second_node_id.to_string() }
    );
    // ideally, the first node is still the leader before we evict the second node
    assert_eq!(cluster_status["this_node_state"], "Leader");

    // now remove the second node
    let resp = client
        .post("admin/cluster/remove-node")
        .json(json!({"node_id": second_node_id}))
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(resp["node_id"].assert_str(), second_node_id.to_string());

    let cluster_status = client
        .get("admin/cluster/status")
        .await?
        .expect(StatusCode::OK)
        .json();
    assert_eq!(cluster_status["nodes"].assert_array().len(), 1);
    assert_eq!(cluster_status["nodes"][0]["node_id"], node_id.to_string());

    Ok(())
}
