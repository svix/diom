use std::{sync::Arc, time::Duration};

use kube::{Client, Resource, api::Patch, runtime::controller::Action};

use crate::{
    context::ClusterCtx,
    crd::{DiomCluster, Phase},
    error::{Error, Result},
    resources::{pdb, pvcs, services, statefulset},
};

pub(crate) struct Context {
    pub client: Client,
}

struct Reconciler {
    ctx: ClusterCtx,
}

impl Reconciler {
    fn new(cluster: Arc<DiomCluster>, client: Client) -> Result<Self> {
        Ok(Self {
            ctx: ClusterCtx::new(cluster, client)?,
        })
    }

    async fn run(self) -> Result<Action> {
        services::reconcile(&self.ctx).await?;
        statefulset::reconcile(&self.ctx).await?;
        pvcs::reconcile_pvcs(&self.ctx).await?;
        pdb::reconcile(&self.ctx).await?;

        self.update_status().await?;
        tracing::info!(name = %self.ctx.name, ns = %self.ctx.ns, "Reconcile complete");
        Ok(Action::requeue(Duration::from_secs(60)))
    }

    async fn update_status(&self) -> Result<()> {
        let name = &self.ctx.name;
        let current_status = self.ctx.cluster.status.as_ref();
        let current_phase = current_status.map(|s| s.phase.clone()).unwrap_or_default();
        let current_ready = current_status.map(|s| s.ready_replicas).unwrap_or(0);

        let (ready_replicas, phase) = match self.ctx.sts_api().get_opt(name).await? {
            Some(sts) => {
                let ready = sts
                    .status
                    .as_ref()
                    .and_then(|s| s.ready_replicas)
                    .unwrap_or(0);
                let desired = self.ctx.cluster.spec.diom.replicas;
                let phase = compute_phase(ready, desired, &current_phase);
                (ready, phase)
            }
            None => (0, Phase::Initializing),
        };

        if phase == current_phase && ready_replicas == current_ready {
            return Ok(());
        }

        let status_patch = serde_json::json!({
            "apiVersion": "diom.svix.com/v1alpha1",
            "kind": "DiomCluster",
            "status": {
                "readyReplicas": ready_replicas,
                "phase": phase,
            }
        });

        self.ctx
            .cluster_api()
            .patch_status(name, &self.ctx.status_pp(), &Patch::Apply(status_patch))
            .await?;

        Ok(())
    }
}

fn compute_phase(ready: i32, desired: i32, current_phase: &Phase) -> Phase {
    if ready == desired {
        Phase::Running
    } else if matches!(current_phase, Phase::Initializing) {
        Phase::Initializing
    } else {
        Phase::Degraded
    }
}

pub(crate) async fn reconcile(cluster: Arc<DiomCluster>, ctx: Arc<Context>) -> Result<Action> {
    if cluster.meta().deletion_timestamp.is_some() {
        return Ok(Action::await_change());
    }

    let r = Reconciler::new(cluster, ctx.client.clone())?;
    tracing::info!(name = %r.ctx.name, ns = %r.ctx.ns, "Reconciling DiomCluster");
    r.run().await
}

pub(crate) fn error_policy(_cluster: Arc<DiomCluster>, err: &Error, _ctx: Arc<Context>) -> Action {
    tracing::warn!("Reconcile error: {err:?}");
    Action::requeue(Duration::from_secs(30))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_phase() {
        assert_eq!(compute_phase(3, 3, &Phase::Initializing), Phase::Running);
        assert_eq!(compute_phase(3, 3, &Phase::Degraded), Phase::Running);
        assert_eq!(compute_phase(3, 3, &Phase::Running), Phase::Running);
        assert_eq!(
            compute_phase(0, 3, &Phase::Initializing),
            Phase::Initializing
        );
        assert_eq!(compute_phase(0, 3, &Phase::Running), Phase::Degraded);
        assert_eq!(compute_phase(0, 3, &Phase::Degraded), Phase::Degraded);
        assert_eq!(
            compute_phase(1, 3, &Phase::Initializing),
            Phase::Initializing
        );
        assert_eq!(
            compute_phase(2, 3, &Phase::Initializing),
            Phase::Initializing
        );
        assert_eq!(compute_phase(1, 3, &Phase::Running), Phase::Degraded);
    }
}
