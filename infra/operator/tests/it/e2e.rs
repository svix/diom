use k8s_openapi::apimachinery::pkg::{api::resource::Quantity, util::intstr::IntOrString};
use kube::api::PostParams;
use std::time::Duration;
use test_utils::{
    JsonFastAndLoose as _, TestResult,
    retry::{run_with_many_retries, run_with_retries, run_with_timeout},
};

use crate::common::{TestContextBuilder, has_owner};

#[tokio::test]
async fn test_basic_creation() -> TestResult {
    let env = TestContextBuilder::new().build().await;
    let uid = env.cluster.metadata.uid.as_deref().unwrap();

    let sts_json = serde_json::to_value(&env.sts).unwrap();
    assert_eq!(sts_json["spec"]["replicas"].assert_u64(), 1);
    assert_eq!(
        sts_json["spec"]["template"]["spec"]["containers"][0]["image"].assert_str(),
        crate::common::E2E_IMAGE,
    );

    env.svc_api().get(env.name()).await?;
    env.svc_api()
        .get(&format!("{}-headless", env.name()))
        .await?;

    assert!(has_owner(&env.sts.metadata, uid));
    assert!(has_owner(
        &env.svc_api().get(env.name()).await?.metadata,
        uid
    ));
    assert!(has_owner(
        &env.svc_api()
            .get(&format!("{}-headless", env.name()))
            .await?
            .metadata,
        uid
    ));

    env.wait_for_ready_pods(1).await?;

    run_with_retries(async || {
        let cluster = env.cluster_api().get(env.name()).await?;
        let cluster_json = serde_json::to_value(&cluster).unwrap();
        anyhow::ensure!(cluster_json["status"]["phase"] == "Running");
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_replica_update() -> TestResult {
    let env = TestContextBuilder::new().build().await;

    // No PDB at 1 replica.
    assert!(env.pdb_api().get(env.name()).await.is_err());

    let mut cluster = env.cluster_api().get(env.name()).await?;
    cluster.spec.diom.replicas = 3;
    env.cluster_api()
        .replace(env.name(), &PostParams::default(), &cluster)
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get(env.name()).await?;
        anyhow::ensure!(sts.spec.as_ref().unwrap().replicas == Some(3));
        Ok(())
    })
    .await?;

    run_with_retries(async || {
        let pdb = env.pdb_api().get(env.name()).await?;
        anyhow::ensure!(pdb.spec.as_ref().unwrap().min_available == Some(IntOrString::Int(2)));
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_degraded_on_bad_image() -> TestResult {
    let env = TestContextBuilder::new().replicas(3).build().await;

    env.wait_for_ready_pods_timeout(3, Duration::from_secs(180))
        .await?;

    let mut cluster = env.cluster_api().get(env.name()).await?;
    cluster.spec.image = format!("{}:nonexistent", crate::common::E2E_IMAGE);
    env.cluster_api()
        .replace(env.name(), &PostParams::default(), &cluster)
        .await?;

    run_with_timeout(
        async || {
            let cluster = env.cluster_api().get(env.name()).await?;
            let cluster_json = serde_json::to_value(&cluster).unwrap();
            anyhow::ensure!(cluster_json["status"]["phase"] == "Degraded");
            Ok(())
        },
        Duration::from_secs(60),
    )
    .await?;

    Ok(())
}

// TODO: The default storage driver in kind doesn't support
// storage resizing, so we can't currently validate that the PVCs
// themselves have actually been resized.
#[tokio::test]
async fn test_storage_resize() -> TestResult {
    let env = TestContextBuilder::new().build().await;

    let mut cluster = env.cluster_api().get(env.name()).await?;
    cluster.spec.diom.storage.persistent.size = Quantity("20M".to_string());
    env.cluster_api()
        .replace(env.name(), &PostParams::default(), &cluster)
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get(env.name()).await?;
        anyhow::ensure!(sts.metadata.deletion_timestamp.is_some());
        Ok(())
    })
    .await?;

    run_with_many_retries(async || {
        let sts = env.sts_api().get(env.name()).await?;
        anyhow::ensure!(sts.metadata.deletion_timestamp.is_none());
        let sts_json = serde_json::to_value(&sts).unwrap();
        let vcts = sts_json["spec"]["volumeClaimTemplates"].assert_array();
        let persistent = vcts
            .iter()
            .find(|t| t["metadata"]["name"] == "persistent")
            .unwrap();
        let size = persistent["spec"]["resources"]["requests"]["storage"].assert_str();
        anyhow::ensure!(size == "20M");
        Ok(())
    })
    .await?;

    Ok(())
}
