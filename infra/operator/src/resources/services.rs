use k8s_openapi::{
    api::core::v1::{Service, ServicePort, ServiceSpec},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::{Resource, ResourceExt, core::ObjectMeta};

use crate::{crd::DiomCluster, error::Result, labels};

/// Name of the headless service used for inter-node DNS.
pub(crate) fn headless_name(cluster_name: &str) -> String {
    format!("{cluster_name}-headless")
}

/// Name of the client-facing service.
pub(crate) fn client_name(cluster_name: &str) -> String {
    cluster_name.to_string()
}

/// Headless service — gives each pod a stable DNS name for peer discovery.
/// Pods are reachable as `<pod>.<headless-name>.<ns>.svc.cluster.local`.
pub(crate) fn build_headless(cluster: &DiomCluster, ns: &str) -> Result<Service> {
    let cluster_name = cluster.name_any();
    let spec = &cluster.spec;
    let cluster_port = spec.diom.api_port + 10000;

    Ok(Service {
        metadata: ObjectMeta {
            name: Some(headless_name(&cluster_name)),
            namespace: Some(ns.into()),
            labels: Some(labels::general_labels(&cluster_name)),
            owner_references: Some(vec![cluster.controller_owner_ref(&()).unwrap()]),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            cluster_ip: Some("None".into()), // headless
            selector: Some(labels::selector(&cluster_name)),
            publish_not_ready_addresses: Some(true), // needed for seed_nodes to work during startup
            ports: Some(vec![
                ServicePort {
                    name: Some("api".into()),
                    port: spec.diom.api_port as i32,
                    target_port: Some(IntOrString::Int(spec.diom.api_port as i32)),
                    ..Default::default()
                },
                ServicePort {
                    name: Some("cluster".into()),
                    port: cluster_port as i32,
                    target_port: Some(IntOrString::Int(cluster_port as i32)),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    })
}

/// Client-facing service — load-balances across all ready pods.
pub(crate) fn build_client(cluster: &DiomCluster, ns: &str) -> Result<Service> {
    let cluster_name = cluster.name_any();
    let spec = &cluster.spec;
    let svc_spec = &spec.service;

    let svc_type = svc_spec
        .r#type
        .clone()
        .unwrap_or_else(|| "ClusterIP".into());

    let mut metadata = ObjectMeta {
        name: Some(client_name(&cluster_name)),
        namespace: Some(ns.into()),
        labels: Some(labels::general_labels(&cluster_name)),
        owner_references: Some(vec![cluster.controller_owner_ref(&()).unwrap()]),
        ..Default::default()
    };

    if !svc_spec.annotations.is_empty() {
        metadata.annotations = Some(svc_spec.annotations.clone());
    }

    Ok(Service {
        metadata,
        spec: Some(ServiceSpec {
            type_: Some(svc_type),
            selector: Some(labels::selector(&cluster_name)),
            traffic_distribution: Some("PreferSameZone".into()),
            ports: Some(vec![ServicePort {
                name: Some("api".into()),
                port: spec.diom.api_port as i32,
                target_port: Some(IntOrString::Int(spec.diom.api_port as i32)),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    })
}
