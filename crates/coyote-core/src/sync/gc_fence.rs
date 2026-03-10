use parking_lot::{Mutex, RwLock};
use std::{
    borrow::Borrow,
    collections::BTreeSet,
    hash::{DefaultHasher, Hash, Hasher},
    marker::PhantomData,
    sync::Arc,
};

// the H is in here to ensure that every value we mark
// is the same underlying type
#[allow(clippy::extra_unused_type_parameters)]
fn hash_value<H, V>(value: &V) -> u64
where
    H: Borrow<V>,
    V: Hash + 'static + ?Sized,
{
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[allow(unused)]
#[must_use]
pub struct Guard(parking_lot::ArcRwLockReadGuard<parking_lot::RawRwLock, GcFenceGc>);

pub struct DrainGuard<H: Hash + 'static> {
    writer: parking_lot::ArcMutexGuard<parking_lot::RawMutex, GcFenceWriter>,
    gc: parking_lot::ArcRwLockWriteGuard<parking_lot::RawRwLock, GcFenceGc>,
    _phantom: PhantomData<H>,
}

impl<H: Hash + 'static> DrainGuard<H> {
    pub fn contains<V>(&self, key: &V) -> bool
    where
        V: Hash + 'static + ?Sized,
        H: Borrow<V>,
    {
        let hash = hash_value::<H, V>(key);
        self.gc.intent_to_gc.contains(&hash) && !self.writer.written_recently.contains(&hash)
    }
}

impl<H: Hash + 'static> Drop for DrainGuard<H> {
    fn drop(&mut self) {
        self.gc.intent_to_gc.clear();
        self.writer.written_recently.clear();
    }
}

pub struct Marker<H: Hash + 'static> {
    writer_side: Arc<Mutex<GcFenceWriter>>,
    gc_side: Arc<RwLock<GcFenceGc>>,
    _phantom: PhantomData<H>,
}

impl<H: Hash + 'static> Marker<H> {
    /// Mark a series values for GC
    ///
    /// This only holds a lock briefly while inserting into the set.
    pub fn intent_to_gc<'a, I, V>(&self, values: I)
    where
        V: Hash + 'static + ?Sized,
        H: Borrow<V>,
        I: IntoIterator<Item = &'a V>,
    {
        let mut inner = self.gc_side.write();
        for value in values {
            let key = hash_value::<H, V>(value);
            inner.intent_to_gc.insert(key);
        }
    }

    /// Drain this object to perform a GC.
    ///
    /// This returns a proxy object which you should call `.contains` on for
    /// each key you intend to GC. When that proxy is dropped, the fence will
    /// be reset.
    ///
    /// If there are concurrent writes, this will block until they finish and drop
    /// their `Guard`s.
    pub fn drain_all(&self) -> DrainGuard<H> {
        let writer = self.writer_side.lock_arc();
        let gc = self.gc_side.write_arc();
        DrainGuard {
            writer,
            gc,
            _phantom: PhantomData,
        }
    }

    /// Drain this object to perform a GC.
    ///
    /// This returns a proxy object which you should call `.contains` on for
    /// each key you intend to GC. When that proxy is dropped, the fence will
    /// be reset.
    ///
    /// If there are concurrent writes, this will return None.
    pub fn try_drain_all(&self) -> Option<DrainGuard<H>> {
        let writer = self.writer_side.lock_arc();
        let gc = self.gc_side.try_write_arc()?;
        Some(DrainGuard {
            writer,
            gc,
            _phantom: PhantomData,
        })
    }
}

impl<H: Hash + 'static> Drop for Marker<H> {
    fn drop(&mut self) {
        let mut inner = self.writer_side.lock();
        inner.marking = false;
        inner.written_recently.clear();
    }
}

/// A helper structure for fencing data accesses around garbage collection
///
/// This is designed to allow concurrent writes and (unboundedly-expensive) garbage collection
/// with minimal lock time. The algorithm is basically as below:
///
/// Writers and GC threads must share a single instance of `GcFence`.
///
/// The garbage collector first marks the values it intends to collect by calling `.intent_to_gc()`
/// on them. After marking finishes, it can sweep all values that are still marked.
///
/// Every time a writer touches a values, it immunizes it from the current round of garbage
/// collection by calling `.want_to_write()`, and grabs a rwlock inhibiting future rounds from
/// starting. Writers are never blocked (except momentarily while inserting into the set of marked
/// values).
///
/// Because we aren't really a garbage collector, we represent values by their Hash value; this may
/// mean that some values aren't garbage-collectable because a conflicting write occurred to a
/// different key with the
#[derive(Clone)]
pub struct GcFence<H: Hash + 'static> {
    writer_side: Arc<Mutex<GcFenceWriter>>,
    gc_side: Arc<RwLock<GcFenceGc>>,
    _phantom: PhantomData<H>,
}

#[derive(Default)]
struct GcFenceWriter {
    marking: bool,
    written_recently: BTreeSet<u64>,
}

#[derive(Default)]
struct GcFenceGc {
    intent_to_gc: BTreeSet<u64>,
}

impl<H: Hash + 'static> GcFence<H> {
    pub fn new() -> Self {
        Self {
            writer_side: Arc::new(Mutex::new(GcFenceWriter::default())),
            gc_side: Arc::new(RwLock::new(GcFenceGc::default())),
            _phantom: PhantomData,
        }
    }

    /// Enable marking
    ///
    /// This must be called before starting any scan of the data.
    pub fn start_marking(&self) -> Marker<H> {
        let mut inner = self.writer_side.lock_arc();
        // we take this lock to ensure that there's a fence between want_to_write calls finishing
        // and start_marking calls
        let gc = self.gc_side.write_arc();
        inner.marking = true;
        let writer_side = parking_lot::ArcMutexGuard::into_arc(inner);
        let gc_side = parking_lot::ArcRwLockWriteGuard::into_arc(gc);
        Marker {
            writer_side,
            gc_side,
            _phantom: PhantomData,
        }
    }

    /// Mark a single value as ineligible for GC
    ///
    /// This returns a guard which will inhibit the collection phase of GC
    /// and should be held until the underlying write is externally-visible.
    pub fn want_to_write<V>(&self, value: &V) -> Guard
    where
        V: Hash + 'static + ?Sized,
        H: Borrow<V>,
    {
        let key = hash_value::<H, V>(value);
        let mut write = self.writer_side.lock();
        let gc = self.gc_side.read_arc();
        if write.marking {
            write.written_recently.insert(key);
        }
        Guard(gc)
    }

    /// The number of values currently marked for GC
    pub fn len(&self) -> usize {
        let write = self.writer_side.lock();
        let gc = self.gc_side.read();
        gc.intent_to_gc.difference(&write.written_recently).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::GcFence;
    use std::sync::Arc;

    #[test]
    fn test_basic() {
        let fence = GcFence::<u32>::new();
        let marker = fence.start_marking();
        let to_remove = [1, 2, 3, 4, 5];
        marker.intent_to_gc(&to_remove);
        assert_eq!(fence.len(), 5);
        let _ = fence.want_to_write(&1);
        assert_eq!(fence.len(), 4);
        let drain_guard = marker.try_drain_all().unwrap();
        assert!(!drain_guard.contains(&1));
        assert!(drain_guard.contains(&2));
        assert!(drain_guard.contains(&3));
        assert!(drain_guard.contains(&4));
        assert!(drain_guard.contains(&5));
        drop(drain_guard);
        assert_eq!(fence.len(), 0);
    }

    #[test]
    fn test_want_to_write_calls_do_not_block_each_other() {
        let gcfence = GcFence::<u32>::new();
        gcfence.start_marking();

        std::thread::scope(|s| {
            let barrier = Arc::new(std::sync::Barrier::new(2));

            let barrier1 = Arc::clone(&barrier);
            let fence1 = gcfence.clone();
            s.spawn(move || {
                let _guard = fence1.want_to_write(&0);
                barrier1.wait();
            });

            let barrier2 = Arc::clone(&barrier);
            let fence2 = gcfence.clone();
            s.spawn(move || {
                let _guard = fence2.want_to_write(&0);
                barrier2.wait();
            });
        });
    }

    #[test]
    fn test_want_to_write_calls_inhibit_gc() {
        let gcfence = GcFence::<u32>::new();
        let marker = gcfence.start_marking();

        marker.intent_to_gc([&0, &1]);

        std::thread::scope(|s| {
            let finished_inhibiting = Arc::new(std::sync::Barrier::new(2));
            let first_drain = Arc::new(std::sync::Barrier::new(2));
            let second_drain = Arc::new(std::sync::Barrier::new(2));
            let second_drain_finished = Arc::new(std::sync::Barrier::new(2));

            let fi1 = Arc::clone(&finished_inhibiting);
            let fd1 = Arc::clone(&first_drain);
            let sd1 = Arc::clone(&second_drain);
            let sf1 = Arc::clone(&second_drain_finished);
            let fence1 = gcfence.clone();
            s.spawn(move || {
                let _guard = fence1.want_to_write(&0);
                fi1.wait();
                fd1.wait();
                drop(_guard);
                sd1.wait();
                sf1.wait();
                let _ = fence1.want_to_write(&1);
            });

            let fi2 = Arc::clone(&finished_inhibiting);
            let fd2 = Arc::clone(&first_drain);
            let sd2 = Arc::clone(&second_drain);
            let sf2 = Arc::clone(&second_drain_finished);
            s.spawn(move || {
                fi2.wait();
                // at this point, the writer still holds a read lock guard
                assert!(marker.try_drain_all().is_none());
                fd2.wait();
                sd2.wait();
                // but now it doesn't, so we can proceed
                let drainer = marker.try_drain_all().unwrap();
                assert!(!drainer.contains(&0));
                assert!(drainer.contains(&1));
                sf2.wait();
                drop(drainer);
            });
        });
    }
}
