use k8s_openapi::{
    api::policy::v1::{PodDisruptionBudget, PodDisruptionBudgetSpec},
    apimachinery::pkg::{apis::meta::v1::LabelSelector, util::intstr::IntOrString},
};
use kube::{
    Resource,
    api::{DeleteParams, Patch},
    core::ObjectMeta,
};

use crate::{
    context::ClusterCtx,
    error::{Error, Result},
    labels,
};

pub(crate) async fn reconcile(ctx: &ClusterCtx) -> Result<()> {
    if ctx.cluster.spec.diom.replicas > 1 {
        let pdb = build(ctx)?;
        ctx.pdb_api()
            .patch(&ctx.name, &ctx.pp(), &Patch::Apply(&pdb))
            .await?;
    } else {
        match ctx
            .pdb_api()
            .delete(&ctx.name, &DeleteParams::default())
            .await
        {
            Ok(_) => {}
            Err(kube::Error::Api(e)) if e.code == 404 => {} // already absent
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

pub(crate) fn build(ctx: &ClusterCtx) -> Result<PodDisruptionBudget> {
    let cluster = &ctx.cluster;
    let cluster_name = &ctx.name;
    let replicas = cluster.spec.diom.replicas;

    // Require a strict majority to be available at all times, which maintains quorum.
    let min_available = (replicas / 2) + 1;

    Ok(PodDisruptionBudget {
        metadata: ObjectMeta {
            name: Some(cluster_name.clone()),
            namespace: Some(ctx.ns.clone()),
            labels: Some(labels::general_labels(cluster_name)),
            owner_references: Some(vec![
                cluster
                    .controller_owner_ref(&())
                    .ok_or(Error::MissingField("owner UID"))?,
            ]),
            ..Default::default()
        },
        spec: Some(PodDisruptionBudgetSpec {
            min_available: Some(IntOrString::Int(min_available)),
            selector: Some(LabelSelector {
                match_labels: Some(labels::selector(cluster_name)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
}
