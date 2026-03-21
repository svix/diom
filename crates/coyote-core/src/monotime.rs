#![allow(clippy::disallowed_methods)]

use jiff::Timestamp;
use std::sync::{
    Arc,
    atomic::{AtomicI64, Ordering},
};

/// A monotonic clock. Stores time internally as milliseconds since Unix epoch. In the event that
/// wall clock goes backwards, this will stall until the clock catches up.
///
/// This structure is cheaply cloneable and clones all point at the same underlying data
/// (internally, this is an Arc). Time is stored with millisecond granularity.
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

    /// Get the last time that the leader set.
    ///
    /// This time only advances when we receive a message through replication (which the Ticker
    /// guarantees will be every several hundred milliseconds).
    pub fn now(&self) -> Timestamp {
        Timestamp::from_millisecond(self.as_i64())
            .expect("time was wildly outside of acceptable ranges, I must crash")
    }

    /// Get a monotonic version of the current time, updating our internal knowledge about time.
    ///
    /// This should only be called on the leader when generating a log message.
    pub fn update_now(&self) -> Timestamp {
        Timestamp::from_millisecond(self.update_now_raw())
            .expect("time was wildly outside of acceptable ranges, I must crash")
    }

    fn update_now_raw(&self) -> i64 {
        let now = Timestamp::now().as_millisecond();
        self.bump_raw(now)
    }

    fn bump_raw(&self, other: i64) -> i64 {
        self.time.fetch_max(other, Ordering::AcqRel).max(other)
    }

    /// Ingest another timestamp
    ///
    /// This should be applied when reading logs or joining a cluster.
    pub fn update_from_other(&self, other: Timestamp) {
        self.bump_raw(other.as_millisecond());
    }

    fn as_i64(&self) -> i64 {
        self.time.load(Ordering::Acquire)
    }

    /// Advance time by the given amount. This should only be used in integration tests.
    #[cfg(debug_assertions)]
    pub fn fast_forward(&self, by: std::time::Duration) -> Timestamp {
        // TODO: it would be cool to only enable this for integration tests, even if the
        // integration tests run in release mode. Unfortunately, there isn't any feature
        // or build variable set when running integration tests, so we're doing this
        // debug_assertions shenanigans for now.
        let millis: i64 = by.as_millis().try_into().expect("duration out of bounds");
        tracing::trace!(millis, "fast-forwarding time");
        self.time.fetch_add(millis, Ordering::AcqRel);
        self.now()
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
        assert_eq!(mt.now(), Timestamp::UNIX_EPOCH);
        // calling .now returns the current time and bumps `last`
        let now = Timestamp::now();
        assert_approximately_equal(now, mt.update_now());
        assert_approximately_equal(now, mt.now());
        let future = now + Duration::from_mins(10);
        // bumping into the future prevents now from rewinding
        mt.update_from_other(future);
        assert_approximately_equal(mt.now(), future);
        assert_approximately_equal(mt.update_now(), future);
    }

    #[test]
    fn test_mockable_time() {
        let mt = Monotime::initial();
        assert_eq!(mt.now(), Timestamp::UNIX_EPOCH);
        mt.fast_forward(Duration::from_hours(1));
        assert_eq!(mt.now().as_second(), 3600);
        // we can still keep moving forward, e.g., with now()
        assert_approximately_equal(Timestamp::now(), mt.update_now());
        mt.fast_forward(Duration::from_hours(1));
        assert_approximately_equal(Timestamp::now() + Duration::from_hours(1), mt.update_now());
        assert_approximately_equal(Timestamp::now() + Duration::from_hours(1), mt.now());
        // running .now won't rewind us, monotonicity stands
        assert!(mt.update_now() > Timestamp::now());
    }
}
