use std::time::Duration;

use coyote_core::types::DurationMs;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenBucket {
    /// Token refill rate in tokens per refill interval
    pub refill_rate: u64,
    /// Token refill interval
    pub refill_interval: DurationMs,
    /// Max tokens allowed in the bucket
    pub bucket_size: u64,
}

impl TokenBucket {
    pub(crate) fn get_new_capacity(
        &self,
        current: u64,
        now: Timestamp,
        last_refill: Timestamp,
    ) -> (u64, Timestamp) {
        let mut capacity = current;
        let mut new_last_refill = last_refill;

        if last_refill < now {
            let elapsed_millis: u64 = now
                .duration_since(last_refill)
                .as_millis()
                .try_into()
                .unwrap();
            let refill_interval_millis: u64 = self.refill_interval.as_millis();
            let intervals = elapsed_millis / refill_interval_millis;

            capacity += intervals * self.refill_rate;
            capacity = capacity.min(self.bucket_size);

            new_last_refill += self.refill_interval.saturating_mul(intervals as u32);
        }

        (capacity, new_last_refill)
    }

    fn calculate_refill_duration(&self, current: u64, wanted: u64) -> Duration {
        let wanted = wanted.saturating_sub(current);
        let intervals = wanted.div_ceil(self.refill_rate);
        Duration::from_millis(intervals * self.refill_interval.as_millis())
    }

    pub(crate) fn calculate_retry_after(&self, current: u64, wanted: u64) -> Duration {
        self.calculate_refill_duration(current, wanted)
    }

    pub(crate) fn calculate_when_full(&self, current: u64) -> Duration {
        self.calculate_refill_duration(current, self.bucket_size)
    }
}
