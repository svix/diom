use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{StatefulSet, StatefulSetSpec},
        core::v1::{
            Affinity, Container, ContainerPort, EnvVar, EnvVarSource, HTTPGetAction,
            ObjectFieldSelector, PersistentVolumeClaim, PersistentVolumeClaimSpec,
            PodSecurityContext, PodSpec, PodTemplateSpec, Probe, ResourceRequirements, Toleration,
            TopologySpreadConstraint, VolumeMount, VolumeResourceRequirements,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity, apis::meta::v1::LabelSelector, util::intstr::IntOrString,
    },
};
use kube::{Resource, ResourceExt, core::ObjectMeta};

use crate::{
    crd::{CoyoteCluster, CoyoteClusterSpec},
    error::Result,
    labels,
    resources::services,
};

/// Path inside the container for the persistent DB.
const PERSISTENT_DATA_PATH: &str = "/data/persistent";

// TODO: path for ephemeral DB
// const EPHEMERAL_DATA_PATH: &str = "/data/ephemeral";

/// Path inside the container for Raft commit logs (when a separate volume is configured).
const LOGS_DATA_PATH: &str = "/data/logs";

/// Path inside the container for Raft snapshots (when a separate volume is configured).
const SNAPSHOTS_DATA_PATH: &str = "/data/snapshots";

pub(crate) fn build(cluster: &CoyoteCluster, ns: &str) -> Result<StatefulSet> {
    let cluster_name = cluster.name_any();
    let spec = &cluster.spec;
    let headless_svc = services::headless_name(&cluster_name);

    let env = build_env(spec, &cluster_name, &headless_svc, ns);
    let volume_claim_templates = build_volume_claim_templates(spec);
    let volume_mounts = build_volume_mounts(spec);
    let container = build_container(spec, env, volume_mounts);

    let mut pod_labels = labels::selector(&cluster_name);
    pod_labels.extend(labels::common(&cluster_name));
    let pod_annotations = spec.pod_annotations.clone();

    let topology_spread_constraints: Vec<TopologySpreadConstraint> =
        spec.topology_spread_constraints.clone();

    let node_selector: Option<BTreeMap<String, String>> = spec.node_selector.clone();
    let tolerations: Option<Vec<Toleration>> = spec.tolerations.clone();
    let affinity: Option<Affinity> = spec.affinity.clone();

    let pod_spec = PodSpec {
        containers: vec![container],
        volumes: None,
        // appuser in the coyote-server image is created with plain `useradd`, giving it UID/GID
        // 1000. fsGroup ensures mounted PVC directories are chowned to that group on attach.
        security_context: Some(PodSecurityContext {
            fs_group: Some(1000),
            ..Default::default()
        }),
        topology_spread_constraints: if topology_spread_constraints.is_empty() {
            None
        } else {
            Some(topology_spread_constraints)
        },
        node_selector,
        tolerations,
        affinity,
        ..Default::default()
    };

    Ok(StatefulSet {
        metadata: ObjectMeta {
            name: Some(cluster_name.clone()),
            namespace: Some(ns.into()),
            labels: Some(labels::common(&cluster_name)),
            owner_references: Some(vec![cluster.controller_owner_ref(&()).unwrap()]),
            ..Default::default()
        },
        spec: Some(StatefulSetSpec {
            replicas: Some(spec.nodes),
            service_name: Some(headless_svc),
            selector: LabelSelector {
                match_labels: Some(labels::selector(&cluster_name)),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(pod_labels),
                    annotations: if pod_annotations.is_empty() {
                        None
                    } else {
                        Some(pod_annotations)
                    },
                    ..Default::default()
                }),
                spec: Some(pod_spec),
            },
            volume_claim_templates: Some(volume_claim_templates),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn build_env(
    spec: &CoyoteClusterSpec,
    cluster_name: &str,
    headless_svc: &str,
    ns: &str,
) -> Vec<EnvVar> {
    let cluster_port = spec.cluster_port();

    // These must come before any vars that reference them via $(VAR) substitution.
    let mut env: Vec<EnvVar> = vec![
        // Downward API: pod name and namespace, used to construct stable DNS addresses.
        env_from_field("POD_NAME", "metadata.name"),
        env_from_field("POD_NAMESPACE", "metadata.namespace"),
        // Each pod advertises its stable StatefulSet DNS name so peers can reach it.
        // Uses k8s env var substitution: $(VAR) references earlier vars in this list.
        env_var(
            "COYOTE_CLUSTER_ADVERTISED_ADDRESS",
            format!("$(POD_NAME).{headless_svc}.$(POD_NAMESPACE).svc.cluster.local:{cluster_port}"),
        ),
        // Seed nodes: all pods in the StatefulSet by their stable DNS names.
        env_var(
            "COYOTE_CLUSTER_SEED_NODES",
            seed_nodes_value(cluster_name, headless_svc, ns, spec.nodes, cluster_port),
        ),
        // Allow any pod to initialize a new cluster if it can't find peers and has no state.
        // StatefulSets start pods sequentially (pod-0 first), so in practice pod-0 initializes
        // first. Setting this on all pods ensures the cluster can still form if pod-0 restarts.
        env_var("COYOTE_CLUSTER_AUTO_INITIALIZE", "true"),
        env_var(
            "COYOTE_LISTEN_ADDRESS",
            format!("0.0.0.0:{}", spec.api_port),
        ),
        env_var("COYOTE_PERSISTENT_DB_PATH", PERSISTENT_DATA_PATH),
    ];

    // Inter-node secret, if configured.
    if let Some(secret_ref) = &spec.cluster.secret_ref {
        env.push(EnvVar {
            name: "COYOTE_CLUSTER_SECRET".into(),
            value_from: Some(EnvVarSource {
                secret_key_ref: Some(k8s_openapi::api::core::v1::SecretKeySelector {
                    name: secret_ref.name.clone(),
                    key: secret_ref.key.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    // Always set log and snapshot paths explicitly to avoid the Dockerfile defaults, which point
    // to ephemeral storage and cause crashes on pod restart if Raft references a missing snapshot.
    env.push(env_var(
        "COYOTE_CLUSTER_LOG_PATH",
        if spec.storage.logs.is_some() {
            LOGS_DATA_PATH.into()
        } else {
            format!("{PERSISTENT_DATA_PATH}/logs")
        },
    ));
    env.push(env_var(
        "COYOTE_CLUSTER_SNAPSHOT_PATH",
        if spec.storage.snapshots.is_some() {
            SNAPSHOTS_DATA_PATH.into()
        } else {
            format!("{PERSISTENT_DATA_PATH}/snapshots")
        },
    ));

    // Optional cluster tuning parameters.
    push_opt_env(
        &mut env,
        "COYOTE_CLUSTER_HEARTBEAT_INTERVAL_MS",
        spec.cluster.heartbeat_interval_ms,
    );
    push_opt_env(
        &mut env,
        "COYOTE_CLUSTER_ELECTION_TIMEOUT_MIN_MS",
        spec.cluster.election_timeout_min_ms,
    );
    push_opt_env(
        &mut env,
        "COYOTE_CLUSTER_ELECTION_TIMEOUT_MAX_MS",
        spec.cluster.election_timeout_max_ms,
    );
    push_opt_env(
        &mut env,
        "COYOTE_CLUSTER_REPLICATION_REQUEST_TIMEOUT_MS",
        spec.cluster.replication_request_timeout_ms,
    );
    push_opt_env(
        &mut env,
        "COYOTE_SNAPSHOT_AFTER_WRITES",
        spec.cluster.snapshot_after_writes,
    );
    push_opt_env(
        &mut env,
        "COYOTE_CLUSTER_SNAPSHOT_AFTER_MS",
        spec.cluster.snapshot_after_ms,
    );

    if let Some(level) = &spec.cluster.log_level {
        env.push(env_var("COYOTE_LOG_LEVEL", level));
    }

    if let Some(bootstrap) = &spec.bootstrap {
        env.push(env_var("COYOTE_BOOTSTRAP_CFG", bootstrap));
    }

    // Extra user-provided env vars.
    for extra in &spec.extra_env {
        env.push(EnvVar {
            name: extra.name.clone(),
            value: extra.value.clone(),
            ..Default::default()
        });
    }

    env
}

fn build_container(
    spec: &CoyoteClusterSpec,
    env: Vec<EnvVar>,
    volume_mounts: Vec<VolumeMount>,
) -> Container {
    let cluster_port = spec.cluster_port();
    const API_HEALTH_ENDPOINT: &str = "/api/v1.health.ping";
    const CLUSTER_HEALTH_ENDPOINT: &str = "/repl/health";

    Container {
        name: "coyote".into(),
        image: Some(spec.image.clone()),
        image_pull_policy: spec.image_pull_policy.clone(),
        command: Some(vec!["/usr/local/bin/coyote-server".into()]),
        env: Some(env),
        ports: Some(vec![
            ContainerPort {
                name: Some("api".into()),
                container_port: spec.api_port as i32,
                ..Default::default()
            },
            ContainerPort {
                name: Some("cluster".into()),
                container_port: cluster_port as i32,
                ..Default::default()
            },
        ]),
        volume_mounts: Some(volume_mounts),
        resources: Some(ResourceRequirements {
            requests: spec.resources.requests.as_ref().map(|r| {
                r.iter()
                    .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                    .collect()
            }),
            limits: spec.resources.limits.as_ref().map(|l| {
                l.iter()
                    .map(|(k, v)| (k.clone(), Quantity(v.clone())))
                    .collect()
            }),
            ..Default::default()
        }),
        liveness_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some(API_HEALTH_ENDPOINT.into()),
                port: IntOrString::Int(spec.api_port as _),
                ..Default::default()
            }),
            initial_delay_seconds: Some(5),
            period_seconds: Some(10),
            failure_threshold: Some(6),
            success_threshold: Some(1),
            ..Default::default()
        }),
        readiness_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some(CLUSTER_HEALTH_ENDPOINT.into()),
                port: IntOrString::Int(cluster_port as _),
                ..Default::default()
            }),
            initial_delay_seconds: Some(5),
            period_seconds: Some(10),
            failure_threshold: Some(6),
            success_threshold: Some(1),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_volume_claim_templates(spec: &CoyoteClusterSpec) -> Vec<PersistentVolumeClaim> {
    let mut templates = vec![pvc_template(
        "persistent",
        &spec.storage.persistent.size,
        spec.storage.persistent.storage_class.as_deref(),
    )];

    if let Some(logs) = &spec.storage.logs {
        templates.push(pvc_template(
            "logs",
            &logs.size,
            logs.storage_class.as_deref(),
        ));
    }

    if let Some(snapshots) = &spec.storage.snapshots {
        templates.push(pvc_template(
            "snapshots",
            &snapshots.size,
            snapshots.storage_class.as_deref(),
        ));
    }

    templates
}

fn build_volume_mounts(spec: &CoyoteClusterSpec) -> Vec<VolumeMount> {
    let mut mounts = vec![VolumeMount {
        name: "persistent".into(),
        mount_path: PERSISTENT_DATA_PATH.into(),
        ..Default::default()
    }];

    if spec.storage.logs.is_some() {
        mounts.push(VolumeMount {
            name: "logs".into(),
            mount_path: LOGS_DATA_PATH.into(),
            ..Default::default()
        });
    }

    if spec.storage.snapshots.is_some() {
        mounts.push(VolumeMount {
            name: "snapshots".into(),
            mount_path: SNAPSHOTS_DATA_PATH.into(),
            ..Default::default()
        });
    }

    mounts
}

// --- Helpers ---

fn env_var(name: &str, value: impl Into<String>) -> EnvVar {
    EnvVar {
        name: name.into(),
        value: Some(value.into()),
        ..Default::default()
    }
}

fn env_from_field(name: &str, field_path: &str) -> EnvVar {
    EnvVar {
        name: name.into(),
        value_from: Some(EnvVarSource {
            field_ref: Some(ObjectFieldSelector {
                field_path: field_path.into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn push_opt_env(env: &mut Vec<EnvVar>, name: &str, value: Option<impl ToString>) {
    if let Some(v) = value {
        env.push(env_var(name, v.to_string()));
    }
}

fn pvc_template(name: &str, size: &str, storage_class: Option<&str>) -> PersistentVolumeClaim {
    let mut resources: BTreeMap<String, Quantity> = BTreeMap::new();
    resources.insert("storage".into(), Quantity(size.into()));

    PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(name.into()),
            ..Default::default()
        },
        spec: Some(PersistentVolumeClaimSpec {
            access_modes: Some(vec!["ReadWriteOnce".into()]),
            storage_class_name: storage_class.map(str::to_string),
            resources: Some(VolumeResourceRequirements {
                requests: Some(resources),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn seed_nodes_value(
    cluster_name: &str,
    headless_svc: &str,
    ns: &str,
    replicas: i32,
    cluster_port: u16,
) -> String {
    (0..replicas)
        .map(|i| format!("{cluster_name}-{i}.{headless_svc}.{ns}.svc.cluster.local:{cluster_port}"))
        .collect::<Vec<_>>()
        .join(",")
}
