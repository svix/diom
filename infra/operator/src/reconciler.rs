#![allow(clippy::disallowed_methods)]

use std::{sync::Arc, time::Duration};

use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    jiff::{SignedDuration, Timestamp},
};
use kube::{Client, Resource, api::Patch, runtime::controller::Action};

use crate::{
    context::ClusterCtx,
    crd::{DiomCluster, DiomClusterStatus},
    error::{Error, Result},
    resources::{pdb, pvcs, services, statefulset},
};

const READY_CONDITION: &str = "Ready";
const RECONCILING_CONDITION: &str = "Reconciling";
const STALLED_CONDITION: &str = "Stalled";

// Duration at which a "Reconciling" error condition becomes "Stalled"
pub const STALL_THRESHOLD: Duration = Duration::from_secs(300);

#[derive(Clone, Copy, PartialEq, Eq)]
enum ConditionStatus {
    True,
    False,
    Unknown,
}

impl ConditionStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::True => "True",
            Self::False => "False",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ReadyState {
    Running,
    Initializing,
    Degraded { ready_replicas: i32, desired: i32 },
    Failed,
}

impl ReadyState {
    fn compute(
        ready_replicas: i32,
        desired: i32,
        previous_ready_replicas: i32,
        was_degraded: bool,
    ) -> Self {
        if ready_replicas == desired {
            Self::Running
        } else if previous_ready_replicas == desired || was_degraded {
            Self::Degraded {
                ready_replicas,
                desired,
            }
        } else {
            Self::Initializing
        }
    }

    fn status(&self) -> ConditionStatus {
        match self {
            Self::Running => ConditionStatus::True,
            Self::Initializing => ConditionStatus::Unknown,
            Self::Degraded { .. } | Self::Failed => ConditionStatus::False,
        }
    }

    fn reason(&self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Initializing => "Initializing",
            Self::Degraded { .. } => "Degraded",
            Self::Failed => "Failed",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Running => "All replicas ready".to_string(),
            Self::Initializing => "Waiting for replicas to become ready".to_string(),
            Self::Degraded {
                ready_replicas,
                desired,
            } => format!("{ready_replicas}/{desired} replicas ready"),
            Self::Failed => "Reconcile failing".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum FailureState {
    Reconciling(String),
    StalledInternalError(String),
    StalledInvalidSpec(String),
    StalledTimeout { err: String, threshold: Duration },
}

impl FailureState {
    fn compute(err: &Error, failing_for: SignedDuration, stall_threshold: Duration) -> Self {
        if matches!(err, Error::MissingField(_)) {
            return Self::StalledInternalError(err.to_string());
        }

        if matches!(err, Error::InvalidStorageSize(_)) {
            return Self::StalledInvalidSpec(err.to_string());
        }

        if failing_for.as_secs() >= stall_threshold.as_secs() as i64 {
            Self::StalledTimeout {
                err: err.to_string(),
                threshold: stall_threshold,
            }
        } else {
            Self::Reconciling(err.to_string())
        }
    }

    fn type_(&self) -> &'static str {
        match self {
            Self::Reconciling(_) => RECONCILING_CONDITION,
            Self::StalledInternalError(_)
            | Self::StalledInvalidSpec(_)
            | Self::StalledTimeout { .. } => STALLED_CONDITION,
        }
    }

    fn status(&self) -> ConditionStatus {
        ConditionStatus::True
    }

    fn reason(&self) -> &'static str {
        match self {
            Self::Reconciling(_) => "ReconcileError",
            Self::StalledInternalError(_) => "InternalError",
            Self::StalledInvalidSpec(_) => "InvalidSpec",
            Self::StalledTimeout { .. } => "Timeout",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Reconciling(err)
            | Self::StalledInternalError(err)
            | Self::StalledInvalidSpec(err) => err.clone(),
            Self::StalledTimeout { err, threshold } => format!(
                "Reconciliation has been failing for over {}s: {err}",
                threshold.as_secs()
            ),
        }
    }
}
pub(crate) struct Context {
    pub client: Client,
    pub requeue_interval: Duration,
    pub stall_threshold: Duration,
}

struct Reconciler {
    ctx: ClusterCtx,
    requeue_interval: Duration,
    stall_threshold: Duration,
}

impl Reconciler {
    fn new(
        cluster: Arc<DiomCluster>,
        client: Client,
        requeue_interval: Duration,
        stall_threshold: Duration,
    ) -> Result<Self> {
        Ok(Self {
            ctx: ClusterCtx::new(cluster, client)?,
            requeue_interval,
            stall_threshold,
        })
    }

    async fn run(&self) -> Result<Action> {
        services::reconcile(&self.ctx).await?;
        statefulset::reconcile(&self.ctx).await?;
        pvcs::reconcile(&self.ctx).await?;
        pdb::reconcile(&self.ctx).await?;

        self.update_status().await?;
        tracing::info!(name = %self.ctx.name, ns = %self.ctx.ns, "Reconcile complete");
        Ok(Action::requeue(self.requeue_interval))
    }

    async fn update_status(&self) -> Result<()> {
        let name = &self.ctx.name;
        let previous_status = self.ctx.cluster.status.clone().unwrap_or_default();
        let generation = self.ctx.cluster.meta().generation.unwrap_or(0);
        let desired = self.ctx.cluster.spec.diom.replicas;

        let ready_replicas = match self.ctx.sts_api().get_opt(name).await? {
            Some(sts) => sts
                .status
                .as_ref()
                .and_then(|s| s.ready_replicas)
                .unwrap_or(0),
            None => 0,
        };

        let prev_ready_condition = find_condition(&previous_status.conditions, READY_CONDITION);
        let was_degraded = prev_ready_condition.is_some_and(|c| c.reason == "Degraded");

        let state = ReadyState::compute(
            ready_replicas,
            desired,
            previous_status.ready_replicas,
            was_degraded,
        );

        let new_ready_condition = ready_condition(state, generation, prev_ready_condition);

        let ready_cond_changed = prev_ready_condition.is_none_or(|c| {
            c.status != new_ready_condition.status || c.reason != new_ready_condition.reason
        });

        if ready_cond_changed {
            tracing::info!(
                name = %self.ctx.name,
                ns = %self.ctx.ns,
                previous_reason = prev_ready_condition.map_or("None", |c| c.reason.as_str()),
                new_reason = %new_ready_condition.reason,
                message = %new_ready_condition.message,
                "Ready condition changed"
            );
        }

        let prior_error_conditions = previous_status
            .conditions
            .iter()
            .any(|c| c.type_ == RECONCILING_CONDITION || c.type_ == STALLED_CONDITION);

        if ready_replicas == previous_status.ready_replicas
            && generation == previous_status.observed_generation
            && !ready_cond_changed
            && !prior_error_conditions
        {
            return Ok(());
        }

        self.apply_status(DiomClusterStatus {
            ready_replicas,
            observed_generation: generation,
            conditions: vec![new_ready_condition],
        })
        .await
    }

    /// Mark the resource not-ready after a failed reconcile
    async fn mark_failed(&self, err: &Error) {
        let previous_status = self.ctx.cluster.status.clone().unwrap_or_default();
        let generation = self.ctx.cluster.meta().generation.unwrap_or(0);

        let previous_ready = find_condition(&previous_status.conditions, READY_CONDITION);
        let ready = ready_condition(ReadyState::Failed, generation, previous_ready);

        let failure_condition = compute_failure_condition(
            err,
            generation,
            &previous_status.conditions,
            Timestamp::now(),
            self.stall_threshold,
        );

        let previous_failure =
            find_condition(&previous_status.conditions, &failure_condition.type_);

        let unchanged = previous_status.observed_generation == generation
            && condition_unchanged(previous_ready, &ready)
            && condition_unchanged(previous_failure, &failure_condition);

        if unchanged {
            return;
        }

        let status = DiomClusterStatus {
            observed_generation: generation,
            conditions: vec![ready, failure_condition],
            ..previous_status
        };

        if let Err(patch_err) = self.apply_status(status).await {
            tracing::error!(name = %self.ctx.name, ns = %self.ctx.ns, "Failed to patch status after reconcile error: {patch_err:?}");
        }
    }

    async fn apply_status(&self, status: DiomClusterStatus) -> Result<()> {
        let status_patch = serde_json::json!({
            "apiVersion": "diom.svix.com/v1alpha1",
            "kind": "DiomCluster",
            "status": status,
        });

        self.ctx
            .cluster_api()
            .patch_status(
                &self.ctx.name,
                &self.ctx.status_pp(),
                &Patch::Apply(status_patch),
            )
            .await?;

        Ok(())
    }
}

pub(crate) async fn reconcile(cluster: Arc<DiomCluster>, ctx: Arc<Context>) -> Result<Action> {
    if cluster.meta().deletion_timestamp.is_some() {
        return Ok(Action::await_change());
    }

    let r = Reconciler::new(
        cluster,
        ctx.client.clone(),
        ctx.requeue_interval,
        ctx.stall_threshold,
    )?;
    tracing::info!(name = %r.ctx.name, ns = %r.ctx.ns, "Reconciling DiomCluster");
    match r.run().await {
        Ok(action) => Ok(action),
        Err(err) => {
            r.mark_failed(&err).await;
            Err(err)
        }
    }
}

pub(crate) fn error_policy(_cluster: Arc<DiomCluster>, err: &Error, ctx: Arc<Context>) -> Action {
    tracing::warn!("Reconcile error: {err:?}");
    Action::requeue(ctx.requeue_interval)
}

fn find_condition<'a>(conditions: &'a [Condition], type_: &str) -> Option<&'a Condition> {
    conditions.iter().find(|c| c.type_ == type_)
}

fn condition_unchanged(previous: Option<&Condition>, new: &Condition) -> bool {
    previous.is_some_and(|c| {
        c.status == new.status && c.reason == new.reason && c.message == new.message
    })
}

fn build_condition(
    type_: &str,
    status: ConditionStatus,
    reason: &str,
    message: String,
    observed_generation: i64,
    previous: Option<&Condition>,
) -> Condition {
    let last_transition_time = previous
        .filter(|c| c.status == status.as_str())
        .map(|c| c.last_transition_time.clone())
        .unwrap_or_else(|| Time(Timestamp::now()));

    Condition {
        type_: type_.to_string(),
        status: status.as_str().to_string(),
        reason: reason.to_string(),
        message,
        observed_generation: Some(observed_generation),
        last_transition_time,
    }
}

fn ready_condition(state: ReadyState, generation: i64, previous: Option<&Condition>) -> Condition {
    build_condition(
        READY_CONDITION,
        state.status(),
        state.reason(),
        state.message(),
        generation,
        previous,
    )
}

fn compute_failure_condition(
    err: &Error,
    generation: i64,
    previous_conditions: &[Condition],
    now: Timestamp,
    stall_threshold: Duration,
) -> Condition {
    let already_stalled = find_condition(previous_conditions, STALLED_CONDITION)
        .is_some_and(|c| c.status == ConditionStatus::True.as_str());

    let failing_for = if already_stalled {
        SignedDuration::from_secs(stall_threshold.as_secs() as i64)
    } else {
        let previous_reconciling = find_condition(previous_conditions, RECONCILING_CONDITION);
        let reconciling_started = previous_reconciling
            .filter(|c| c.status == ConditionStatus::True.as_str())
            .map_or(now, |c: &Condition| c.last_transition_time.0);
        now.duration_since(reconciling_started)
    };

    let state = FailureState::compute(err, failing_for, stall_threshold);
    let previous = find_condition(previous_conditions, state.type_());

    build_condition(
        state.type_(),
        state.status(),
        state.reason(),
        state.message(),
        generation,
        previous,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ready_state_compute() {
        assert_eq!(ReadyState::compute(3, 3, 0, false), ReadyState::Running);
        assert_eq!(ReadyState::compute(3, 3, 3, false), ReadyState::Running);
        assert_eq!(
            ReadyState::compute(0, 3, 0, false),
            ReadyState::Initializing
        );
        assert_eq!(
            ReadyState::compute(2, 3, 1, false),
            ReadyState::Initializing
        );
        assert_eq!(
            ReadyState::compute(0, 3, 3, false),
            ReadyState::Degraded {
                ready_replicas: 0,
                desired: 3
            }
        );
        assert_eq!(
            ReadyState::compute(1, 3, 3, false),
            ReadyState::Degraded {
                ready_replicas: 1,
                desired: 3
            }
        );
        assert_eq!(
            ReadyState::compute(1, 3, 1, true),
            ReadyState::Degraded {
                ready_replicas: 1,
                desired: 3
            }
        );
        assert_eq!(
            ReadyState::compute(1, 3, 1, false),
            ReadyState::Initializing
        );
    }

    fn fake_condition(type_: &str, status: &str, last_transition_time: Timestamp) -> Condition {
        Condition {
            type_: type_.to_string(),
            status: status.to_string(),
            reason: "PreviousReason".to_string(),
            message: "previous message".to_string(),
            observed_generation: Some(1),
            last_transition_time: Time(last_transition_time),
        }
    }

    #[test]
    fn test_compute_failure_condition_missing_field_stalls_immediately() {
        let now = Timestamp::now();
        let err = Error::MissingField("adminToken");

        let condition = compute_failure_condition(&err, 1, &[], now, STALL_THRESHOLD);

        assert_eq!(condition.type_, STALLED_CONDITION);
        assert_eq!(condition.status, "True");
    }

    #[test]
    fn test_compute_failure_condition_bad_storage_spec_stalls_immediately() {
        let now = Timestamp::now();
        let err = Error::InvalidStorageSize("invalid quantity".to_string());

        let condition = compute_failure_condition(&err, 1, &[], now, STALL_THRESHOLD);

        assert_eq!(condition.type_, STALLED_CONDITION);
        assert_eq!(condition.status, "True");
    }

    #[test]
    fn test_compute_failure_condition_first_failure_reconciles() {
        let now = Timestamp::now();
        let err = Error::Timeout("statefulset not ready".to_string());

        let condition = compute_failure_condition(&err, 1, &[], now, STALL_THRESHOLD);

        assert_eq!(condition.type_, RECONCILING_CONDITION);
        assert_eq!(condition.status, "True");
        assert!(
            condition
                .last_transition_time
                .0
                .duration_since(now)
                .as_secs()
                < 1
        );
    }

    #[test]
    fn test_compute_failure_condition_stays_reconciling_below_threshold() {
        let now = Timestamp::now();
        let started = now - (STALL_THRESHOLD - Duration::from_secs(1));
        let previous = [fake_condition(RECONCILING_CONDITION, "True", started)];
        let err = Error::Timeout("statefulset not ready".to_string());

        let condition = compute_failure_condition(&err, 1, &previous, now, STALL_THRESHOLD);

        assert_eq!(condition.type_, RECONCILING_CONDITION);
        assert_eq!(condition.last_transition_time.0, started);
    }

    #[test]
    fn test_compute_failure_condition_escalates_past_threshold() {
        let now = Timestamp::now();
        let started = now - (STALL_THRESHOLD + Duration::from_secs(1));
        let previous = [fake_condition(RECONCILING_CONDITION, "True", started)];
        let err = Error::Timeout("statefulset not ready".to_string());

        let condition = compute_failure_condition(&err, 1, &previous, now, STALL_THRESHOLD);

        assert_eq!(condition.type_, STALLED_CONDITION);
        assert_eq!(condition.status, "True");
    }

    #[test]
    fn test_compute_failure_condition_respects_custom_threshold() {
        let now = Timestamp::now();
        let short_threshold = Duration::from_secs(5);
        let started = now - Duration::from_secs(6);
        let previous = [fake_condition(RECONCILING_CONDITION, "True", started)];
        let err = Error::Timeout("statefulset not ready".to_string());

        let condition = compute_failure_condition(&err, 1, &previous, now, short_threshold);

        assert_eq!(condition.type_, STALLED_CONDITION);
    }

    #[test]
    fn test_compute_failure_condition_stays_stalled_across_ticks() {
        let now = Timestamp::now();
        let err = Error::Timeout("statefulset not ready".to_string());

        let started = now - (STALL_THRESHOLD + Duration::from_secs(1));
        let previous = [fake_condition(RECONCILING_CONDITION, "True", started)];
        let stalled = compute_failure_condition(&err, 1, &previous, now, STALL_THRESHOLD);
        assert_eq!(stalled.type_, STALLED_CONDITION);

        let previous = [stalled.clone()];
        let next_tick = now + Duration::from_secs(5);
        let condition = compute_failure_condition(&err, 1, &previous, next_tick, STALL_THRESHOLD);

        assert_eq!(condition.type_, STALLED_CONDITION);
        assert_eq!(condition.status, "True");
        assert_eq!(
            condition.last_transition_time.0,
            stalled.last_transition_time.0
        );
    }

    #[test]
    fn test_failure_state_decide() {
        let zero = SignedDuration::ZERO;
        let threshold = STALL_THRESHOLD;

        assert!(matches!(
            FailureState::compute(&Error::MissingField("adminToken"), zero, threshold),
            FailureState::StalledInternalError(_)
        ));
        assert!(matches!(
            FailureState::compute(
                &Error::InvalidStorageSize("bad".to_string()),
                zero,
                threshold
            ),
            FailureState::StalledInvalidSpec(_)
        ));

        let other = Error::PvcStorageState("unexpected".to_string());
        assert!(matches!(
            FailureState::compute(&other, zero, threshold),
            FailureState::Reconciling(_)
        ));
        assert!(matches!(
            FailureState::compute(
                &other,
                SignedDuration::from_secs(threshold.as_secs() as i64 - 1),
                threshold
            ),
            FailureState::Reconciling(_)
        ));
        assert!(matches!(
            FailureState::compute(
                &other,
                SignedDuration::from_secs(threshold.as_secs() as i64),
                threshold
            ),
            FailureState::StalledTimeout { .. }
        ));
    }
}
