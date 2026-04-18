use std::{sync::Arc, time::Duration};

use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    jiff::Timestamp,
};
use kube::{
    Client, Resource,
    api::{DeleteParams, Patch, PatchParams, PropagationPolicy},
    runtime::{
        controller::Action,
        events::{Event as KubeEvent, EventType, Recorder, Reporter},
    },
};
use tracing::*;

use crate::{
    context::{ClusterCtx, FIELD_MANAGER},
    crd::{DiomCluster, Phase},
    error::{Error, Result},
    resources::{pdb, pvcs, services, statefulset},
};

// Condition type names following Kubernetes naming conventions.
const COND_READY: &str = "Ready";
const COND_PROGRESSING: &str = "Progressing";
const COND_STORAGE_EXPANSION_FAILED: &str = "StorageExpansionFailed";

pub(crate) struct Context {
    pub client: Client,
}

pub(crate) async fn reconcile(cluster: Arc<DiomCluster>, ctx: Arc<Context>) -> Result<Action> {
    if cluster.meta().deletion_timestamp.is_some() {
        return Ok(Action::await_change());
    }

    let cctx = ClusterCtx::new(cluster.clone(), ctx.client.clone())?;
    let pp = cctx.pp();
    let generation = cluster.meta().generation.unwrap_or(0);

    // Snapshot current conditions and last_error to be mutated in-place as reconcile proceeds.
    let mut conditions: Vec<Condition> = cluster
        .status
        .as_ref()
        .map(|s| s.conditions.clone())
        .unwrap_or_default();
    let mut last_error: Option<String> = cluster.status.as_ref().and_then(|s| s.last_error.clone());
    // Tracks the phase explicitly set during this loop so update_status can use it as context
    // for compute_phase (e.g. treat Resizing like Initializing rather than Degraded).
    let mut phase_intent: Option<Phase> = None;

    info!(name = %cctx.name, ns = %cctx.ns, "Reconciling DiomCluster");

    // Headless service (inter-node DNS)
    let headless = services::build_headless(&cctx)?;
    cctx.svc_api()
        .patch(
            headless.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&headless),
        )
        .await?;

    // Client-facing service
    let client_svc = services::build_lb_svc(&cctx)?;
    cctx.svc_api()
        .patch(
            client_svc.metadata.name.as_deref().unwrap(),
            &pp,
            &Patch::Apply(&client_svc),
        )
        .await?;

    // StatefulSet — volumeClaimTemplates is an immutable field, so if storage sizes change
    // we must: (1) expand existing PVCs where the storage class allows it, then (2) orphan-delete
    // the StatefulSet so it can be recreated with the new template. Pods and PVCs survive.
    let desired_sts = statefulset::build(&cctx)?;
    let sts_api = cctx.sts_api();

    // Set to false only when all PVC expansions failed and the CR has been reverted: in that
    // case desired_sts still carries the new (failed) sizes, so applying it would be rejected
    // by Kubernetes (volumeClaimTemplates is immutable). The CR revert triggers a fresh
    // reconcile that will build the correct desired_sts from the reverted spec.
    let mut proceed_with_sts_recreation = true;

    if let Some(existing_sts) = sts_api.get_opt(&cctx.name).await? {
        if pvcs::volume_claim_templates_changed(&existing_sts, &desired_sts) {
            info!(
                name = %cctx.name,
                ns = %cctx.ns,
                "StatefulSet volumeClaimTemplates changed; orphan-deleting to allow recreation"
            );

            // Signal that a storage resize is in progress before doing any destructive work.
            phase_intent = Some(Phase::Resizing);
            upsert_condition(
                &mut conditions,
                COND_PROGRESSING,
                "True",
                "StorageResizing",
                "Expanding PVCs and recreating StatefulSet for storage size change",
                generation,
            );
            patch_status(&cctx, Phase::Resizing, &last_error, &conditions).await?;

            // Expand PVCs before deleting the StatefulSet. Any PVC whose storage class
            // does not support volume expansion is recorded as a failure.
            let (expansion_failures, expanded_templates) =
                pvcs::try_expand_pvcs(&existing_sts, &desired_sts, &cctx.client, &cctx.ns).await?;
            let any_expanded = !expanded_templates.is_empty();

            if !expansion_failures.is_empty() {
                // Set StorageExpansionFailed condition and last_error before handling individual
                // failures so they are visible even if a subsequent revert call errors out.
                let first = &expansion_failures[0];
                let error_msg = format!(
                    "Storage expansion failed for '{}': {}",
                    first.template_name, first.reason
                );
                last_error = Some(error_msg.clone());
                upsert_condition(
                    &mut conditions,
                    COND_STORAGE_EXPANSION_FAILED,
                    "True",
                    "ExpansionFailed",
                    error_msg,
                    generation,
                );

                let recorder = Recorder::new(cctx.client.clone(), Reporter::from(FIELD_MANAGER));
                let cluster_api = cctx.cluster_api();
                let mut seen_templates: std::collections::HashSet<String> =
                    std::collections::HashSet::new();

                for failure in &expansion_failures {
                    error!(
                        pvc = failure.pvc_name,
                        template = failure.template_name,
                        original_size = %failure.original_size.0,
                        reason = %failure.reason,
                        "PVC storage expansion failed; reverting CR spec to original size"
                    );

                    // Revert and emit an event once per template (multiple ordinals may fail).
                    if seen_templates.insert(failure.template_name.clone()) {
                        pvcs::revert_cluster_storage_size(
                            &cluster_api,
                            &cctx.name,
                            &failure.template_name,
                            &failure.original_size,
                        )
                        .await?;

                        if let Err(e) = recorder
                            .publish(
                                &KubeEvent {
                                    type_: EventType::Warning,
                                    reason: "PVCExpansionFailed".into(),
                                    note: Some(format!(
                                        "Failed to expand storage for volume '{}': {}. \
                                         CR spec size has been reverted to {}.",
                                        failure.template_name,
                                        failure.reason,
                                        failure.original_size.0
                                    )),
                                    action: "ExpandPVCStorage".into(),
                                    secondary: None,
                                },
                                &cluster.object_ref(&()),
                            )
                            .await
                        {
                            warn!(error = %e, "Failed to publish PVC expansion failure event");
                        }

                        // If some ordinals for this template succeeded while others failed,
                        // the PVC sizes across the StatefulSet are permanently inconsistent —
                        // the successful expansions cannot be undone. Emit a distinct event so
                        // operators know manual intervention may be required.
                        if expanded_templates.contains(&failure.template_name) {
                            error!(
                                template = failure.template_name,
                                "Volume template partially expanded: some replica PVCs were \
                                 resized and others were not. PVC sizes are now inconsistent \
                                 across the StatefulSet. Manual intervention may be required."
                            );
                            if let Err(e) = recorder
                                .publish(
                                    &KubeEvent {
                                        type_: EventType::Warning,
                                        reason: "PVCPartialExpansion".into(),
                                        note: Some(format!(
                                            "Volume '{}' was partially expanded: some replica \
                                             PVCs were resized and others were not. PVC sizes \
                                             are now inconsistent across the StatefulSet. \
                                             Manual intervention may be required.",
                                            failure.template_name
                                        )),
                                        action: "ExpandPVCStorage".into(),
                                        secondary: None,
                                    },
                                    &cluster.object_ref(&()),
                                )
                                .await
                            {
                                warn!(
                                    error = %e,
                                    "Failed to publish partial PVC expansion event"
                                );
                            }
                        }
                    }
                }
            } else {
                // All expansions succeeded — clear any previous failure state.
                last_error = None;
                upsert_condition(
                    &mut conditions,
                    COND_STORAGE_EXPANSION_FAILED,
                    "False",
                    "ExpansionSucceeded",
                    "",
                    generation,
                );
            }

            // Orphan-delete: removes the StatefulSet object but leaves pods and PVCs intact.
            // The immediately following SSA patch will recreate the StatefulSet and the
            // StatefulSet controller will adopt the existing pods via the label selector.
            //
            // Skip both the orphan-delete AND the SSA patch when every PVC expansion attempt
            // failed. desired_sts was built before the CR revert and still carries the new
            // (failed) template size. Applying it against the existing STS would be rejected
            // by Kubernetes (volumeClaimTemplates is immutable) and produce a spurious error.
            // The CR revert will trigger a fresh reconcile that builds the correct desired_sts
            // from the reverted spec.
            proceed_with_sts_recreation = any_expanded || expansion_failures.is_empty();
            if proceed_with_sts_recreation {
                sts_api
                    .delete(
                        &cctx.name,
                        &DeleteParams {
                            propagation_policy: Some(PropagationPolicy::Orphan),
                            ..Default::default()
                        },
                    )
                    .await?;
            } else {
                info!(
                    name = %cctx.name,
                    ns = %cctx.ns,
                    "Skipping StatefulSet orphan-delete and SSA patch: all PVC expansion \
                     attempts failed and CR spec has been reverted; fresh reconcile will follow"
                );
            }
        }
    }

    if proceed_with_sts_recreation {
        sts_api
            .patch(
                desired_sts.metadata.name.as_deref().unwrap(),
                &pp,
                &Patch::Apply(&desired_sts),
            )
            .await?;
    }

    // PodDisruptionBudget (only meaningful for replicas > 1)
    if cluster.spec.diom.replicas > 1 {
        let pdb = pdb::build(&cctx)?;
        cctx.pdb_api()
            .patch(
                pdb.metadata.name.as_deref().unwrap(),
                &pp,
                &Patch::Apply(&pdb),
            )
            .await?;
    }

    update_status(
        &cluster,
        &cctx,
        &mut conditions,
        last_error,
        generation,
        phase_intent.as_ref(),
    )
    .await?;

    info!(name = %cctx.name, ns = %cctx.ns, "Reconcile complete");
    Ok(Action::requeue(Duration::from_secs(300)))
}

/// Upserts a condition into the conditions vec, preserving `lastTransitionTime` when the
/// status value has not changed (as required by the Kubernetes API conventions).
fn upsert_condition(
    conditions: &mut Vec<Condition>,
    type_: &str,
    status: &str,
    reason: &str,
    message: impl Into<String>,
    generation: i64,
) {
    let message = message.into();
    let now = Time(Timestamp::now());
    if let Some(c) = conditions.iter_mut().find(|c| c.type_ == type_) {
        // Only advance lastTransitionTime when the status value actually changes.
        let transition_time = if c.status == status {
            c.last_transition_time.clone()
        } else {
            now
        };
        c.status = status.to_string();
        c.reason = reason.to_string();
        c.message = message;
        c.last_transition_time = transition_time;
        c.observed_generation = Some(generation);
    } else {
        conditions.push(Condition {
            type_: type_.to_string(),
            status: status.to_string(),
            reason: reason.to_string(),
            message,
            last_transition_time: now,
            observed_generation: Some(generation),
        });
    }
}

/// Patches only the phase, lastError, and conditions on the status subresource. Used
/// mid-reconcile to make in-progress state observable before expensive operations complete.
async fn patch_status(
    cctx: &ClusterCtx,
    phase: Phase,
    last_error: &Option<String>,
    conditions: &[Condition],
) -> Result<()> {
    let patch = serde_json::json!({
        "apiVersion": "diom.svix.com/v1alpha1",
        "kind": "DiomCluster",
        "status": {
            "phase": phase,
            "lastError": last_error,
            "conditions": conditions,
        }
    });
    cctx.cluster_api()
        .patch_status(&cctx.name, &PatchParams::default(), &Patch::Merge(patch))
        .await?;
    Ok(())
}

fn compute_phase(ready: i32, desired: i32, current_phase: &Phase) -> Phase {
    if ready == desired {
        Phase::Running
    } else if matches!(current_phase, Phase::Initializing | Phase::Resizing) {
        // Keep Initializing (not Degraded) when we just started or just resized —
        // replicas are expected to be temporarily unavailable.
        Phase::Initializing
    } else {
        Phase::Degraded
    }
}

/// Writes the final status at the end of every reconcile loop: updates readyReplicas, phase,
/// lastError, and the Ready/Progressing conditions. The `phase_hint` is the phase explicitly
/// set during this loop (e.g. Resizing), used so compute_phase has the right context.
async fn update_status(
    cluster: &DiomCluster,
    cctx: &ClusterCtx,
    conditions: &mut Vec<Condition>,
    last_error: Option<String>,
    generation: i64,
    phase_hint: Option<&Phase>,
) -> Result<()> {
    // Prefer the phase set explicitly during this loop over the stored status, so that
    // compute_phase treats a just-resized cluster as Initializing rather than Degraded.
    let effective_current_phase = phase_hint
        .or_else(|| cluster.status.as_ref().map(|s| &s.phase))
        .cloned()
        .unwrap_or_default();

    let (ready_replicas, phase) = match cctx.sts_api().get_opt(&cctx.name).await? {
        Some(sts) => {
            let ready = sts
                .status
                .as_ref()
                .and_then(|s| s.ready_replicas)
                .unwrap_or(0);
            let desired = cluster.spec.diom.replicas;
            (
                ready,
                compute_phase(ready, desired, &effective_current_phase),
            )
        }
        None => (0, Phase::Initializing),
    };

    let desired = cluster.spec.diom.replicas;
    let ready_msg = format!("{ready_replicas}/{desired} replicas ready");

    let (ready_status, ready_reason) = match &phase {
        Phase::Running => ("True", "AllReplicasReady"),
        Phase::Initializing => ("False", "WaitingForReplicas"),
        Phase::Resizing => ("False", "StorageResizeInProgress"),
        Phase::Degraded => ("False", "SomeReplicasNotReady"),
    };
    upsert_condition(
        conditions,
        COND_READY,
        ready_status,
        ready_reason,
        &ready_msg,
        generation,
    );

    // Resize (if any) is complete for this loop — clear the Progressing condition.
    upsert_condition(
        conditions,
        COND_PROGRESSING,
        "False",
        "ReconcileComplete",
        "Reconciliation loop completed",
        generation,
    );

    let status_patch = serde_json::json!({
        "apiVersion": "diom.svix.com/v1alpha1",
        "kind": "DiomCluster",
        "status": {
            "readyReplicas": ready_replicas,
            "phase": phase,
            "lastError": last_error,
            "conditions": conditions,
        }
    });

    cctx.cluster_api()
        .patch_status(&cctx.name, &cctx.status_pp(), &Patch::Apply(status_patch))
        .await?;

    Ok(())
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
        // Resizing is treated like Initializing — replicas coming back up after orphan-delete.
        assert_eq!(compute_phase(0, 3, &Phase::Resizing), Phase::Initializing);
        assert_eq!(compute_phase(3, 3, &Phase::Resizing), Phase::Running);
    }
}
