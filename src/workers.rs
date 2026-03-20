use crate::core::cluster::RaftState;
use coyote_operations::{BackgroundError, BackgroundResult, workers::BackgroundWorker};
use tokio::task::JoinSet;

pub(crate) struct Workers {
    join_set: JoinSet<BackgroundResult<()>>,
}

impl Workers {
    pub(crate) fn new() -> Self {
        Self {
            join_set: JoinSet::new(),
        }
    }

    pub(crate) async fn spawn_all(&mut self, state: RaftState) {
        tracing::debug!("spawning background workers");
        let time = state.state_machine.time.clone();
        {
            tracing::debug!("spawning KV module worker");
            let state = state.state_machine.kv_store().await;
            let time = time.clone();
            self.spawn(coyote_kv::AllNodesWorker::new(state, time));
        }
        {
            tracing::debug!("spawning cache module worker");
            let state = state.state_machine.cache_store().await;
            let time = time.clone();
            self.spawn(coyote_cache::AllNodesWorker::new(state, time));
        }
        {
            tracing::debug!("spawning idempotency module worker");
            let state = state.state_machine.idempotency_store().await;
            let time = time.clone();
            self.spawn(coyote_idempotency::AllNodesWorker::new(state, time));
        }
    }

    pub(crate) async fn shutdown(mut self) {
        tracing::debug!("shutting down all background workers");
        self.join_set.abort_all();
        for item in self.join_set.join_all().await {
            if let Err(err) = item {
                tracing::warn!(?err, "error from background job at shutdown");
            }
        }
    }

    fn spawn<W>(&mut self, job: W)
    where
        W: BackgroundWorker + 'static,
    {
        self.join_set.spawn(async move {
            match job.run_while_handling_panics().await {
                Err(BackgroundError::TooManyPanics) => {
                    tracing::error!(
                        job_name = W::NAME,
                        "background worker had too many panics, shutting down server"
                    );
                    crate::start_shut_down();
                    Ok(())
                }
                other => {
                    tracing::debug!(job_name = W::NAME, res=?other, "background worker has finished");
                    other
                }
            }
        });
    }
}
