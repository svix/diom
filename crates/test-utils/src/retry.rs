use std::time::Duration;

use anyhow::Result;

pub async fn run_with_retries<O>(f: impl AsyncFnMut() -> Result<O>) -> Result<O> {
    run_with_n_retries(f, 32).await
}

pub async fn run_with_many_retries<O>(f: impl AsyncFnMut() -> Result<O>) -> Result<O> {
    run_with_n_retries(f, 48).await
}

async fn run_with_n_retries<O>(mut f: impl AsyncFnMut() -> Result<O>, n: usize) -> Result<O> {
    let mut dur = Duration::from_millis(15);
    for attempt in 0..n {
        let result = f().await;
        let Err(err) = result else { return result };

        tracing::error!("Attempt {attempt}: {err}");
        tokio::time::sleep(dur).await;
        if dur < Duration::from_millis(480) {
            dur *= 2;
        }
    }

    anyhow::bail!("All attempts failed");
}
