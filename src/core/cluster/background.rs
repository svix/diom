use std::{
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
    time::Duration,
};

use super::{
    Node, NodeId,
    handle::{BackgroundCommand, RaftState},
    operations::{RecordLogTimestampOperation, TickOperation},
};
use crate::cfg::Configuration;
use coyote_operations::{BackgroundError, BackgroundResult, OperationWriter};
use futures_util::FutureExt;
use openraft::error::{ClientWriteError, RaftError};
use tap::TapFallible;
use tokio::task::JoinSet;

trait CanBeForwardToLeader {
    fn is_forward_to_leader_err(&self) -> bool;
}

impl CanBeForwardToLeader for anyhow::Error {
    fn is_forward_to_leader_err(&self) -> bool {
        if let Some(raft_err) =
            self.downcast_ref::<RaftError<NodeId, ClientWriteError<NodeId, Node>>>()
        {
            raft_err.forward_to_leader().is_some()
        } else {
            false
        }
    }
}

impl CanBeForwardToLeader for BackgroundError {
    fn is_forward_to_leader_err(&self) -> bool {
        matches!(self, Self::NotLeader)
    }
}

trait BackgroundJob: Send + Clone {
    fn run_on_leader(self) -> impl Future<Output = BackgroundResult<()>> + Send;

    fn name(&self) -> &'static str;
}

#[derive(Clone)]
struct RecordLogTimestamps {
    cfg: Configuration,
    handle: RaftState,
}

impl BackgroundJob for RecordLogTimestamps {
    fn name(&self) -> &'static str {
        "record-log-timestamps"
    }

    async fn run_on_leader(self) -> BackgroundResult<()> {
        let mut ticker = tokio::time::interval(self.cfg.cluster.log_index_interval);
        loop {
            tracing::trace!("recording log timestamps");
            let op = RecordLogTimestampOperation {};
            self.handle.write_request(op).await?;
            ticker.tick().await;
        }
    }
}

#[derive(Clone)]
struct Tick {
    handle: RaftState,
}

impl BackgroundJob for Tick {
    fn name(&self) -> &'static str {
        "tick"
    }

    async fn run_on_leader(self) -> BackgroundResult<()> {
        let mut ticker = tokio::time::interval(Duration::from_secs(10));
        loop {
            // TODO: only do this if there haven't been any other
            // writes recently
            tracing::trace!("recording a no-op event");
            let op = TickOperation {};
            self.handle.write_request(op).await?;
            ticker.tick().await;
        }
    }
}

#[derive(Clone)]
struct KvBackground {
    handle: RaftState,
}

impl BackgroundJob for KvBackground {
    fn name(&self) -> &'static str {
        "worker:kv"
    }

    async fn run_on_leader(self) -> BackgroundResult<()> {
        let store = self.handle.state_machine.kv_store().await;
        let time = self.handle.state_machine.time.clone();
        coyote_kv::leader_worker(store, self.handle, time).await
    }
}

#[derive(Clone)]
struct CacheBackground {
    handle: RaftState,
}

impl BackgroundJob for CacheBackground {
    fn name(&self) -> &'static str {
        "worker:cache"
    }

    async fn run_on_leader(self) -> BackgroundResult<()> {
        let store = self.handle.state_machine.cache_store().await;
        let time = self.handle.state_machine.time.clone();
        coyote_cache::worker(store, self.handle, time).await
    }
}

#[derive(Clone)]
struct IdempotencyBackground {
    handle: RaftState,
}

impl BackgroundJob for IdempotencyBackground {
    fn name(&self) -> &'static str {
        "worker:idempotency"
    }

    async fn run_on_leader(self) -> BackgroundResult<()> {
        let store = self.handle.state_machine.idempotency_store().await;
        let time = self.handle.state_machine.time.clone();
        coyote_idempotency::worker(store, self.handle, time).await
    }
}

struct BackgroundJobRunner {
    jobs: JoinSet<BackgroundResult<()>>,
}

const MAX_PANICS_PER_JOB: usize = 10;

fn spawn_job<J: BackgroundJob + 'static>(jobs: &mut JoinSet<BackgroundResult<()>>, job: J) {
    let shutting_down = coyote_core::shutdown::shutting_down_token();
    let mut backoff = coyote_core::backoff::ExponentialBackoffWithJitter::new(
        Duration::from_millis(10),
        Duration::from_secs(5),
    );
    let mut failures = 0;
    jobs.spawn(async move {
        loop {
            match AssertUnwindSafe(job.clone().run_on_leader())
                .catch_unwind()
                .await
            {
                Ok(v) => v?,
                Err(err) => {
                    failures += 1;
                    if failures > MAX_PANICS_PER_JOB {
                        tracing::error!(
                            ?err,
                            job_name = job.name(),
                            "a panic occurred during a background job; shutting down the server"
                        );
                        coyote_core::shutdown::start_shut_down();
                        shutting_down.cancelled().await;
                        return Ok(());
                    } else {
                        tracing::error!(
                            ?err,
                            job_name = job.name(),
                            "a panic occurred during a background job; restarting"
                        );
                    }
                }
            }
            shutting_down.run_until_cancelled(backoff.backoff()).await;
        }
    });
}

impl BackgroundJobRunner {
    fn spawn_all(cfg: Configuration, handle: RaftState) -> Self {
        let mut jobs = JoinSet::new();
        spawn_job(
            &mut jobs,
            RecordLogTimestamps {
                cfg,
                handle: handle.clone(),
            },
        );
        spawn_job(
            &mut jobs,
            KvBackground {
                handle: handle.clone(),
            },
        );
        spawn_job(
            &mut jobs,
            CacheBackground {
                handle: handle.clone(),
            },
        );
        spawn_job(
            &mut jobs,
            IdempotencyBackground {
                handle: handle.clone(),
            },
        );
        jobs.spawn(Tick { handle }.run_on_leader());
        Self { jobs }
    }

    async fn stop_all(mut self) -> anyhow::Result<()> {
        tracing::debug!("shutting down background jobs");
        self.jobs.abort_all();
        while let Some(job) = self.jobs.join_next().await {
            match job {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    if e.is_forward_to_leader_err() {
                        tracing::trace!("some worker died with forward-to-leader, who cares");
                    } else {
                        return Err(e.into());
                    }
                }
                Err(e) if e.is_cancelled() => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}

/// Generate a channel of all leadership changes in the raft cluster
async fn leadership_changes(handle: RaftState) -> tokio::sync::broadcast::Receiver<Option<NodeId>> {
    // this is racy (because it could change multiple times in between calls to `.wait`), so we
    // back it up by polling
    const POLL_INTERVAL: Duration = Duration::from_secs(10);

    let shutdown = crate::shutting_down_token();
    let (tx, rx) = tokio::sync::broadcast::channel(10);
    tokio::spawn(async move {
        let last_leader = Arc::new(Mutex::new(None));
        while shutdown
            .run_until_cancelled(handle.raft.wait(Some(POLL_INTERVAL)).metrics(
                |m| {
                    let mut l = last_leader.lock().unwrap();
                    if m.current_leader != *l {
                        tracing::debug!(old_leader=?l, new_leader=?m, "leader has changed");
                        *l = m.current_leader;
                        if tx.send(m.current_leader).is_err() {
                            return true;
                        }
                        true
                    } else {
                        false
                    }
                },
                "metrics to change",
            ))
            .await
            .is_some()
        {}
    });
    rx
}

/// Wait until the current node becomes the leader (or we shutdown)
async fn wait_until_leader(
    me: NodeId,
    mut chan: tokio::sync::broadcast::Receiver<Option<NodeId>>,
) -> bool {
    let shutdown = crate::shutting_down_token();
    loop {
        let Some(value) = shutdown.run_until_cancelled(chan.recv()).await else {
            return false;
        };
        match value {
            Ok(Some(node)) if node == me => {
                tracing::debug!("I believe I am the leader! Spawning background tasks");
                return true;
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                tracing::debug!("shutdown detected");
                return false;
            }
            _ => {}
        }
    }
}

pub(super) async fn run_background_jobs_on_leader(
    cfg: Configuration,
    handle: RaftState,
) -> anyhow::Result<()> {
    let shutdown = crate::shutting_down_token();
    let mut chan = leadership_changes(handle.clone()).await;
    while wait_until_leader(handle.node_id, chan.resubscribe()).await {
        let mut runner = BackgroundJobRunner::spawn_all(cfg.clone(), handle.clone());
        loop {
            tokio::select! {
                new_leader = chan.recv() => {
                    match new_leader {
                        Ok(new_leader) if new_leader == Some(handle.node_id) => {
                            // we might receive ourselves several times
                        },
                        Ok(new_leader) => {
                            tracing::debug!(?new_leader, "No longer the leader");
                            break;
                        },
                        Err(err) => {
                            tracing::warn!(?err, "leader detection died");
                            break;
                        }
                    }
                },
                _ = shutdown.cancelled() => {
                    tracing::debug!("shutting down");
                    break;
                },
                res = runner.jobs.join_next() => {
                    if let Some(res) = res {
                        tracing::debug!("a background job ended unexpectedly");
                        match res {
                            Ok(Ok(_)) => {},
                            Ok(Err(e)) => {
                                if e.is_forward_to_leader_err() {
                                    tracing::debug!("failed a write because we are not the leader");
                                    break;
                                } else {
                                    runner.stop_all().await?;
                                    return Err(e.into());
                                }
                            }
                            Err(e) => {
                                if !e.is_cancelled() {
                                    return Err(e.into());
                                }
                            }
                        }
                    }
                }
            }
        }
        runner.stop_all().await?;
    }
    Ok(())
}

enum PurgeBy {
    Time(Duration),
    Index(u64),
    Nothing,
}

pub(super) async fn run_background_jobs_on_all_nodes(
    cfg: Configuration,
    handle: RaftState,
    mut receiver: tokio::sync::mpsc::Receiver<BackgroundCommand>,
) -> anyhow::Result<()> {
    let mut last_snapshot_time = std::time::Instant::now();
    let mut last_snapshot_index = handle.raft.with_raft_state(|st| st.committed).await?;
    let mut ticker = tokio::time::interval(Duration::from_secs(60));
    let shutdown = crate::shutting_down_token();

    loop {
        let event = tokio::select! {
            event = receiver.recv() => {
                if event.is_some() {
                    event
                } else {
                    break;
                }
            },
            _ = ticker.tick() => None,
            _ = shutdown.cancelled() => break,
        };
        let (committed, state) = handle
            .raft
            .with_raft_state(|st| (st.committed, st.server_state))
            .await?;
        // even if the time interval has passed, if we haven't written anything it would be dumb to
        // snapshot
        if committed == last_snapshot_index {
            continue;
        }
        let delta = match (committed, last_snapshot_index) {
            (Some(a), Some(b)) => Some(a.index - b.index),
            (Some(a), None) => Some(a.index),
            _ => None,
        };
        let (should_snapshot, purge_by) = if let Some(threshold) = cfg.cluster.snapshot_after_time
            && last_snapshot_time.elapsed() > threshold
        {
            (true, PurgeBy::Time(threshold))
        } else if let Some(threshold) = cfg.cluster.snapshot_after_writes
            && let Some(delta) = delta
            && delta > (threshold as u64)
        {
            let purge_by = if let Some(idx) = last_snapshot_index {
                PurgeBy::Index(idx.index)
            } else {
                PurgeBy::Nothing
            };
            (true, purge_by)
        } else if event == Some(BackgroundCommand::Snapshot) {
            (true, PurgeBy::Nothing)
        } else {
            (false, PurgeBy::Nothing)
        };

        if should_snapshot {
            if state.is_learner() {
                tracing::warn!("refusing to snapshot a learner");
            } else {
                last_snapshot_time = std::time::Instant::now();
                last_snapshot_index = committed;
                tracing::debug!("triggering background snapshot");
                if let Err(err) = handle.raft.trigger().snapshot().await {
                    tracing::error!(?err, "error triggering background snapshot; ignoring");
                }
            }

            let offset_to_purge = match purge_by {
                PurgeBy::Time(duration) => {
                    let then = jiff::Timestamp::now() - duration;
                    handle
                        .state_machine
                        .log_id_before_time(then)
                        .await
                        .tap_err(|err| {
                            tracing::warn!(
                                ?err,
                                "unable to find index for timestamp; not purging logs"
                            )
                        })
                        .ok()
                        .flatten()
                }
                PurgeBy::Index(log_id) => Some(log_id),
                PurgeBy::Nothing => None,
            };

            if let Some(offset_to_purge) = offset_to_purge {
                tracing::debug!(offset_to_purge, "triggering purge of old logs");
                if let Err(err) = handle.raft.trigger().purge_log(offset_to_purge).await {
                    tracing::error!(?err, "failed to purge old logs");
                }
            }
        }
    }
    tracing::info!("shutting down");
    Ok(())
}
