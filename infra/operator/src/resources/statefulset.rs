use std::{collections::BTreeMap, time::Duration};

use k8s_openapi::{
    api::{
        apps::v1::{StatefulSet, StatefulSetPersistentVolumeClaimRetentionPolicy, StatefulSetSpec},
        core::v1::{
            Container, ContainerPort, EnvVar, EnvVarSource, HTTPGetAction, LocalObjectReference,
            ObjectFieldSelector, PersistentVolumeClaim, PersistentVolumeClaimSpec,
            PodSecurityContext, PodSpec, PodTemplateSpec, Probe, VolumeMount,
            VolumeResourceRequirements,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity, apis::meta::v1::LabelSelector, util::intstr::IntOrString,
    },
};
use kube::{
    Resource,
    api::{DeleteParams, Patch, PostParams, PropagationPolicy},
    core::ObjectMeta,
};

use crate::{
    context::ClusterCtx,
    crd::{DiomClusterSpec, INTRACLUSTER_PORT},
    error::{Error, Result},
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

pub(crate) async fn reconcile(ctx: &ClusterCtx) -> Result<()> {
    let new_sts = build(ctx)?;
    let sts_api = ctx.sts_api();

    if let Some(current) = sts_api.get_opt(&ctx.name).await?
        && volume_claim_templates_differ(&current, &new_sts)
    {
        tracing::info!(
            name = %ctx.name,
            "volumeClaimTemplates changed. Orphaning/Deleting the StatefulSet."
        );
        sts_api
            .delete(
                &ctx.name,
                &DeleteParams {
                    propagation_policy: Some(PropagationPolicy::Orphan),
                    ..Default::default()
                },
            )
            .await?;
        wait_for_sts_deleted(&sts_api, &ctx.name).await?;
        sts_api.create(&PostParams::default(), &new_sts).await?;
    } else {
        sts_api
            .patch(&ctx.name, &ctx.pp(), &Patch::Apply(new_sts))
            .await?;
    }

    Ok(())
}

fn build(ctx: &ClusterCtx) -> Result<StatefulSet> {
    let cluster_name = &ctx.name;
    let spec = &ctx.cluster.spec;
    let headless_svc = services::headless_svc_name(cluster_name);

    let env = build_env(spec, cluster_name, &headless_svc, &ctx.ns);
    let volume_claim_templates = build_volume_claim_templates(spec);
    let volume_mounts = build_volume_mounts(spec);
    let container = build_container(spec, env, volume_mounts);

    let pod_labels = labels::general_labels(cluster_name);
    let pod_annotations = spec.pod_annotations.clone();

    let pod_spec = PodSpec {
        containers: vec![container],
        volumes: None,
        // appuser in the diom-server image is created with plain `useradd`, giving it UID/GID
        // 1000. fsGroup ensures mounted PVC directories are chowned to that group on attach.
        security_context: Some(PodSecurityContext {
            fs_group: Some(1000),
            ..Default::default()
        }),
        topology_spread_constraints: Some(spec.topology_spread_constraints.clone()),
        node_selector: spec.node_selector.clone(),
        tolerations: spec.tolerations.clone(),
        affinity: spec.affinity.clone(),
        image_pull_secrets: spec.image_pull_secrets.as_ref().map(|secrets| {
            secrets
                .iter()
                .map(|s| LocalObjectReference {
                    name: s.name.clone(),
                })
                .collect()
        }),
        ..Default::default()
    };

    Ok(StatefulSet {
        metadata: ObjectMeta {
            name: Some(cluster_name.clone()),
            namespace: Some(ctx.ns.clone()),
            labels: Some(labels::general_labels(cluster_name)),
            owner_references: Some(vec![
                ctx.cluster
                    .controller_owner_ref(&())
                    .ok_or(Error::MissingField("owner UID"))?,
            ]),
            ..Default::default()
        },
        spec: Some(StatefulSetSpec {
            replicas: Some(spec.diom.replicas),
            service_name: Some(headless_svc),
            selector: LabelSelector {
                match_labels: Some(labels::selector(cluster_name)),
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
            persistent_volume_claim_retention_policy: Some(
                StatefulSetPersistentVolumeClaimRetentionPolicy {
                    when_deleted: Some("Retain".into()),
                    when_scaled: Some("Retain".into()),
                },
            ),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn build_env(
    spec: &DiomClusterSpec,
    cluster_name: &str,
    headless_svc: &str,
    ns: &str,
) -> Vec<EnvVar> {
    let intracluster_port = INTRACLUSTER_PORT;

    // These must come before any vars that reference them via $(VAR) substitution.
    let mut env: Vec<EnvVar> = vec![
        env_from_field("POD_NAME", "metadata.name"),
        env_from_field("POD_NAMESPACE", "metadata.namespace"),
        env_var(
            "DIOM_CLUSTER_ADVERTISED_ADDRESS",
            format!(
                "$(POD_NAME).{headless_svc}.$(POD_NAMESPACE).svc.cluster.local:{intracluster_port}"
            ),
        ),
        env_var(
            "DIOM_CLUSTER_SEED_NODES",
            seed_nodes_value(
                cluster_name,
                headless_svc,
                ns,
                spec.diom.replicas,
                intracluster_port,
            ),
        ),
        env_var("DIOM_CLUSTER_AUTO_INITIALIZE", "true"),
        env_var(
            "DIOM_LISTEN_ADDRESS",
            format!("0.0.0.0:{}", spec.diom.api_port),
        ),
        env_var(
            "DIOM_SERVER_URL",
            format!("http://localhost:{}", spec.diom.api_port),
        ),
        env_var("DIOM_PERSISTENT_DB_PATH", PERSISTENT_DATA_PATH),
    ];

    // Always set log and snapshot paths explicitly to avoid the Dockerfile defaults, which point
    // to ephemeral storage and cause crashes on pod restart if Raft references a missing snapshot.
    env.push(env_var(
        "DIOM_CLUSTER_LOG_PATH",
        if spec.diom.storage.logs.is_some() {
            LOGS_DATA_PATH.into()
        } else {
            format!("{PERSISTENT_DATA_PATH}/logs")
        },
    ));
    env.push(env_var(
        "DIOM_CLUSTER_SNAPSHOT_PATH",
        if spec.diom.storage.snapshots.is_some() {
            SNAPSHOTS_DATA_PATH.into()
        } else {
            format!("{PERSISTENT_DATA_PATH}/snapshots")
        },
    ));

    if let Some(level) = &spec.diom.log_level {
        env.push(env_var("DIOM_LOG_LEVEL", level));
    }

    if let Some(format) = &spec.diom.log_format {
        env.push(env_var("DIOM_LOG_FORMAT", format));
    }

    if let Some(opentelemetry) = &spec.diom.opentelemetry {
        if let Some(addr) = &opentelemetry.address {
            env.push(env_var("DIOM_OPENTELEMETRY_ADDRESS", addr));
        }

        if let Some(addr) = &opentelemetry.metrics_address {
            env.push(env_var("DIOM_OPENTELEMETRY_METRICS_ADDRESS", addr));
        }

        if let Some(proto) = &opentelemetry.metrics_protocol {
            env.push(env_var(
                "DIOM_OPENTELEMETRY_METRICS_PROTOCOL",
                proto.to_string(),
            ));
        }
    }

    if let Some(bootstrap) = &spec.diom.bootstrap {
        env.push(env_var("DIOM_BOOTSTRAP_CFG", bootstrap));
    }

    // Extra user-provided env vars.
    env.extend(spec.diom.env_var.iter().cloned());

    // The rest will shadow any manually applied env vars with
    // the same name.

    if let Some(admin_token) = &spec.diom.admin_token {
        env.push(admin_token.to_env_var("DIOM_ADMIN_TOKEN"));
    }

    if let Some(internode_secret) = &spec.diom.internode_secret {
        env.push(internode_secret.to_env_var("DIOM_CLUSTER_SECRET"));
    }

    env
}

fn build_container(
    spec: &DiomClusterSpec,
    env: Vec<EnvVar>,
    volume_mounts: Vec<VolumeMount>,
) -> Container {
    let intracluster_port = INTRACLUSTER_PORT;
    const API_HEALTH_ENDPOINT: &str = "/api/v1.health.ping";
    const CLUSTER_HEALTH_ENDPOINT: &str = "/repl/health";

    Container {
        name: "diom".into(),
        image: Some(spec.image.clone()),
        image_pull_policy: spec.image_pull_policy.clone(),
        command: Some(vec!["/usr/local/bin/diom-server".into()]),
        env: Some(env),
        ports: Some(vec![
            ContainerPort {
                name: Some("api".into()),
                container_port: spec.diom.api_port as i32,
                ..Default::default()
            },
            ContainerPort {
                name: Some("cluster".into()),
                container_port: intracluster_port as i32,
                ..Default::default()
            },
        ]),
        volume_mounts: Some(volume_mounts),
        resources: Some(spec.resources.clone()),
        liveness_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some(API_HEALTH_ENDPOINT.into()),
                port: IntOrString::Int(spec.diom.api_port as _),
                ..Default::default()
            }),
            initial_delay_seconds: Some(5),
            period_seconds: Some(10),
            failure_threshold: Some(2),
            success_threshold: Some(1),
            ..Default::default()
        }),
        readiness_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some(CLUSTER_HEALTH_ENDPOINT.into()),
                port: IntOrString::Int(intracluster_port as _),
                ..Default::default()
            }),
            initial_delay_seconds: Some(15),
            period_seconds: Some(10),
            failure_threshold: Some(2),
            success_threshold: Some(1),
            ..Default::default()
        }),
        startup_probe: Some(Probe {
            http_get: Some(HTTPGetAction {
                path: Some(CLUSTER_HEALTH_ENDPOINT.into()),
                port: IntOrString::Int(intracluster_port as _),
                ..Default::default()
            }),
            initial_delay_seconds: Some(15),
            period_seconds: Some(10),
            failure_threshold: Some(120), // TODO: this should come from the helm chart
            success_threshold: Some(1),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn build_volume_claim_templates(spec: &DiomClusterSpec) -> Vec<PersistentVolumeClaim> {
    let mut templates = vec![pvc_template(
        "persistent",
        &spec.diom.storage.persistent.size,
        spec.diom.storage.persistent.storage_class.as_deref(),
    )];

    if let Some(logs) = &spec.diom.storage.logs {
        templates.push(pvc_template(
            "logs",
            &logs.size,
            logs.storage_class.as_deref(),
        ));
    }

    if let Some(snapshots) = &spec.diom.storage.snapshots {
        templates.push(pvc_template(
            "snapshots",
            &snapshots.size,
            snapshots.storage_class.as_deref(),
        ));
    }

    templates
}

fn build_volume_mounts(spec: &DiomClusterSpec) -> Vec<VolumeMount> {
    let mut mounts = vec![VolumeMount {
        name: "persistent".into(),
        mount_path: PERSISTENT_DATA_PATH.into(),
        ..Default::default()
    }];

    if spec.diom.storage.logs.is_some() {
        mounts.push(VolumeMount {
            name: "logs".into(),
            mount_path: LOGS_DATA_PATH.into(),
            ..Default::default()
        });
    }

    if spec.diom.storage.snapshots.is_some() {
        mounts.push(VolumeMount {
            name: "snapshots".into(),
            mount_path: SNAPSHOTS_DATA_PATH.into(),
            ..Default::default()
        });
    }

    mounts
}

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

fn pvc_template(name: &str, size: &Quantity, storage_class: Option<&str>) -> PersistentVolumeClaim {
    let mut resources: BTreeMap<String, Quantity> = BTreeMap::new();
    resources.insert("storage".into(), size.clone());

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

fn volume_claim_templates_differ(current: &StatefulSet, desired: &StatefulSet) -> bool {
    let current_templates = current
        .spec
        .as_ref()
        .and_then(|s| s.volume_claim_templates.as_deref())
        .unwrap_or(&[]);
    let desired_templates = desired
        .spec
        .as_ref()
        .and_then(|s| s.volume_claim_templates.as_deref())
        .unwrap_or(&[]);

    for desired_tpl in desired_templates {
        let desired_name = desired_tpl.metadata.name.as_deref().unwrap_or("");
        let desired_size = desired_tpl
            .spec
            .as_ref()
            .and_then(|s| s.resources.as_ref())
            .and_then(|r| r.requests.as_ref())
            .and_then(|r| r.get("storage"));

        let current_size = current_templates
            .iter()
            .find(|t| t.metadata.name.as_deref() == Some(desired_name))
            .and_then(|t| t.spec.as_ref())
            .and_then(|s| s.resources.as_ref())
            .and_then(|r| r.requests.as_ref())
            .and_then(|r| r.get("storage"));

        if desired_size != current_size {
            return true;
        }
    }
    false
}

async fn wait_for_sts_deleted(sts_api: &kube::Api<StatefulSet>, name: &str) -> Result<()> {
    const TIMEOUT_SECS: u64 = 30;
    const POLL_INTERVAL: Duration = Duration::from_secs(1);

    let deadline = tokio::time::Instant::now() + Duration::from_secs(TIMEOUT_SECS);

    loop {
        if sts_api.get_opt(name).await?.is_none() {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(Error::Timeout(format!(
                "StatefulSet {name} not deleted after {TIMEOUT_SECS}s"
            )));
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crd::{
        DiomSpec, DiomStorageSpec, OpenTelemetryProtocol, OpenTelemetrySpec, VolumeSpec,
    };
    use k8s_openapi::{
        api::core::v1::{
            PersistentVolumeClaim, PersistentVolumeClaimSpec, VolumeResourceRequirements,
        },
        apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::ObjectMeta},
    };
    use std::collections::BTreeMap;

    fn make_spec() -> DiomClusterSpec {
        DiomClusterSpec {
            diom: DiomSpec {
                replicas: 1,
                api_port: crate::crd::DEFAULT_API_PORT,
                storage: DiomStorageSpec {
                    persistent: VolumeSpec {
                        size: Quantity("1Gi".to_string()),
                        storage_class: None,
                    },
                    logs: None,
                    snapshots: None,
                },
                opentelemetry: None,
                internode_secret: None,
                log_level: None,
                log_format: None,
                bootstrap: None,
                admin_token: None,
                env_var: vec![],
            },
            image: "test-image".to_string(),
            image_pull_policy: None,
            service: Default::default(),
            resources: Default::default(),
            pod_annotations: Default::default(),
            topology_spread_constraints: vec![],
            node_selector: None,
            tolerations: None,
            affinity: None,
            image_pull_secrets: None,
        }
    }

    #[test]
    fn test_build_env() {
        let mut spec = make_spec();
        spec.diom.opentelemetry = Some(OpenTelemetrySpec {
            address: Some("grpc://otel:4317".to_string()),
            metrics_address: Some("http://otel:4318".to_string()),
            metrics_protocol: Some(OpenTelemetryProtocol::Http),
        });

        let env_vars = build_env(&spec, "my-cluster", "my-cluster-headless", "my-ns");
        // This var has k8s placeholders that can't be parsed by Diom config, so just assert it directly
        // and exclude from the temp env:
        assert_eq!(
            env_vars
                .iter()
                .find(|e| e.name == "DIOM_CLUSTER_ADVERTISED_ADDRESS")
                .and_then(|e| e.value.as_deref()),
            Some("$(POD_NAME).my-cluster-headless.$(POD_NAMESPACE).svc.cluster.local:8625"),
        );

        let env_vars: Vec<(String, Option<String>)> = env_vars
            .into_iter()
            .filter(|e| e.name.starts_with("DIOM_"))
            .filter(|e| e.name != "DIOM_CLUSTER_ADVERTISED_ADDRESS")
            .map(|e| (e.name, e.value))
            .collect();

        // Backend config should successfully load given our Operator-supplied env vars
        let cfg = temp_env::with_vars(env_vars, || diom_backend::cfg::load(None).unwrap());

        assert_eq!(
            cfg.listen_address,
            "0.0.0.0:8624".parse::<std::net::SocketAddr>().unwrap()
        );
        assert_eq!(
            cfg.persistent_db.path,
            std::path::PathBuf::from("/data/persistent")
        );
        assert!(cfg.cluster.auto_initialize);
        assert_eq!(
            cfg.cluster.seed_nodes,
            vec![
                "my-cluster-0.my-cluster-headless.my-ns.svc.cluster.local:8625"
                    .parse::<diom_backend::cfg::PeerAddr>()
                    .unwrap()
            ]
        );
        assert_eq!(
            cfg.cluster.log_path,
            Some(std::path::PathBuf::from("/data/persistent/logs"))
        );
        assert_eq!(
            cfg.cluster.snapshot_path,
            Some(std::path::PathBuf::from("/data/persistent/snapshots"))
        );
        assert_eq!(
            cfg.opentelemetry.address.as_deref(),
            Some("grpc://otel:4317")
        );
        assert_eq!(
            cfg.opentelemetry.metrics_address.as_deref(),
            Some("http://otel:4318")
        );
        assert!(matches!(
            cfg.opentelemetry.metrics_protocol,
            diom_backend::cfg::OpenTelemetryProtocol::Http
        ));
    }

    fn make_pvc(name: &str, storage: &str) -> PersistentVolumeClaim {
        let mut requests = BTreeMap::new();
        requests.insert("storage".to_string(), Quantity(storage.to_string()));
        PersistentVolumeClaim {
            metadata: ObjectMeta {
                name: Some(name.to_string()),
                ..Default::default()
            },
            spec: Some(PersistentVolumeClaimSpec {
                resources: Some(VolumeResourceRequirements {
                    requests: Some(requests),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn make_sts(pvcs: Vec<PersistentVolumeClaim>) -> StatefulSet {
        use k8s_openapi::api::apps::v1::StatefulSetSpec;
        StatefulSet {
            spec: Some(StatefulSetSpec {
                volume_claim_templates: Some(pvcs),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_volume_claim_templates_differ() {
        let a = make_sts(vec![make_pvc("data", "10Gi")]);
        let b = make_sts(vec![make_pvc("data", "10Gi")]);
        assert!(!volume_claim_templates_differ(&a, &b));

        let current = make_sts(vec![make_pvc("data", "10Gi")]);
        let desired = make_sts(vec![make_pvc("data", "20Gi")]);
        assert!(volume_claim_templates_differ(&current, &desired));

        let current = make_sts(vec![make_pvc("data", "20Gi")]);
        let desired = make_sts(vec![make_pvc("data", "10Gi")]);
        assert!(volume_claim_templates_differ(&current, &desired));

        let current = make_sts(vec![make_pvc("data", "10Gi")]);
        let desired = make_sts(vec![make_pvc("data", "10Gi"), make_pvc("logs", "5Gi")]);
        assert!(volume_claim_templates_differ(&current, &desired));

        let current = make_sts(vec![make_pvc("data", "10Gi"), make_pvc("logs", "5Gi")]);
        let desired = make_sts(vec![make_pvc("data", "10Gi")]);
        assert!(!volume_claim_templates_differ(&current, &desired));

        let empty = make_sts(vec![]);
        assert!(!volume_claim_templates_differ(&empty, &empty));
    }
}
