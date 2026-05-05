// schemars schema_with proc macro expansion triggers a false positive for this lint
#![allow(unused_qualifications)]

use k8s_openapi::{
    api::core::v1::{
        Affinity, EnvVar, EnvVarSource, HTTPGetAction, Probe, ResourceRequirements,
        SecretKeySelector, Toleration, TopologySpreadConstraint,
    },
    apimachinery::pkg::api::resource::Quantity,
};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, num::NonZeroU16};

pub const DEFAULT_API_PORT: u16 = 8624;
pub const INTRACLUSTER_PORT: u16 = 8625;

fn default_api_port() -> u16 {
    DEFAULT_API_PORT
}

fn default_replicas() -> i32 {
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
    printcolumn = r#"{"name":"Replicas","type":"integer","jsonPath":".spec.replicas"}"#,
    printcolumn = r#"{"name":"Phase","type":"string","jsonPath":".status.phase"}"#,
    printcolumn = r#"{"name":"Ready","type":"integer","jsonPath":".status.readyReplicas"}"#
)]
pub struct DiomClusterSpec {
    /// Cluster/replication configuration.
    #[serde(flatten)]
    pub diom: DiomSpec,

    /// Container image to deploy.
    pub image: String,

    /// Image pull policy (Always, IfNotPresent, Never).
    #[serde(default)]
    pub image_pull_policy: Option<String>,

    /// Configuration for the externally-facing Service.
    #[serde(default)]
    pub service: ServiceSpec,

    /// CPU and memory resource requests and limits for the diom container.
    #[serde(default)]
    pub resources: ResourceRequirements,

    /// Additional annotations to add to pods.
    #[serde(default)]
    pub pod_annotations: BTreeMap<String, String>,

    /// Topology spread constraints for pod scheduling.
    ///
    /// Constraints that target diom pods should use the `diom.svix.com/cluster` label
    /// (replacing `my-cluster` with the name of your DiomCluster resource).
    ///
    /// ```yaml
    /// labelSelector:
    ///   matchLabels:
    ///     diom.svix.com/cluster: my-cluster
    /// ```
    #[serde(default)]
    pub topology_spread_constraints: Vec<TopologySpreadConstraint>,

    /// Node selector for scheduling pods onto nodes with matching labels.
    #[serde(default)]
    pub node_selector: Option<BTreeMap<String, String>>,

    /// Tolerations to allow pods to be scheduled onto nodes with matching taints.
    #[serde(default)]
    pub tolerations: Option<Vec<Toleration>>,

    /// Affinity rules for advanced pod scheduling (node affinity, pod affinity/anti-affinity).
    ///
    /// Pod affinity/anti-affinity rules that target diom pods should use the
    /// `diom.svix.com/cluster` label (replacing `my-cluster` with the name of your
    /// DiomCluster resource).
    ///
    /// ```yaml
    /// labelSelector:
    ///   matchLabels:
    ///     diom.svix.com/cluster: my-cluster
    /// ```
    #[serde(default)]
    pub affinity: Option<Affinity>,

    /// Optional image pull secret to use when pulling the Diom image.
    #[serde(default)]
    pub image_pull_secrets: Option<Vec<ImagePullSecret>>,

    #[serde(default)]
    pub probes: ProbesSpec,
}

/// Storage configuration for a Diom cluster.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DiomStorageSpec {
    /// Persistent database storage
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
pub struct VolumeSpec {
    /// Storage size, e.g. "10Gi".
    pub size: Quantity,

    /// Storage class name. Uses the cluster default if not specified.
    #[serde(default)]
    pub storage_class: Option<String>,
}

/// Diom configuration.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiomSpec {
    /// Number of Diom replicas.
    ///
    /// Should be an odd number. Recommended value is 3, or 1 if you want only a single node.
    #[serde(default = "default_replicas")]
    #[schemars(schema_with = "replicas_schema")]
    pub replicas: i32,

    /// Port for the external API and service.
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    /// Inter-node authentication token.
    ///
    /// Set either `value` for a plain string or `valueFrom` to pull from a Kubernetes Secret.
    /// Using a plain string value is only recommended for testing. Use `valueFrom` in all other cases.
    #[serde(default)]
    pub internode_secret: Option<ValueOrSecretRef>,

    /// The log level to run the service with. Supported: info, debug, trace
    #[serde(default)]
    pub log_level: Option<String>,

    /// The log format that all output will follow. Supported: default, json
    #[serde(default)]
    pub log_format: Option<String>,

    /// Newline-delimited bootstrap script to run on cluster startup.
    #[serde(default)]
    pub bootstrap: Option<String>,

    /// Admin token for privileged API access.
    ///
    /// Set either `value` for a plain string or `valueFrom` to pull from a Kubernetes Secret.
    /// Using a plain string value is only recommended for testing. Use `valueFrom` in all other cases.
    #[serde(default)]
    pub admin_token: Option<ValueOrSecretRef>,

    /// Additional environment variables to inject into pods.
    /// Follows the Kubernetes EnvVar API spec (v1.EnvVar): supports plain `value`
    /// and `valueFrom` (secretKeyRef, configMapKeyRef, fieldRef, resourceFieldRef)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_var: Vec<EnvVar>,

    /// Storage configuration.
    pub storage: DiomStorageSpec,

    #[serde(default)]
    pub opentelemetry: Option<OpenTelemetrySpec>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum OpenTelemetryProtocol {
    #[default]
    Grpc,
    Http,
}

impl fmt::Display for OpenTelemetryProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grpc => write!(f, "grpc"),
            Self::Http => write!(f, "http"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenTelemetrySpec {
    /// The OpenTelemetry address to send events to if given.
    ///
    /// Currently only GRPC exports are supported.
    #[serde(default)]
    pub address: Option<String>,

    /// The OpenTelemetry address to send metrics to if given.
    ///
    /// If not specified, the server will attempt to fall back
    /// to `opentelemetry_address`.
    #[serde(default)]
    pub metrics_address: Option<String>,

    /// OpenTelemetry metrics protocol
    ///
    /// By default, metrics are sent via GRPC. Some metrics destinations, most
    /// notably Prometheus, only support receiving metrics via HTTP.
    #[serde(default)]
    pub metrics_protocol: Option<OpenTelemetryProtocol>,
}

/// Configuration for the client-facing Service.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct ServiceSpec {
    /// Service type (ClusterIP, LoadBalancer, NodePort). Defaults to ClusterIP.
    #[serde(default)]
    pub r#type: Option<String>,

    /// Additional annotations for the Service (e.g. for cloud load balancer configuration).
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
}

/// Configuration for the pod Probes (healthchecks)
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProbesSpec {
    #[serde(default = "ProbeSpec::default_liveness")]
    pub liveness: ProbeSpec,

    #[serde(default = "ProbeSpec::default_readiness")]
    pub readiness: ProbeSpec,

    #[serde(default = "ProbeSpec::default_startup")]
    pub startup: ProbeSpec,
}

impl Default for ProbesSpec {
    fn default() -> Self {
        Self {
            liveness: ProbeSpec::default_liveness(),
            readiness: ProbeSpec::default_readiness(),
            startup: ProbeSpec::default_startup(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
/// Configuration for an individual probe
pub struct ProbeSpec {
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub initial_delay_seconds: NonZeroU16,
    /// How often (in seconds) to perform the probe.
    pub period_seconds: NonZeroU16,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded.
    pub failure_threshold: NonZeroU16,
    /// Minimum consecutive successes for the probe to be considered successful after having failed.
    pub success_threshold: NonZeroU16,
    /// Number of seconds after which the probe times out.
    pub timeout_seconds: NonZeroU16,
}

impl ProbeSpec {
    const fn default_liveness() -> Self {
        Self {
            initial_delay_seconds: NonZeroU16::new(5).unwrap(),
            period_seconds: NonZeroU16::new(5).unwrap(),
            failure_threshold: NonZeroU16::new(2).unwrap(),
            success_threshold: NonZeroU16::new(1).unwrap(),
            timeout_seconds: NonZeroU16::new(1).unwrap(),
        }
    }

    const fn default_readiness() -> Self {
        Self {
            initial_delay_seconds: NonZeroU16::new(15).unwrap(),
            period_seconds: NonZeroU16::new(10).unwrap(),
            failure_threshold: NonZeroU16::new(2).unwrap(),
            success_threshold: NonZeroU16::new(1).unwrap(),
            timeout_seconds: NonZeroU16::new(1).unwrap(),
        }
    }

    const fn default_startup() -> Self {
        Self {
            initial_delay_seconds: NonZeroU16::new(15).unwrap(),
            period_seconds: NonZeroU16::new(10).unwrap(),
            failure_threshold: NonZeroU16::new(120).unwrap(),
            success_threshold: NonZeroU16::new(1).unwrap(),
            timeout_seconds: NonZeroU16::new(1).unwrap(),
        }
    }

    fn nzu16asi32(value: NonZeroU16) -> i32 {
        let value: u16 = value.into();
        value.into()
    }

    pub fn as_probe_with_http_get(&self, http_get: HTTPGetAction) -> Probe {
        Probe {
            http_get: Some(http_get),
            initial_delay_seconds: Some(Self::nzu16asi32(self.initial_delay_seconds)),
            period_seconds: Some(Self::nzu16asi32(self.period_seconds)),
            failure_threshold: Some(Self::nzu16asi32(self.failure_threshold)),
            success_threshold: Some(Self::nzu16asi32(self.success_threshold)),
            timeout_seconds: Some(Self::nzu16asi32(self.timeout_seconds)),
            ..Default::default()
        }
    }
}

/// Status of a DiomCluster.
#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiomClusterStatus {
    #[serde(default)]
    pub phase: Phase,

    #[serde(default)]
    pub ready_replicas: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema, PartialEq)]
pub enum Phase {
    #[default]
    Initializing,
    Running,
    Degraded,
}

/// Source for the admin token.
/// Set either `value` for a plain string or `valueFrom` to reference a Kubernetes Secret.
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum ValueOrSecretRef {
    Value {
        value: String,
    },
    ValueFrom {
        #[serde(rename = "valueFrom")]
        value_from: SecretKeySelector,
    },
}

impl ValueOrSecretRef {
    pub(crate) fn to_env_var(&self, name: &str) -> EnvVar {
        match self {
            ValueOrSecretRef::Value { value } => EnvVar {
                name: name.into(),
                value: Some(value.clone()),
                ..Default::default()
            },
            ValueOrSecretRef::ValueFrom { value_from } => EnvVar {
                name: name.into(),
                value_from: Some(EnvVarSource {
                    secret_key_ref: Some(value_from.clone()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ImagePullSecret {
    pub name: String,
}

fn replicas_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "integer",
        "format": "int32",
        "default": 1,
        "minimum": 1.0,
        "description": "Number of Diom replicas. Should be odd for quorum (1, 3, 5...)."
    })
}
