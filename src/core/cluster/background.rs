use std::time::Duration;

use super::{
    LogId,
    handle::{BackgroundCommand, RaftState},
    operations::{RecordLogTimestampOperation, TickOperation},
    raft::TypeConfig,
};
use crate::cfg::Configuration;
use diom_error::CanFailExt;
use diom_operations::{
    BackgroundError, BackgroundResult, OperationWriter, workers::BackgroundWorker,
};
use futures_util::TryFutureExt;
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
    spawned: bool,
}

impl BackgroundJobRunner {
    fn new() -> Self {
        Self {
            jobs: JoinSet::new(),
            spawned: false,
        }
    }

    fn spawn_job<J: BackgroundWorker + 'static>(&mut self, job: J) {
        self.jobs
            .spawn(async move { job.run_while_handling_panics().await });
    }

    async fn spawn_all(&mut self, cfg: Configuration, handle: RaftState) {
        if self.spawned {
            return;
        }
        tracing::debug!("starting leader-only background jobs");
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
        self.spawn_job(diom_msgs::svix_poller::LeaderWorker::<
            diom_core::svix_client::RealSvixAutoConfigClient,
            _,
        >::new(
            handle.state_machine.msgs_store().await,
            cfg.background_cleanup_interval.into(),
            handle.clone(),
            diom_msgs::svix_poller::SvixPollerConfig {
                max_concurrent_pollers: cfg.svix_poller_max_concurrency,
                max_task_duration: cfg.svix_poller_max_task_duration.into(),
            },
        ));
        self.spawn_job(diom_msgs::sink::LeaderWorker::new(
            handle.state_machine.msgs_store().await,
            cfg.background_cleanup_interval.into(),
            handle.clone(),
            diom_msgs::sink::SinkWorkerConfig {
                max_concurrent: cfg.sink_max_concurrency,
                max_task_duration: cfg.sink_max_task_duration.into(),
            },
        ));
        tracing::trace!("leader-only background jobs started");
        self.spawned = true;
    }

    async fn stop_all(&mut self) -> anyhow::Result<()> {
        if !self.spawned {
            return Ok(());
        }
        tracing::debug!("shutting down leader-only background jobs");
        self.jobs.abort_all();
        self.spawned = false;
        while let Some(job) = self.jobs.join_next().await {
            match job {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    if e.is_forward_to_leader_err() {
                        tracing::trace!("some worker died with forward-to-leader, who cares");
                    } else {
                        tracing::trace!(error=?e, "leader-only background job had an error");
                        return Err(e.into());
                    }
                }
                Err(e) if e.is_cancelled() => {}
                Err(e) => return Err(e.into()),
            }
        }
        tracing::trace!("leader-only background jobs stopped");
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum BackgroundJobLeaderMessage {
    StartBeingLeader,
    StopBeingLeader,
}

pub(super) async fn run_background_jobs_on_leader(
    cfg: Configuration,
    handle: RaftState,
) -> anyhow::Result<()> {
    let mut runner = BackgroundJobRunner::new();
    let shutdown = crate::shutting_down_token();

    let (tx, mut rx) = tokio::sync::mpsc::channel(5);

    let watch_tx = tx.clone();

    let my_node_id = handle.node_id;

    tracing::debug!("initializing cluster leader change watcher");

    let mut watcher = handle
        .raft
        .on_cluster_leader_change(move |prev, (leader_id, _)| {
            let tx = watch_tx.clone();
            let old_leader_id = prev.map(|x| x.0.node_id);
            tracing::debug!(
                ?old_leader_id,
                new_leader_id = ?leader_id.node_id,
                "cluster leader changed"
            );
            let message = if leader_id.node_id == my_node_id {
                BackgroundJobLeaderMessage::StartBeingLeader
            } else {
                BackgroundJobLeaderMessage::StopBeingLeader
            };
            async move {
                tx.send(message)
                    .await
                    .can_fail("sending notification of leader change")
            }
        });

    tracing::debug!("checking for immediate leadership changes");

    // we might miss the first transition if it happens before our watch is started,
    // so once the watcher is registered, check once by hand
    handle
        .raft
        .with_raft_state(|state| state.server_state.is_leader())
        .map_err(|err| {
            tracing::error!(?err, "unable to determine server state");
            diom_error::Error::internal(err)
        })
        .and_then(|state| {
            let message = if state {
                BackgroundJobLeaderMessage::StartBeingLeader
            } else {
                BackgroundJobLeaderMessage::StopBeingLeader
            };
            tx.send(message).map_err(diom_error::Error::internal)
        })
        .await?;

    tracing::debug!("starting loop waiting to become leader");

    while !shutdown.is_cancelled() {
        tokio::select! {
            message = rx.recv() => {
                tracing::debug!(?my_node_id, ?message, "receive message in leader background process");
                match message {
                    Some(BackgroundJobLeaderMessage::StartBeingLeader) => {
                        runner.spawn_all(cfg.clone(), handle.clone()).await;
                    }
                    Some(BackgroundJobLeaderMessage::StopBeingLeader) => {
                        runner.stop_all().await?;
                    }
                    None => {
                        tracing::warn!("leader detection died");
                        break;
                    }
                }
            },
            res = runner.jobs.join_next(), if !runner.jobs.is_empty() => {
                if let Some(res) = res {
                    tracing::debug!("a background job ended unexpectedly");
                    match res {
                        Ok(Ok(_)) => {},
                        Ok(Err(e)) => {
                            if e.is_forward_to_leader_err() {
                                tracing::trace!("failed a write because we are not the leader");
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
            },
            _ = shutdown.cancelled() => {
                tracing::debug!("shutting down");
                break
            }
        }
    }
    tracing::debug!("shutting down leader-change watcher");
    watcher.close().await;
    tracing::debug!("shutting down any remaining background jobs");
    runner.stop_all().await?;
    tracing::debug!("and we're outta here");
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
    if handle.state_machine.is_loading_snapshot() {
        tracing::warn!("refusing to snapshot while loading another snapshot");
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
    tracing::debug!("starting all-node background jobs");

    let mut last_snapshot_time = std::time::Instant::now();
    let mut last_snapshot_index = handle
        .raft
        .with_raft_state(|st| st.local_committed().copied())
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
            .with_raft_state(|st| (st.local_committed().copied(), st.server_state))
            .await?;

        let delta = match (committed, last_snapshot_index) {
            (Some(a), Some(b)) => Some(a.index - b.index),
            (Some(a), None) => Some(a.index),
            _ => None,
        };
        let (should_snapshot, purge_by, responder) = if let Some(threshold) =
            cfg.cluster.snapshot_after_time
            && last_snapshot_time.elapsed() > threshold.as_duration()
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
    tracing::debug!("shutting down all-node background jobs");
    Ok(())
}
