use futures_util::future::FutureExt;
use std::{panic::AssertUnwindSafe, time::Duration};

const MAX_PANICS_PER_JOB: usize = 10;

pub trait BackgroundWorker: Clone + Send {
    const NAME: &str;

    fn run(self) -> impl Future<Output = crate::BackgroundResult<()>> + Send;

    fn run_while_handling_panics(self) -> impl Future<Output = crate::BackgroundResult<()>> + Send {
        async move {
            let shutting_down = diom_core::shutdown::shutting_down_token();
            let mut backoff = diom_core::backoff::ExponentialBackoffWithJitter::new(
                Duration::from_millis(10),
                Duration::from_secs(5),
            );
            let mut failures = 0;
            let job_name = Self::NAME;

            loop {
                let job = self.clone();
                match AssertUnwindSafe(job.run()).catch_unwind().await {
                    Ok(v) => v?,
                    Err(err) => {
                        failures += 1;
                        if failures > MAX_PANICS_PER_JOB {
                            tracing::error!(
                                ?err,
                                job_name,
                                "a panic occurred during a background job; shutting down the server"
                            );
                            return Err(crate::BackgroundError::TooManyPanics);
                        } else {
                            tracing::error!(
                                ?err,
                                job_name,
                                "a panic occurred during a background job; restarting"
                            );
                        }
                    }
                }
                if shutting_down
                    .run_until_cancelled(backoff.backoff())
                    .await
                    .is_none()
                {
                    return Ok(());
                }
            }
        }
    }
}
