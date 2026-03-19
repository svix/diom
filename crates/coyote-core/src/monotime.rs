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
pub struct Monotime {
    time: Arc<AtomicI64>,
}

impl Monotime {
    pub fn initial() -> Self {
        Self {
            time: Arc::new(AtomicI64::new(0)),
        }
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
        self.time.fetch_max(other, Ordering::AcqRel).max(other)
    }

    fn as_i64(&self) -> i64 {
        self.time.load(Ordering::Acquire)
    }

    #[cfg(feature = "mockable-time")]
    /// Advance time by the given amount. This should only be used in integration tests.
    pub fn fast_forward(&self, by: std::time::Duration) -> Timestamp {
        let millis: i64 = by.as_millis().try_into().expect("duration out of bounds");
        tracing::trace!(millis, "fast-forwarding time");
        self.time.fetch_add(millis, Ordering::AcqRel);
        self.last()
    }
}

#[cfg(test)]
mod tests {
    use super::Monotime;
    use jiff::Timestamp;
    use std::time::Duration;

    #[track_caller]
    fn assert_approximately_equal(ts1: Timestamp, ts2: Timestamp) {
        let ts1_millis = ts1.as_millisecond();
        let ts2_millis = ts2.as_millisecond();
        assert!(ts1_millis.abs_diff(ts2_millis) < 5);
    }

    #[test]
    fn test_basic_behavior() {
        let mt = Monotime::initial();
        // initially, it's empty
        assert_eq!(mt.last(), Timestamp::UNIX_EPOCH);
        // calling .now returns the current time and bumps `last`
        let now = Timestamp::now();
        assert_approximately_equal(now, mt.now());
        assert_approximately_equal(now, mt.last());
        let future = now + Duration::from_mins(10);
        // bumping into the future prevents now from rewinding
        mt.bump(future);
        assert_approximately_equal(mt.last(), future);
        assert_approximately_equal(mt.now(), future);
    }

    #[cfg(feature = "mockable-time")]
    #[test]
    fn test_mockable_time() {
        let mt = Monotime::initial();
        assert_eq!(mt.last(), Timestamp::UNIX_EPOCH);
        mt.fast_forward(Duration::from_hours(1));
        assert_eq!(mt.last().as_second(), 3600);
        // we can still keep moving forward, e.g., with now()
        assert_approximately_equal(Timestamp::now(), mt.now());
        mt.fast_forward(Duration::from_hours(1));
        assert_approximately_equal(Timestamp::now() + Duration::from_hours(1), mt.now());
        assert_approximately_equal(Timestamp::now() + Duration::from_hours(1), mt.last());
        // running .now won't rewind us, monotonicity stands
        assert!(mt.now() > Timestamp::now());
    }
}
