use std::{
    future::Future,
    num::NonZeroUsize,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::Instrument;

/// Spawns tokio tasks with a bounded concurrency limit.
///
/// [`TaskNursery::spawn`] ensures at most `limit` tasks run at once. [`TaskNursery::join_all`] waits for every outstanding
/// task to finish.
pub struct TaskNursery {
    semaphore: Arc<Semaphore>,
    tasks: JoinSet<()>,
}

impl TaskNursery {
    /// Log at `warn` when a spawn waits longer than this for a permit.
    const WARN_SPAWN_WAIT: Duration = Duration::from_millis(10);

    /// Create a nursery that runs at most `limit` tasks concurrently.
    pub fn new(limit: NonZeroUsize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(limit.get())),
            tasks: JoinSet::new(),
        }
    }

    /// Spawns the task, similar to `tokio::spawn`. If the limit of concurrent tasks is
    /// reached, this will block until the future can safely be spawned.
    pub async fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let permit = {
            let start_wait = Instant::now();

            let permit = Arc::clone(&self.semaphore)
                .acquire_owned()
                .await
                .expect("nursery semaphore is never closed");

            let wait = start_wait.elapsed();
            if wait > Self::WARN_SPAWN_WAIT {
                tracing::warn!(?wait, "slow task spawn, waiting for nursery permit");
            }

            permit
        };

        let task = async move {
            future.await;
            drop(permit);
        };

        self.tasks.spawn(task.in_current_span());
    }

    /// Wait for all outstanding tasks to finish.
    pub async fn join_all(mut self) {
        // use join_next() over join_all() to avoid propagating panics
        while let Some(result) = self.tasks.join_next().await {
            if let Err(e) = result {
                tracing::error!(error = %e, "panic in spawned task");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TaskNursery;
    use std::{
        num::NonZeroUsize,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };

    #[tokio::test]
    async fn enforces_concurrency_limit() {
        const LIMIT: NonZeroUsize = NonZeroUsize::new(2).unwrap();
        const TASKS: usize = 8;

        let in_flight = Arc::new(AtomicUsize::new(0));
        let max_seen = Arc::new(AtomicUsize::new(0));
        let completed = Arc::new(AtomicUsize::new(0));

        let mut nursery = TaskNursery::new(LIMIT);
        for _ in 0..TASKS {
            let in_flight = in_flight.clone();
            let max_seen = max_seen.clone();
            let completed = completed.clone();
            nursery
                .spawn(async move {
                    let current = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                    max_seen.fetch_max(current, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    in_flight.fetch_sub(1, Ordering::SeqCst);
                    completed.fetch_add(1, Ordering::SeqCst);
                })
                .await;
        }
        nursery.join_all().await;

        assert_eq!(completed.load(Ordering::SeqCst), TASKS);
        assert!(max_seen.load(Ordering::SeqCst) <= LIMIT.get());
        assert_eq!(in_flight.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn spawn_blocks_until_permit_frees() {
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        let mut nursery = TaskNursery::new(NonZeroUsize::new(1).unwrap());

        // First task holds the only permit for a while.
        let order1 = order.clone();
        nursery
            .spawn(async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                order1.lock().unwrap().push("first");
            })
            .await;

        // This spawn must block until the first task releases its permit, so the
        // "first" marker is pushed before "second".
        let order2 = order.clone();
        nursery
            .spawn(async move {
                order2.lock().unwrap().push("second");
            })
            .await;

        nursery.join_all().await;

        assert_eq!(*order.lock().unwrap(), vec!["first", "second"]);
    }

    #[tokio::test]
    async fn join_all_waits_for_completion() {
        const TASKS: usize = 5;
        let completed = Arc::new(AtomicUsize::new(0));

        let mut nursery = TaskNursery::new(NonZeroUsize::new(3).unwrap());
        for _ in 0..TASKS {
            let completed = completed.clone();
            nursery
                .spawn(async move {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    completed.fetch_add(1, Ordering::SeqCst);
                })
                .await;
        }
        nursery.join_all().await;

        assert_eq!(completed.load(Ordering::SeqCst), TASKS);
    }

    #[tokio::test]
    async fn panicking_task_does_not_break_join() {
        let ok_ran = Arc::new(AtomicUsize::new(0));

        let mut nursery = TaskNursery::new(NonZeroUsize::new(2).unwrap());
        nursery
            .spawn(async {
                panic!("boom");
            })
            .await;
        let ok_ran_clone = ok_ran.clone();
        nursery
            .spawn(async move {
                ok_ran_clone.fetch_add(1, Ordering::SeqCst);
            })
            .await;

        nursery.join_all().await;

        assert_eq!(ok_ran.load(Ordering::SeqCst), 1);
    }
}
