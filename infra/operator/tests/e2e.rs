use diom_operator::crd::DiomCluster;
use envtest::Environment;
use k8s_openapi::{
    api::{apps::v1::StatefulSet, core::v1::Service, policy::v1::PodDisruptionBudget},
    apimachinery::pkg::{api::resource::Quantity, util::intstr::IntOrString},
};
use kube::{
    Api, Client, CustomResourceExt,
    api::{Patch, PatchParams, PostParams},
};
use std::time::Duration;
use test_utils::retry::run_with_retries;

struct TestContext {
    client: Client,
    _handle: tokio::task::JoinHandle<anyhow::Result<()>>,
    _server: envtest::Server,
}

impl TestContext {
    fn cluster_api(&self) -> Api<DiomCluster> {
        Api::namespaced(self.client.clone(), "default")
    }
    fn sts_api(&self) -> Api<StatefulSet> {
        Api::namespaced(self.client.clone(), "default")
    }
    fn svc_api(&self) -> Api<Service> {
        Api::namespaced(self.client.clone(), "default")
    }
    fn pdb_api(&self) -> Api<PodDisruptionBudget> {
        Api::namespaced(self.client.clone(), "default")
    }
}

async fn setup() -> TestContext {
    let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    let server = Environment::default()
        .with_crds(DiomCluster::crd())
        .unwrap()
        .create()
        .await
        .unwrap();
    let client = server.client().unwrap();
    let _handle = tokio::spawn({
        let client = client.clone();
        async move { diom_operator::run_with_requeue(client, Duration::from_secs(2)).await }
    });
    TestContext {
        client,
        _handle,
        _server: server,
    }
}

async fn create_cluster(
    env: &TestContext,
    name: &str,
    replicas: i32,
    image: &str,
) -> anyhow::Result<DiomCluster> {
    Ok(env
        .cluster_api()
        .create(
            &PostParams::default(),
            &serde_json::from_value(serde_json::json!({
                "apiVersion": "diom.svix.com/v1",
                "kind": "DiomCluster",
                "metadata": { "name": name },
                "spec": {
                    "image": image,
                    "replicas": replicas,
                    "storage": { "persistent": { "size": "100M" } }
                }
            }))?,
        )
        .await?)
}

async fn wait_for_sts(env: &TestContext, name: &str) -> anyhow::Result<StatefulSet> {
    run_with_retries(async || Ok(env.sts_api().get(name).await?)).await
}

fn has_owner(meta: &k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta, uid: &str) -> bool {
    meta.owner_references
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .any(|r| r.uid == uid && r.controller == Some(true))
}

#[tokio::test]
async fn test_basic_creation() -> anyhow::Result<()> {
    let env = setup().await;
    create_cluster(&env, "test-cluster", 1, "test-image:latest").await?;

    let sts = wait_for_sts(&env, "test-cluster").await?;
    let sts_spec = sts.spec.as_ref().unwrap();
    assert_eq!(sts_spec.replicas, Some(1));
    assert_eq!(
        sts_spec.template.spec.as_ref().unwrap().containers[0]
            .image
            .as_deref(),
        Some("test-image:latest"),
    );

    env.svc_api().get("test-cluster").await?;
    env.svc_api().get("test-cluster-headless").await?;

    Ok(())
}

#[tokio::test]
async fn test_owner_references() -> anyhow::Result<()> {
    let env = setup().await;
    let cluster = create_cluster(&env, "test-cluster", 1, "test-image:latest").await?;
    let uid = cluster.metadata.uid.as_deref().unwrap();

    wait_for_sts(&env, "test-cluster").await?;
    run_with_retries(async || {
        env.svc_api()
            .get("test-cluster")
            .await
            .map(|_| ())
            .map_err(Into::into)
    })
    .await?;
    run_with_retries(async || {
        env.svc_api()
            .get("test-cluster-headless")
            .await
            .map(|_| ())
            .map_err(Into::into)
    })
    .await?;

    assert!(has_owner(
        &env.sts_api().get("test-cluster").await?.metadata,
        uid
    ));
    assert!(has_owner(
        &env.svc_api().get("test-cluster").await?.metadata,
        uid
    ));
    assert!(has_owner(
        &env.svc_api().get("test-cluster-headless").await?.metadata,
        uid
    ));

    Ok(())
}

#[tokio::test]
async fn test_replica_update() -> anyhow::Result<()> {
    let env = setup().await;
    create_cluster(&env, "test-cluster", 1, "test-image:latest").await?;
    wait_for_sts(&env, "test-cluster").await?;

    let mut cluster = env.cluster_api().get("test-cluster").await?;
    cluster.spec.diom.replicas = 3;
    env.cluster_api()
        .replace("test-cluster", &PostParams::default(), &cluster)
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get("test-cluster").await?;
        anyhow::ensure!(
            sts.spec.as_ref().unwrap().replicas == Some(3),
            "replicas not yet updated"
        );
        Ok(())
    })
    .await?;

    run_with_retries(async || {
        let pdb = env.pdb_api().get("test-cluster").await?;
        anyhow::ensure!(
            pdb.spec.as_ref().unwrap().min_available == Some(IntOrString::Int(2)),
            "PDB not yet correct"
        );
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_image_update() -> anyhow::Result<()> {
    let env = setup().await;
    create_cluster(&env, "test-cluster", 1, "test-image:v1").await?;
    wait_for_sts(&env, "test-cluster").await?;

    let mut cluster = env.cluster_api().get("test-cluster").await?;
    cluster.spec.image = "test-image:v2".to_string();
    env.cluster_api()
        .replace("test-cluster", &PostParams::default(), &cluster)
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get("test-cluster").await?;
        let image = sts
            .spec
            .as_ref()
            .unwrap()
            .template
            .spec
            .as_ref()
            .unwrap()
            .containers[0]
            .image
            .as_deref();
        anyhow::ensure!(
            image == Some("test-image:v2"),
            "image not yet updated: {image:?}"
        );
        Ok(())
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_storage_resize() -> anyhow::Result<()> {
    let env = setup().await;
    create_cluster(&env, "test-cluster", 1, "test-image:latest").await?;
    wait_for_sts(&env, "test-cluster").await?;

    let mut cluster = env.cluster_api().get("test-cluster").await?;
    cluster.spec.diom.storage.persistent.size = Quantity("200M".to_string());
    env.cluster_api()
        .replace("test-cluster", &PostParams::default(), &cluster)
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get("test-cluster").await?;
        anyhow::ensure!(
            sts.metadata.deletion_timestamp.is_some(),
            "STS not yet orphaned"
        );
        Ok(())
    })
    .await?;

    // Manually clear the finalizers b/c envtest is not great
    env.sts_api()
        .patch(
            "test-cluster",
            &PatchParams::default(),
            &Patch::Merge(serde_json::json!({ "metadata": { "finalizers": [] } })),
        )
        .await?;

    run_with_retries(async || {
        let sts = env.sts_api().get("test-cluster").await?;
        anyhow::ensure!(
            sts.metadata.deletion_timestamp.is_none(),
            "old STS still terminating"
        );
        let size = sts
            .spec
            .as_ref()
            .unwrap()
            .volume_claim_templates
            .as_ref()
            .unwrap()
            .iter()
            .find(|t| t.metadata.name.as_deref() == Some("persistent"))
            .unwrap()
            .spec
            .as_ref()
            .unwrap()
            .resources
            .as_ref()
            .unwrap()
            .requests
            .as_ref()
            .unwrap()
            .get("storage")
            .unwrap();
        anyhow::ensure!(
            size == &Quantity("200M".to_string()),
            "storage size not yet updated: {size:?}"
        );
        Ok(())
    })
    .await?;

    Ok(())
}
