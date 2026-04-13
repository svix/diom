use k8s_openapi::{
    api::policy::v1::{PodDisruptionBudget, PodDisruptionBudgetSpec},
    apimachinery::pkg::{apis::meta::v1::LabelSelector, util::intstr::IntOrString},
};
use kube::{Resource, ResourceExt, core::ObjectMeta};

use crate::{crd::DiomCluster, error::Result, labels};

pub(crate) fn build(cluster: &DiomCluster, ns: &str) -> Result<PodDisruptionBudget> {
    let cluster_name = cluster.name_any();
    let replicas = cluster.spec.diom.replicas;

    // Require a strict majority to be available at all times, which maintains quorum.
    let min_available = (replicas / 2) + 1;

    Ok(PodDisruptionBudget {
        metadata: ObjectMeta {
            name: Some(cluster_name.clone()),
            namespace: Some(ns.into()),
            labels: Some(labels::general_labels(&cluster_name)),
            owner_references: Some(vec![cluster.controller_owner_ref(&()).unwrap()]),
            ..Default::default()
        },
        spec: Some(PodDisruptionBudgetSpec {
            min_available: Some(IntOrString::Int(min_available)),
            selector: Some(LabelSelector {
                match_labels: Some(labels::selector(&cluster_name)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
