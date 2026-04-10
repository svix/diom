use k8s_openapi::{
    api::core::v1::{Service, ServicePort, ServiceSpec},
    apimachinery::pkg::util::intstr::IntOrString,
};
use kube::{Resource, api::Patch, core::ObjectMeta};

use crate::{context::ClusterCtx, error::Result, labels};

pub(crate) fn headless_name(cluster_name: &str) -> String {
    format!("{cluster_name}-headless")
}

pub(crate) fn client_name(cluster_name: &str) -> String {
    cluster_name.to_string()
}

pub(crate) async fn reconcile(ctx: &ClusterCtx) -> Result<()> {
    let svc_api = ctx.svc_api();
    let pp = ctx.pp();

    let headless = build_headless(ctx)?;
    svc_api
        .patch(
            headless.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&headless),
        )
        .await?;

    let client_svc = build_client_facing(ctx)?;
    svc_api
        .patch(
            client_svc.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&client_svc),
        )
        .await?;

    Ok(())
}

pub(crate) fn build_headless(ctx: &ClusterCtx) -> Result<Service> {
    let cluster = &ctx.cluster;
    let cluster_name = &ctx.name;
    let spec = &cluster.spec;
    let cluster_port = spec.diom.api_port + 10000;

    Ok(Service {
        metadata: ObjectMeta {
            name: Some(headless_name(cluster_name)),
            namespace: Some(ctx.ns.clone()),
            labels: Some(labels::general_labels(cluster_name)),
            owner_references: Some(vec![cluster.controller_owner_ref(&()).unwrap()]),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            cluster_ip: Some("None".into()), // headless
            selector: Some(labels::selector(cluster_name)),
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
pub(crate) fn build_client_facing(ctx: &ClusterCtx) -> Result<Service> {
    let cluster = &ctx.cluster;
    let cluster_name = &ctx.name;
    let spec = &cluster.spec;
    let svc_spec = &spec.service;

    let svc_type = svc_spec
        .r#type
        .clone()
        .unwrap_or_else(|| "ClusterIP".into());

    let mut metadata = ObjectMeta {
        name: Some(client_name(cluster_name)),
        namespace: Some(ctx.ns.clone()),
        labels: Some(labels::general_labels(cluster_name)),
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
            selector: Some(labels::selector(cluster_name)),
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
