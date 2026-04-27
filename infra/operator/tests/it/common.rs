use diom_operator::crd::DiomCluster;
use k8s_openapi::{
    api::{
        apps::v1::StatefulSet,
        core::v1::{Pod, Service},
        policy::v1::PodDisruptionBudget,
    },
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    Api, Client, CustomResourceExt,
    api::{ListParams, PostParams},
};
use std::{
    process::Command,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};
use test_utils::retry::{run_with_retries, run_with_timeout};

pub(crate) const E2E_IMAGE: &str = "diom-e2e";

static E2E_IMAGE_BUILT: OnceLock<()> = OnceLock::new();

fn ensure_e2e_image_built() {
    E2E_IMAGE_BUILT.get_or_init(|| {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let repo_root = format!("{manifest_dir}/../..");
        let dockerfile = format!("{manifest_dir}/tests/Dockerfile.e2e");

        let status = Command::new("cargo")
            .args(["build", "-p", "diom-server", "--bin", "diom-server"])
            .current_dir(&repo_root)
            .status()
            .expect("failed to build diom-server");
        assert!(status.success(), "cargo build of diom-server failed");

        let binary_dir = format!("{repo_root}/target/debug");
        let status = Command::new("docker")
            .args(["build", "-t", E2E_IMAGE, "-f", &dockerfile, &binary_dir])
            .status()
            .expect("failed to run docker build for e2e image");
        assert!(status.success(), "docker build for e2e image failed");
    });
}

static CLUSTER_COUNTER: AtomicU32 = AtomicU32::new(0);
static CLUSTER_CREATION_LOCK: Mutex<()> = Mutex::new(());

struct KindClusterGuard(String);

impl Drop for KindClusterGuard {
    fn drop(&mut self) {
        let _ = Command::new("kind")
            .args(["delete", "cluster", "--name", &self.0])
            .output();
    }
}

pub(crate) struct TestContext {
    pub client: Client,
    pub cluster: DiomCluster,
    pub sts: StatefulSet,
    _handle: tokio::task::JoinHandle<anyhow::Result<()>>,
    _kind_cluster: KindClusterGuard,
}

#[allow(unused)]
impl TestContext {
    pub(crate) fn name(&self) -> &str {
        self.cluster.metadata.name.as_deref().unwrap()
    }

    pub(crate) fn cluster_api(&self) -> Api<DiomCluster> {
        Api::namespaced(self.client.clone(), "default")
    }

    pub(crate) fn sts_api(&self) -> Api<StatefulSet> {
        Api::namespaced(self.client.clone(), "default")
    }

    pub(crate) fn svc_api(&self) -> Api<Service> {
        Api::namespaced(self.client.clone(), "default")
    }

    pub(crate) fn pdb_api(&self) -> Api<PodDisruptionBudget> {
        Api::namespaced(self.client.clone(), "default")
    }

    pub(crate) fn pod_api(&self) -> Api<Pod> {
        Api::namespaced(self.client.clone(), "default")
    }

    pub(crate) async fn wait_for_sts(&self) -> anyhow::Result<StatefulSet> {
        run_with_retries(async || Ok(self.sts_api().get(self.name()).await?)).await
    }

    pub(crate) async fn wait_for_ready_pods(&self, expected: i32) -> anyhow::Result<()> {
        self.wait_for_ready_pods_timeout(expected, Duration::from_secs(60))
            .await
    }

    pub(crate) async fn wait_for_ready_pods_timeout(
        &self,
        expected: i32,
        timeout: Duration,
    ) -> anyhow::Result<()> {
        let lp = ListParams::default().labels(&format!("diom.svix.com/cluster={}", self.name()));
        run_with_timeout(
            async || {
                let pods = self.pod_api().list(&lp).await?;
                let ready = pods.items.iter().filter(|p| pod_is_ready(p)).count() as i32;
                anyhow::ensure!(ready >= expected, "{ready}/{expected} pods ready");
                Ok(())
            },
            timeout,
        )
        .await
    }
}

fn pod_is_ready(pod: &Pod) -> bool {
    pod.status
        .as_ref()
        .and_then(|s| s.conditions.as_deref())
        .unwrap_or(&[])
        .iter()
        .any(|c| c.type_ == "Ready" && c.status == "True")
}

pub(crate) struct TestContextBuilder {
    replicas: i32,
    image: String,
    storage_size: String,
    #[allow(clippy::disallowed_types)]
    extra_spec: serde_json::Map<String, serde_json::Value>,
}

impl Default for TestContextBuilder {
    fn default() -> Self {
        Self {
            replicas: 1,
            image: E2E_IMAGE.to_string(),
            storage_size: "10M".to_string(),
            extra_spec: serde_json::Map::new(),
        }
    }
}

#[allow(unused)]
impl TestContextBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn replicas(mut self, n: i32) -> Self {
        self.replicas = n;
        self
    }

    pub(crate) fn image(mut self, image: impl Into<String>) -> Self {
        self.image = image.into();
        self
    }

    pub(crate) fn storage_size(mut self, size: impl Into<String>) -> Self {
        self.storage_size = size.into();
        self
    }

    #[allow(clippy::disallowed_types)]
    pub(crate) fn spec_fields(mut self, fields: serde_json::Value) -> Self {
        if let serde_json::Value::Object(map) = fields {
            self.extra_spec.extend(map);
        }
        self
    }

    pub(crate) async fn build(self) -> TestContext {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
        let _ = rustls::crypto::ring::default_provider().install_default();
        if self.image == E2E_IMAGE {
            ensure_e2e_image_built();
        }

        let n = CLUSTER_COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let kind_cluster_name = format!("diom-e2e-{pid}-{n}");

        let kubeconfig_str = {
            let name = kind_cluster_name.clone();
            #[allow(clippy::disallowed_methods)]
            tokio::task::spawn_blocking(move || {
                let _guard = CLUSTER_CREATION_LOCK
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                let status = Command::new("kind")
                    .args(["create", "cluster", "--name", &name])
                    .status()
                    .expect("failed to run kind");
                assert!(status.success(), "kind create cluster failed");

                let output = Command::new("kind")
                    .args(["get", "kubeconfig", "--name", &name])
                    .output()
                    .expect("failed to get kind kubeconfig");
                assert!(output.status.success(), "kind get kubeconfig failed");
                String::from_utf8(output.stdout).unwrap()
            })
            .await
            .unwrap()
        };

        let kind_cluster = KindClusterGuard(kind_cluster_name.clone());

        let load_status = Command::new("kind")
            .args([
                "load",
                "docker-image",
                &self.image,
                "--name",
                &kind_cluster_name,
            ])
            .status()
            .expect("failed to load docker-image");
        assert!(load_status.success(),);

        let kubeconfig: kube::config::Kubeconfig = serde_yaml::from_str(&kubeconfig_str).unwrap();
        let config = kube::Config::from_custom_kubeconfig(kubeconfig, &Default::default())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();

        let crd_api: Api<CustomResourceDefinition> = Api::all(client.clone());
        crd_api
            .create(&PostParams::default(), &DiomCluster::crd())
            .await
            .unwrap();
        run_with_retries(async || {
            let crd = crd_api.get("diomclusters.diom.svix.com").await?;
            let crd_json = serde_json::to_value(&crd).unwrap();
            let established = crd_json["status"]["conditions"]
                .as_array()
                .is_some_and(|cs| {
                    cs.iter()
                        .any(|c| c["type"] == "Established" && c["status"] == "True")
                });
            anyhow::ensure!(established);
            Ok(())
        })
        .await
        .unwrap();

        let _handle = tokio::spawn({
            let client = client.clone();
            async move { diom_operator::run_with_requeue(client, Duration::from_secs(2)).await }
        });

        let cluster_api: Api<DiomCluster> = Api::namespaced(client.clone(), "default");
        let mut spec = serde_json::json!({
            "image": self.image,
            "imagePullPolicy": "Never",
            "replicas": self.replicas,
            "storage": { "persistent": { "size": self.storage_size } }
        });
        spec.as_object_mut().unwrap().extend(self.extra_spec);
        let cluster = cluster_api
            .create(
                &PostParams::default(),
                &serde_json::from_value(serde_json::json!({
                    "apiVersion": "diom.svix.com/v1alpha1",
                    "kind": "DiomCluster",
                    "metadata": { "name": "test-cluster" },
                    "spec": spec
                }))
                .unwrap(),
            )
            .await
            .unwrap();

        let sts_api: Api<StatefulSet> = Api::namespaced(client.clone(), "default");
        let sts = run_with_retries(async || Ok(sts_api.get("test-cluster").await?))
            .await
            .unwrap();

        TestContext {
            client,
            cluster,
            sts,
            _handle,
            _kind_cluster: kind_cluster,
        }
    }
}

pub(crate) fn has_owner(
    meta: &k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    uid: &str,
) -> bool {
    meta.owner_references
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .any(|r| r.uid == uid && r.controller == Some(true))
}
