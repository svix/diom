use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::{
    LogId, NodeId,
    handle::{BackgroundCommand, RaftState},
    operations::{RecordLogTimestampOperation, TickOperation},
    raft::TypeConfig,
};
use crate::cfg::Configuration;
use diom_error::CanFailExt;
use diom_operations::{
    BackgroundError, BackgroundResult, OperationWriter, workers::BackgroundWorker,
};
use openraft::error::{ClientWriteError, RaftError};
use tap::TapFallible;
use tokio::task::JoinSet;

trait CanBeForwardToLeader {
    fn is_forward_to_leader_err(&self) -> bool;
}

impl CanBeForwardToLeader for anyhow::Error {
    fn is_forward_to_leader_err(&self) -> bool {
        if let Some(raft_err) =
            self.downcast_ref::<RaftError<TypeConfig, ClientWriteError<TypeConfig>>>()
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

#[derive(Clone)]
struct RecordLogTimestamps {
    cfg: Configuration,
    handle: RaftState,
}

impl BackgroundWorker for RecordLogTimestamps {
    const NAME: &str = "record-log-timestamps";

    async fn run(self) -> BackgroundResult<()> {
        let mut ticker = tokio::time::interval(self.cfg.cluster.log_index_interval.into());
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

impl Tick {
    const CHECK_INTERVAL: Duration = Duration::from_millis(50);
    const THRESHOLD: jiff::SignedDuration = jiff::SignedDuration::from_millis(250);
}

impl BackgroundWorker for Tick {
    const NAME: &str = "tick";

    async fn run(self) -> BackgroundResult<()> {
        let mut ticker = tokio::time::interval(Self::CHECK_INTERVAL);
        loop {
            let delta = self.handle.time.offset();
            if delta > Self::THRESHOLD {
                tracing::trace!("recording a no-op event");
                let op = TickOperation {};
                self.handle.write_request(op).await?;
            }
            ticker.tick().await;
        }
    }
}

struct BackgroundJobRunner {
    jobs: JoinSet<BackgroundResult<()>>,
}

impl BackgroundJobRunner {
    fn new() -> Self {
        Self {
            jobs: JoinSet::new(),
        }
    }

    fn spawn_job<J: BackgroundWorker + 'static>(&mut self, job: J) {
        self.jobs
            .spawn(async move { job.run_while_handling_panics().await });
    }

    async fn spawn_all(&mut self, cfg: Configuration, handle: RaftState) {
        self.spawn_job(RecordLogTimestamps {
            cfg: cfg.clone(),
            handle: handle.clone(),
        });
        self.spawn_job(Tick {
            handle: handle.clone(),
        });
        self.spawn_job(diom_kv::LeaderWorker::new(
            handle.state_machine.kv_store().await,
            handle.time.clone(),
            cfg.background_cleanup_interval.into(),
            handle.clone(),
        ));
        self.spawn_job(diom_cache::LeaderWorker::new(
            handle.state_machine.cache_store().await,
            handle.time.clone(),
            cfg.background_cleanup_interval.into(),
            handle.clone(),
        ));
        self.spawn_job(diom_idempotency::LeaderWorker::new(
            handle.state_machine.idempotency_store().await,
            handle.time.clone(),
            cfg.background_cleanup_interval.into(),
            handle.clone(),
        ));
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
        let mut runner = BackgroundJobRunner::new();
        runner.spawn_all(cfg.clone(), handle.clone()).await;
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

async fn trigger_snapshot(
    handle: &RaftState,
    state: openraft::ServerState,
    purge_by: PurgeBy,
    committed: Option<LogId>,
) -> anyhow::Result<bool> {
    if committed.is_none() {
        tracing::warn!("refusing to snapshot without any committed logs");
        return Ok(false);
    }
    if state.is_learner() {
        tracing::warn!("refusing to snapshot a learner");
        return Ok(false);
    }

    tracing::debug!("triggering background snapshot");
    if let Err(err) = handle.raft.trigger().snapshot().await {
        tracing::error!(?err, "error triggering background snapshot; ignoring");
        return Ok(false);
    }

    let offset_to_purge = match purge_by {
        PurgeBy::Time(duration) => {
            #[allow(clippy::disallowed_methods)]
            let then = jiff::Timestamp::now() - duration;
            handle
                .state_machine
                .log_id_before_time(then)
                .await
                .tap_err(|err| {
                    tracing::warn!(?err, "unable to find index for timestamp; not purging logs")
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

    Ok(true)
}

pub(super) async fn run_background_jobs_on_all_nodes(
    cfg: Configuration,
    handle: RaftState,
    mut receiver: tokio::sync::mpsc::Receiver<BackgroundCommand>,
) -> anyhow::Result<()> {
    let mut last_snapshot_time = std::time::Instant::now();
    let mut last_snapshot_index = handle
        .raft
        .with_raft_state(|st| st.committed().copied())
        .await?;
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
            .with_raft_state(|st| (st.committed().copied(), st.server_state))
            .await?;

        let delta = match (committed, last_snapshot_index) {
            (Some(a), Some(b)) => Some(a.index - b.index),
            (Some(a), None) => Some(a.index),
            _ => None,
        };
        let (should_snapshot, purge_by, responder) = if let Some(threshold) =
            cfg.cluster.snapshot_after_time
            && last_snapshot_time.elapsed() > threshold
        {
            (true, PurgeBy::Time(threshold.into()), None)
        } else if let Some(threshold) = cfg.cluster.snapshot_after_writes
            && let Some(delta) = delta
            && delta > (threshold as u64)
        {
            let purge_by = if let Some(idx) = last_snapshot_index {
                PurgeBy::Index(idx.index)
            } else {
                PurgeBy::Nothing
            };
            (true, purge_by, None)
        } else if let Some(BackgroundCommand::Snapshot(tx)) = event {
            (true, PurgeBy::Nothing, Some(tx))
        } else {
            (false, PurgeBy::Nothing, None)
        };

        if should_snapshot {
            if committed == last_snapshot_index && responder.is_none() {
                tracing::trace!("skipping background snapshot because nothing has changed");
                continue;
            }
            last_snapshot_time = std::time::Instant::now();
            last_snapshot_index = committed;
            // this timestamp is just for debugging so that users can see
            // when their request was actually processed
            #[allow(clippy::disallowed_methods)]
            let last_snapshot_timestamp = jiff::Timestamp::now();
            let payload = if trigger_snapshot(&handle, state, purge_by, committed).await?
                && let Some(index) = last_snapshot_index
            {
                Some((last_snapshot_timestamp, index))
            } else {
                None
            };
            if let Some(tx) = responder {
                tx.send(payload)
                    .can_fail("error sending response to snapshot request");
            }
        }
    }
    tracing::info!("shutting down");
    Ok(())
}
