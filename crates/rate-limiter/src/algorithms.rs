use std::time::Duration;

use jiff::Timestamp;

pub struct FixedWindow {
    /// Window size
    pub size: Duration,
    /// Max tokens allowed per window
    pub tokens: u64,
}

impl FixedWindow {
    pub(crate) fn get_window_start(&self, now: Timestamp) -> Timestamp {
        let size_ms = self.size.as_millis() as i64;
        let now_ms = now.as_millisecond();
        let window_start_ms = now_ms - (now_ms % size_ms);
        Timestamp::from_millisecond(window_start_ms).unwrap()
    }
}

pub struct TokenBucket {
    /// Token refill rate in tokens per refill interval
    pub refill_rate: u64,
    /// Token refill interval
    pub refill_interval: Duration,
    /// Max tokens allowed in the bucket
    pub bucket_size: u64,
}

impl TokenBucket {
    pub(crate) fn get_new_capacity(
        &self,
        current: u64,
        now: Timestamp,
        last_refill: Timestamp,
    ) -> u64 {
        let mut capacity = current;
        if last_refill < now {
            let elapsed_millis: u64 = now
                .duration_since(last_refill)
                .as_millis()
                .try_into()
                .unwrap();
            let refill_interval_millis: u64 = self.refill_interval.as_millis().try_into().unwrap();

            capacity += (elapsed_millis / refill_interval_millis) * self.refill_rate;
        }
        capacity.min(self.bucket_size)
    }
}

pub enum RateLimitConfig {
    FixedWindow(FixedWindow),
    TokenBucket(TokenBucket),
}
