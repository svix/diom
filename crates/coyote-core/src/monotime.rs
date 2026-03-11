use jiff::Timestamp;
use std::sync::{
    Arc,
    atomic::{AtomicI64, Ordering},
};

/// A monotonic clock. Stores time internally as milliseconds since Unix epoch. In the event that
/// wall clock goes backwards, this will stall until the clock catches up.
///
/// This structure is cheaply cloneable and clones all point at the same underlying data
/// (internally, this is an Arc).
#[derive(Debug, Clone)]
pub struct Monotime(Arc<AtomicI64>);

impl Monotime {
    pub fn initial() -> Self {
        Self(Arc::new(AtomicI64::new(0)))
    }

    /// Get the last time that the leader set
    pub fn last(&self) -> Timestamp {
        Timestamp::from_millisecond(self.as_i64())
            .expect("time was wildly outside of acceptable ranges, I must crash")
    }

    /// Get a monotonic version of the current time, updating our internal knowledge about time.
    pub fn now(&self) -> Timestamp {
        Timestamp::from_millisecond(self.now_raw())
            .expect("time was wildly outside of acceptable ranges, I must crash")
    }

    /// Get a monotonic version of the current time as the number of millis since epoch, updating our internal knowledge about time.
    fn now_raw(&self) -> i64 {
        let now = Timestamp::now().as_millisecond();
        self.bump_raw(now)
    }

    /// Ingest another timestamp
    pub fn bump(&self, other: Timestamp) {
        self.bump_raw(other.as_millisecond());
    }

    fn bump_raw(&self, other: i64) -> i64 {
        self.0.fetch_max(other, Ordering::AcqRel).max(other)
    }

    fn as_i64(&self) -> i64 {
        self.0.load(Ordering::Acquire)
    }
}
