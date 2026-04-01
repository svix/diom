// schemars schema_with proc macro expansion triggers a false positive for this lint
#![allow(unused_qualifications)]

use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::{
    Affinity, EnvVar, SecretKeySelector, Toleration, TopologySpreadConstraint,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub(crate) const DEFAULT_API_PORT: u16 = 8080;

fn default_api_port() -> u16 {
    DEFAULT_API_PORT
}

fn default_nodes() -> i32 {
    1
}

/// A Diom cluster deployment.
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "diom.svix.com",
    version = "v1alpha1",
    kind = "DiomCluster",
    namespaced,
    status = "DiomClusterStatus",
    shortname = "cc",
    printcolumn = r#"{"name":"Nodes","type":"integer","jsonPath":".spec.nodes"}"#,
    printcolumn = r#"{"name":"Phase","type":"string","jsonPath":".status.phase"}"#,
    printcolumn = r#"{"name":"Ready","type":"integer","jsonPath":".status.readyReplicas"}"#
)]
pub(crate) struct DiomClusterSpec {
    /// Number of Diom nodes. Should be odd for quorum (1, 3, 5...).
    #[serde(default = "default_nodes")]
    #[schemars(schema_with = "nodes_schema")]
    pub nodes: i32,

    /// Container image to deploy.
    pub image: String,

    /// Image pull policy (Always, IfNotPresent, Never).
    #[serde(default)]
    pub image_pull_policy: Option<String>,

    /// Storage configuration.
    pub storage: StorageSpec,

    /// Cluster/replication configuration.
    #[serde(default)]
    pub cluster: ClusterSpec,

    /// Configuration for the externally-facing Service.
    #[serde(default)]
    pub service: ServiceSpec,

    /// Port for the external API. The inter-node port is this value + 10000.
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    /// Additional environment variables to inject into pods.
    /// Follows the Kubernetes EnvVar API spec (v1.EnvVar): supports plain `value`
    /// and `valueFrom` (secretKeyRef, configMapKeyRef, fieldRef, resourceFieldRef)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_var: Vec<EnvVar>,

    /// CPU and memory resource requests and limits for the diom container.
    #[serde(default)]
    pub resources: ResourcesSpec,

    /// Additional annotations to add to pods.
    #[serde(default)]
    pub pod_annotations: BTreeMap<String, String>,

    /// Bootstrap script to run on cluster startup.
    /// Currently a YAML file defining namespaces to pre-create; may become a shell script in future.
    /// Mounted into pods and passed to the server via DIOM_BOOTSTRAP_CFG_PATH.
    #[serde(default)]
    pub bootstrap: Option<String>,

    /// Topology spread constraints for pod scheduling.
    /// Use this to spread pods across availability zones or nodes.
    /// See: https://kubernetes.io/docs/concepts/scheduling-eviction/topology-spread-constraints/
    #[serde(default)]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,

    /// Node selector for scheduling pods onto nodes with matching labels.
    #[serde(default)]
    pub node_selector: Option<BTreeMap<String, String>>,

    /// Tolerations to allow pods to be scheduled onto nodes with matching taints.
    #[serde(default)]
    pub tolerations: Option<Vec<Toleration>>,

    /// Affinity rules for advanced pod scheduling (node affinity, pod affinity/anti-affinity).
    #[serde(default)]
    pub affinity: Option<Affinity>,

    /// Admin token for privileged API access.
    /// Set either `value` for a plain string or `valueFrom` to pull from a Kubernetes Secret.
    /// Using a plain string value is only recommended for testing. Use `valueFrom` in all other cases.
    #[serde(default)]
    pub admin_token: Option<AdminTokenSource>,
}

/// Storage configuration for a Diom cluster.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub(crate) struct StorageSpec {
    /// Persistent database storage (fjall persistent DB).
    pub persistent: VolumeSpec,

    // TODO: ephemeral DB storage — fjall ephemeral DB
    // pub ephemeral: VolumeSpec,
    /// Separate volume for Raft commit logs.
    /// Recommended for high-throughput deployments to avoid I/O contention
    /// with the persistent DB.
    #[serde(default)]
    pub logs: Option<VolumeSpec>,

    /// Separate volume for Raft snapshots.
    /// Must be at least as large as persistent + ephemeral DB combined.
    #[serde(default)]
    pub snapshots: Option<VolumeSpec>,
}

/// Configuration for a single persistent volume.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VolumeSpec {
    /// Storage size in Kubernetes Quantity format, e.g. "10Gi".
    pub size: String,

    /// Storage class name. Uses the cluster default if not specified.
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// Cluster/replication configuration.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClusterSpec {
    /// Reference to a Secret containing the inter-node authentication token.
    /// `name` is the name of the Kubernetes Secret.
    /// `key` is the key within the Secret whose value is the token.
    #[serde(default)]
    pub secret_ref: Option<SecretKeySelector>,

    /// Heartbeat interval in milliseconds.
    #[serde(default)]
    pub heartbeat_interval_ms: Option<u64>,

    /// Minimum election timeout in milliseconds.
    #[serde(default)]
    pub election_timeout_min_ms: Option<u64>,

    /// Maximum election timeout in milliseconds.
    #[serde(default)]
    pub election_timeout_max_ms: Option<u64>,

    /// Replication request timeout in milliseconds.
    #[serde(default)]
    pub replication_request_timeout_ms: Option<u64>,

    /// Trigger a background snapshot after this many writes.
    #[serde(default)]
    pub snapshot_after_writes: Option<u32>,

    /// Trigger a background snapshot after this many milliseconds.
    #[serde(default)]
    pub snapshot_after_ms: Option<u64>,

    /// Log level (info, debug, trace). Defaults to info.
    #[serde(default)]
    pub log_level: Option<String>,
}

/// Configuration for the client-facing Service.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub(crate) struct ServiceSpec {
    /// Service type (ClusterIP, LoadBalancer, NodePort). Defaults to ClusterIP.
    #[serde(default)]
    pub r#type: Option<String>,

    /// Additional annotations for the Service (e.g. for cloud load balancer configuration).
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
}

/// Status of a DiomCluster.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiomClusterStatus {
    #[serde(default)]
    pub phase: Phase,

    #[serde(default)]
    pub ready_replicas: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub(crate) enum Phase {
    #[default]
    Initializing,
    Running,
    Degraded,
}

/// Source for the admin token.
/// Set either `value` for a plain string or `valueFrom` to reference a Kubernetes Secret.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
#[serde(untagged)]
pub(crate) enum AdminTokenSource {
    Value {
        value: String,
    },
    ValueFrom {
        #[serde(rename = "valueFrom")]
        value_from: SecretKeySelector,
    },
}

/// CPU and memory resource requests/limits for the diom container.
/// Values are in standard Kubernetes quantity format, e.g. "500m", "1", "512Mi", "2Gi".
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResourcesSpec {
    /// Resource requests — the minimum guaranteed resources for the container.
    #[serde(default)]
    pub requests: Option<BTreeMap<String, String>>,

    /// Resource limits — the maximum resources the container may use.
    #[serde(default)]
    pub limits: Option<BTreeMap<String, String>>,
}

impl DiomClusterSpec {
    pub(crate) fn cluster_port(&self) -> u16 {
        self.api_port + 10000
    }
}

fn nodes_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "integer",
        "format": "int32",
        "default": 1,
        "minimum": 1.0,
        "description": "Number of Diom nodes. Should be odd for quorum (1, 3, 5...)."
    })
}
