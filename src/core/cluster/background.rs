use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::{Node, NodeId, handle::RaftState};
use crate::cfg::Configuration;
use openraft::error::{ClientWriteError, RaftError};
use tokio::task::JoinSet;

trait BackgroundJob {
    async fn run_on_leader(self) -> anyhow::Result<()>;
}

struct RecordLogTimestamps {
    cfg: Configuration,
    handle: RaftState,
}

impl BackgroundJob for RecordLogTimestamps {
    async fn run_on_leader(self) -> anyhow::Result<()> {
        let mut ticker = tokio::time::interval(self.cfg.cluster.log_index_interval);
        loop {
            tracing::debug!("recording log timestamps");
            let now = jiff::Timestamp::now();
            self.handle
                .raft
                .client_write(super::handle::Request::ClusterInternal(
                    super::operations::InternalRequest::RecordLogTimestamp(now),
                ))
                .await?;
            ticker.tick().await;
        }
    }
}

struct BackgroundJobRunner {
    jobs: JoinSet<anyhow::Result<()>>,
}

fn is_forward_to_leader_err(e: &anyhow::Error) -> bool {
    if let Some(raft_err) = e.downcast_ref::<RaftError<NodeId, ClientWriteError<NodeId, Node>>>() {
        raft_err.forward_to_leader().is_some()
    } else {
        false
    }
}

impl BackgroundJobRunner {
    fn spawn_all(cfg: Configuration, handle: RaftState) -> Self {
        let mut jobs = JoinSet::new();
        jobs.spawn(RecordLogTimestamps { cfg, handle }.run_on_leader());
        Self { jobs }
    }

    async fn stop_all(mut self) -> anyhow::Result<()> {
        self.jobs.abort_all();
        while let Some(job) = self.jobs.join_next().await {
            match job {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    if is_forward_to_leader_err(&e) {
                        tracing::trace!("some worker died with forward-to-leader, who cares");
                    } else {
                        return Err(e);
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
                                if is_forward_to_leader_err(&e) {
                                    tracing::debug!("failed a write because we are not the leader");
                                    break;
                                } else {
                                    runner.stop_all().await?;
                                    return Err(e);
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
