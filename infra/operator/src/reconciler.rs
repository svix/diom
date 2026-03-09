use std::{sync::Arc, time::Duration};

use k8s_openapi::api::{apps::v1::StatefulSet, core::v1::Service, policy::v1::PodDisruptionBudget};
use kube::{
    Api, Client, Resource, ResourceExt,
    api::{Patch, PatchParams},
    runtime::controller::Action,
};
use tracing::*;

use crate::{
    crd::{CoyoteCluster, Phase},
    error::{Error, Result},
    resources,
};

const FIELD_MANAGER: &str = "coyote-operator";

pub(crate) struct Context {
    pub client: Client,
}

pub(crate) async fn reconcile(cluster: Arc<CoyoteCluster>, ctx: Arc<Context>) -> Result<Action> {
    let ns = cluster
        .namespace()
        .ok_or(Error::MissingField("namespace"))?;
    let name = cluster.name_any();
    let client = &ctx.client;
    let pp = PatchParams::apply(FIELD_MANAGER).force();

    // If the cluster is being deleted, nothing to do — owned resources are
    // garbage collected automatically via owner references.
    if cluster.meta().deletion_timestamp.is_some() {
        return Ok(Action::await_change());
    }

    info!(name, ns, "Reconciling CoyoteCluster");

    // Headless service (inter-node DNS)
    let svc_api: Api<Service> = Api::namespaced(client.clone(), &ns);
    let headless = resources::services::build_headless(&cluster, &ns)?;
    svc_api
        .patch(
            headless.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&headless),
        )
        .await?;

    // Client-facing service
    let client_svc = resources::services::build_client(&cluster, &ns)?;
    svc_api
        .patch(
            client_svc.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&client_svc),
        )
        .await?;

    // StatefulSet
    let sts_api: Api<StatefulSet> = Api::namespaced(client.clone(), &ns);
    let sts = resources::statefulset::build(&cluster, &ns)?;
    sts_api
        .patch(
            sts.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&sts),
        )
        .await?;

    // PodDisruptionBudget (only meaningful for nodes > 1)
    if cluster.spec.nodes > 1 {
        let pdb_api: Api<PodDisruptionBudget> = Api::namespaced(client.clone(), &ns);
        let pdb = resources::pdb::build(&cluster, &ns)?;
        pdb_api
            .patch(
                pdb.metadata.name.as_deref().unwrap(),
                &pp,
                &Patch::Apply(&pdb),
            )
            .await?;
    }

    update_status(&cluster, client, &ns).await?;

    info!(name, ns, "Reconcile complete");
    Ok(Action::requeue(Duration::from_secs(300)))
}

async fn update_status(cluster: &CoyoteCluster, client: &Client, ns: &str) -> Result<()> {
    let cluster_api: Api<CoyoteCluster> = Api::namespaced(client.clone(), ns);
    let name = cluster.name_any();

    let sts_api: Api<StatefulSet> = Api::namespaced(client.clone(), ns);
    let (ready_replicas, phase) = match sts_api.get_opt(&name).await? {
        Some(sts) => {
            let ready = sts
                .status
                .as_ref()
                .and_then(|s| s.ready_replicas)
                .unwrap_or(0);
            let desired = cluster.spec.nodes;
            let phase = if ready == desired {
                Phase::Running
            } else if ready == 0 {
                Phase::Initializing
            } else {
                Phase::Degraded
            };
            (ready, phase)
        }
        None => (0, Phase::Initializing),
    };

    let status_patch = serde_json::json!({
        "apiVersion": "coyote.svix.com/v1alpha1",
        "kind": "CoyoteCluster",
        "status": {
            "readyReplicas": ready_replicas,
            "phase": phase,
        }
    });

    cluster_api
        .patch_status(
            &name,
            &PatchParams::apply(FIELD_MANAGER),
            &Patch::Apply(status_patch),
        )
        .await?;

    Ok(())
}

pub(crate) fn error_policy(
    _cluster: Arc<CoyoteCluster>,
    err: &Error,
    _ctx: Arc<Context>,
) -> Action {
    warn!("Reconcile error: {err:?}");
    Action::requeue(Duration::from_secs(30))
}
