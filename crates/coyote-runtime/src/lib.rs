use std::{
    any::Any,
    hash::{DefaultHasher, Hash, Hasher},
    panic::AssertUnwindSafe,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread::JoinHandle,
};

use tokio::sync::mpsc::{Receiver, Sender};

type Task<State> = Box<dyn FnOnce(&mut State) + Send + 'static>;

/// A captured panic from a closure thread.
#[derive(Debug)]
pub struct PanicError(Box<dyn Any + Send + 'static>);

impl std::fmt::Display for PanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Is there a better type we should be downcasting too here?
        if let Some(s) = self.0.downcast_ref::<&str>() {
            write!(f, "worker panic: {s}")
        } else if let Some(s) = self.0.downcast_ref::<String>() {
            write!(f, "worker panic: {s}")
        } else {
            write!(f, "worker panic")
        }
    }
}

impl std::error::Error for PanicError {}

/// A handle to the pool of Coyote workers.
///
/// The coyote runtime consists of a dedicated threadpool for IO, where all Database interactions are delegated.
///
/// Additionally, some resources and operations are "pinned" to specific threads.
/// This ensures that there isn't any concurrent access to these resources, even when the Coyote server is handling
/// concurrent requests for the same resource.
///
/// When the WorkerPool is DROPed, it waits for all threads to finish.
pub struct WorkerPool<State> {
    workers: Vec<Worker<State>>,
}

struct Worker<State> {
    sender: Option<Sender<Task<State>>>,
    handle: Option<JoinHandle<()>>,
    pending_tasks: Arc<AtomicUsize>,
}

impl<State: Clone + Send + 'static> WorkerPool<State> {
    /// Spawns the worker pool and all of its threads.
    pub fn spawn(state: State, thread_count: usize) -> std::io::Result<Self> {
        let mut workers = Vec::with_capacity(thread_count);

        for i in 0..thread_count {
            let (sender, receiver) = tokio::sync::mpsc::channel::<Task<State>>(1);

            let thread_state = state.clone();

            let handle = std::thread::Builder::new()
                .name(format!("coyote-worker-{i}"))
                .spawn(move || Self::worker_loop(thread_state, receiver))?;

            workers.push(Worker {
                sender: Some(sender),
                handle: Some(handle),
                pending_tasks: Arc::new(AtomicUsize::new(0)),
            });
        }

        Ok(Self { workers })
    }

    fn worker_loop(mut state: State, mut receiver: Receiver<Task<State>>) {
        while let Some(task) = receiver.blocking_recv() {
            task(&mut state);
        }
    }

    /// Runs the given function on a thread determined by hashing the target.
    ///
    /// Operations on the same target will always run on the same thread, ensuring
    /// serialized access without explicit locking.
    ///
    /// Returns `Err(PanicError)` if the closure panics.
    pub async fn run_on_owned_thread<Output>(
        &self,
        target: impl Hash,
        func: impl FnOnce(&mut State) -> Output + Send + 'static,
    ) -> Result<Output, PanicError>
    where
        Output: Send + 'static,
    {
        let mut hasher = DefaultHasher::new();
        target.hash(&mut hasher);
        let hash = hasher.finish();
        let worker_index = hash as usize;

        self.run_on_worker(worker_index, func).await
    }

    /// Runs the given function on the worker thread with the least pending tasks.
    ///
    /// Returns `Err(PanicError)` if the closure panics.
    pub async fn run_on_any_thread<Output>(
        &self,
        func: impl FnOnce(&mut State) -> Output + Send + 'static,
    ) -> Result<Output, PanicError>
    where
        Output: Send + 'static,
    {
        let worker_index = self
            .workers
            .iter()
            .enumerate()
            .min_by_key(|(_, w)| w.pending_tasks.load(Ordering::Relaxed))
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.run_on_worker(worker_index, func).await
    }

    async fn run_on_worker<Output>(
        &self,
        worker_index: usize,
        func: impl FnOnce(&mut State) -> Output + Send + 'static,
    ) -> Result<Output, PanicError>
    where
        Output: Send + 'static,
    {
        let worker_index = worker_index % self.workers.len();
        let worker = &self.workers[worker_index];
        let pending_tasks = Arc::clone(&worker.pending_tasks);

        pending_tasks.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = tokio::sync::oneshot::channel();

        let task: Task<State> = Box::new(move |state| {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| func(state)));
            pending_tasks.fetch_sub(1, Ordering::Relaxed);
            if tx.send(result).is_err() {
                tracing::warn!("Worker output dropped.");
            }
        });

        worker
            .sender
            .as_ref()
            .expect("sender is only None once DROPed")
            .send(task)
            .await
            .expect("worker thread has terminated unexpectedly");

        rx.await
            .expect("worker thread has terminated unexpectedly")
            .map_err(PanicError)
    }
}

impl<State> Drop for WorkerPool<State> {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // Close the channels so the threads shutdown.
            let _ = worker.sender.take();
        }

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread::{self};

    #[tokio::test]
    async fn run_on_owned_thread_executes_closure() {
        let pool = WorkerPool::spawn(42_i32, 4).unwrap();

        let result = pool
            .run_on_owned_thread("key", |state| *state * 2)
            .await
            .unwrap();

        assert_eq!(result, 84);
    }

    #[tokio::test]
    async fn run_on_any_thread_executes_closure() {
        let pool = WorkerPool::spawn(42_i32, 4).unwrap();

        let result = pool.run_on_any_thread(|state| *state * 2).await.unwrap();

        assert_eq!(result, 84);
    }

    #[tokio::test]
    async fn run_on_owned_thread_routes_same_key_to_same_thread() {
        let pool = WorkerPool::spawn((), 10).unwrap();

        // Same key should always route to the same thread
        let mut thread_ids = Vec::new();
        for _ in 0..10 {
            let id = pool
                .run_on_owned_thread("same_key", |_| thread::current().id())
                .await
                .unwrap();
            thread_ids.push(id);
        }

        let control = thread_ids[0];
        assert_eq!(thread_ids, vec![control; 10]);

        // Different keys should route to different threads
        let mut distinct_threads = HashSet::new();
        for i in 0..10 {
            let id = pool
                .run_on_worker(i, |_| thread::current().id())
                .await
                .unwrap();
            distinct_threads.insert(id);
        }

        assert!(
            distinct_threads.len() == 10,
            "expected different keys to route to different threads {distinct_threads:?}"
        );
    }

    #[tokio::test]
    async fn worker_survives_panic_in_closure() {
        let pool = WorkerPool::spawn((), 1).unwrap();

        let result: Result<(), PanicError> = pool
            .run_on_owned_thread("key", |_| panic!("intentional panic"))
            .await;

        assert!(result.is_err(), "expected panic to be captured as Err");

        // The worker should still be able to handle new tasks
        let result = pool.run_on_owned_thread("key", |_| 42).await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn run_on_any_thread_picks_least_busy_worker() {
        let num_workers = 4;
        let idle_worker_index = num_workers - 1;
        let pool = Arc::new(WorkerPool::spawn((), num_workers).unwrap());

        // Get the thread ID of the idle worker before saturating others
        let idle_thread_id = pool
            .run_on_worker(idle_worker_index, |_| thread::current().id())
            .await
            .unwrap();

        // Create channels to block workers 0..N-1
        let mut release_senders = Vec::new();
        let mut blocking_handles = Vec::new();

        for worker_idx in 0..idle_worker_index {
            let (release_tx, release_rx) = std::sync::mpsc::sync_channel(0);
            release_senders.push(release_tx);

            let pool_clone = Arc::clone(&pool);
            let handle = tokio::spawn(async move {
                pool_clone
                    .run_on_worker(worker_idx, move |_| {
                        // Block until signaled
                        let _ = release_rx.recv();
                    })
                    .await
                    .unwrap();
            });
            blocking_handles.push(handle);
        }

        // Give the blocking tasks time to start executing on their workers
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        // Now call run_on_any_thread - it should pick the idle worker
        // Use a timeout to fail fast if it picks a busy worker
        let selected_thread_id = tokio::time::timeout(
            tokio::time::Duration::from_millis(500),
            pool.run_on_any_thread(|_| thread::current().id()),
        )
        .await
        .expect("run_on_any_thread should complete quickly by picking idle worker")
        .unwrap();

        assert_eq!(
            selected_thread_id, idle_thread_id,
            "run_on_any_thread should pick the idle worker"
        );

        // Release the blocked workers
        for sender in release_senders {
            let _ = sender.send(());
        }

        // Wait for all blocking tasks to complete
        for handle in blocking_handles {
            handle.await.unwrap();
        }
    }
}
