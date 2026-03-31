use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use diom_id::NamespaceId;
use tokio::sync::Notify;

use crate::entities::{Partition, TopicName};

type Key = (NamespaceId, TopicName);

struct WaiterState {
    /// An empty vec is treated as "all partitions"
    partitions: Vec<Partition>,
    count: AtomicU64,
    notify: Notify,
}

struct Inner {
    waiters: Mutex<HashMap<Key, Vec<Arc<WaiterState>>>>,
    closed: AtomicBool,
    /// Notified on shutdown to wake all waiters.
    shutdown: Notify,
}

/// Notifies when a topic has published messages to it.
#[derive(Clone)]
pub struct TopicPublishNotifier {
    inner: Arc<Inner>,
}

/// Can be `await`ed to block until messages are published to a topic + partition.
pub struct Notified {
    state: Arc<WaiterState>,
    inner: Arc<Inner>,
    key: Key,
}

impl TopicPublishNotifier {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                waiters: Mutex::new(HashMap::new()),
                closed: AtomicBool::new(false),
                shutdown: Notify::new(),
            }),
        }
    }

    /// Sends a notification that messages were published to the topic + partition.
    pub fn notify_published(
        &self,
        namespace: NamespaceId,
        topic: TopicName,
        partition: Partition,
        count: u64,
    ) {
        let map = self.inner.waiters.lock().unwrap();
        let Some(waiters) = map.get(&(namespace, topic)) else {
            return;
        };
        for waiter in waiters {
            if waiter.partitions.is_empty() || waiter.partitions.contains(&partition) {
                waiter.count.fetch_add(count, Ordering::Release);
                waiter.notify.notify_waiters();
            }
        }
    }

    /// Returns a Notified that can be `await`ed
    pub fn register_notifier(
        &self,
        namespace: NamespaceId,
        topic: TopicName,
        partitions: Vec<Partition>,
    ) -> Notified {
        let state = Arc::new(WaiterState {
            partitions,
            count: AtomicU64::new(0),
            notify: Notify::new(),
        });
        let key = (namespace, topic);

        self.inner
            .waiters
            .lock()
            .unwrap()
            .entry(key.clone())
            .or_default()
            .push(Arc::clone(&state));

        Notified {
            state,
            inner: Arc::clone(&self.inner),
            key,
        }
    }
}

impl Notified {
    /// Waits for the number of messages to be published to the registered topic + partitions.
    /// Returns early if the notifier is shut down.
    pub async fn wait(&mut self, count: u64) {
        loop {
            let notified = self.state.notify.notified();
            let shutdown = self.inner.shutdown.notified();
            if self.inner.closed.load(Ordering::Acquire) {
                return;
            }
            if self.state.count.load(Ordering::Acquire) >= count {
                return;
            }
            tokio::select! {
                _ = notified => {},
                _ = shutdown => return,
            }
        }
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        self.closed.store(true, Ordering::Release);
        self.shutdown.notify_waiters();
    }
}

impl Drop for Notified {
    fn drop(&mut self) {
        let mut map = self.inner.waiters.lock().unwrap();
        if let Some(waiters) = map.get_mut(&self.key) {
            waiters.retain(|w| !Arc::ptr_eq(w, &self.state));
            if waiters.is_empty() {
                map.remove(&self.key);
            }
        }
    }
}
