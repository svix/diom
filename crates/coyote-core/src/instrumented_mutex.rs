use parking_lot::Mutex;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

pub struct InstrumentedMutex<T> {
    id: u64,
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for InstrumentedMutex<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            inner: Arc::clone(&self.inner),
        }
    }
}

static ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

impl<T> InstrumentedMutex<T> {
    const WARN_LOCK_TIME: Duration = Duration::from_millis(10);

    pub fn new(t: T) -> Self {
        let id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            inner: Arc::new(Mutex::new(t)),
        }
    }

    pub fn lock(&self, caller: &'static str) -> InstrumentedMutexGuard<'_, T> {
        let start_acquire = Instant::now();
        let guard = self.inner.lock();
        let acquire_time = Instant::now();
        let duration = acquire_time
            .checked_duration_since(start_acquire)
            .unwrap_or_default();
        if duration > Self::WARN_LOCK_TIME {
            tracing::warn!(
                caller,
                lock_id = self.id,
                ?duration,
                "slow lock acquisition!"
            )
        }
        InstrumentedMutexGuard {
            caller,
            lock_id: self.id,
            acquire_time,
            guard,
        }
    }

    /// Return the enclosed data if and only if there is exactly one strong reference
    pub fn try_into_inner(self) -> Option<T> {
        Arc::into_inner(self.inner).map(|m| m.into_inner())
    }
}

pub struct InstrumentedMutexGuard<'a, T> {
    caller: &'static str,
    lock_id: u64,
    acquire_time: Instant,
    guard: parking_lot::MutexGuard<'a, T>,
}

impl<'a, T: 'a> std::ops::Deref for InstrumentedMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<'a, T: 'a> std::ops::DerefMut for InstrumentedMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guard.deref_mut()
    }
}

impl<'a, T> InstrumentedMutexGuard<'a, T> {}

impl<'a, T> Drop for InstrumentedMutexGuard<'a, T> {
    fn drop(&mut self) {
        let duration = self.acquire_time.elapsed();
        if duration > InstrumentedMutex::<T>::WARN_LOCK_TIME {
            tracing::warn!(
                caller = self.caller,
                lock_id = self.lock_id,
                ?duration,
                "lock held for a long time"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InstrumentedMutex;
    use std::{
        sync::{Arc, Barrier},
        time::Duration,
    };
    use tracing_test::traced_test;

    #[test]
    fn test_deref_impls() {
        let mutex = InstrumentedMutex::new(0);
        let mut guard = mutex.lock("caller");
        assert_eq!(*guard, 0);
        *guard = 1;
        assert_eq!(*guard, 1);
        drop(guard);
        assert_eq!(mutex.try_into_inner(), Some(1));
    }

    #[test]
    #[traced_test]
    fn test_does_not_log_when_held_briefly() {
        let mutex = InstrumentedMutex::new("foo");
        let guard = mutex.lock("key");
        assert_eq!(*guard, "foo");
        drop(guard);
        assert!(!logs_contain("lock held for a long time"));
        assert!(!logs_contain("slow lock acquisition"));
    }

    #[test]
    #[traced_test]
    fn test_does_log_when_held_for_a_long_time() {
        let mutex = InstrumentedMutex::new("foo");
        let guard = mutex.lock("key");
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(*guard, "foo");
        drop(guard);
        assert!(logs_contain("lock held for a long time"));
        assert!(!logs_contain("slow lock acquisition"));
    }

    #[test]
    #[traced_test]
    fn test_does_log_when_waiting_for_a_long_time() {
        let mutex1 = InstrumentedMutex::new("foo");
        let span = tracing::Span::current();
        std::thread::scope(|s| {
            let barrier = Arc::new(Barrier::new(2));
            let barrier2 = Arc::clone(&barrier);
            let mutex2 = mutex1.clone();
            let span2 = span.clone();
            s.spawn(move || {
                // need to propagate the tracing span for traced_test to work
                let _g = span.entered();
                let _guard = mutex1.lock("thread 1");
                barrier.wait();
                std::thread::sleep(Duration::from_millis(100));
            });
            s.spawn(move || {
                // need to propagate the tracing span for traced_test to work
                let _g = span2.entered();
                barrier2.wait();
                let _guard = mutex2.lock("thread 2");
            });
        });
        assert!(logs_contain("slow lock acquisition"));
    }
}
