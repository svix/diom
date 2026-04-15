use crate::{AppState, core::cluster::RaftState};
use diom_operations::{BackgroundError, BackgroundResult, workers::BackgroundWorker};
use tokio::task::JoinSet;

pub(crate) struct Workers {
    #[allow(unused)]
    app_state: AppState,
    join_set: JoinSet<BackgroundResult<()>>,
}

impl Workers {
    pub(crate) fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            join_set: JoinSet::new(),
        }
    }

    pub(crate) async fn spawn_all(&mut self, state: RaftState) {
        tracing::debug!("spawning background workers");
        {
            tracing::debug!("spawning msgs module worker");
            let state = state.state_machine.msgs_store().await;
            self.spawn(diom_msgs::AllNodesWorker::new(state));
        }
    }

    pub(crate) async fn shutdown(mut self) {
        tracing::debug!("shutting down all background workers");
        self.join_set.abort_all();
        while let Some(job) = self.join_set.join_next().await {
            match job {
                Ok(_) => {}
                Err(e) if e.is_cancelled() => {}
                Err(err) => tracing::warn!(?err, "error from background job at shutdown"),
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
