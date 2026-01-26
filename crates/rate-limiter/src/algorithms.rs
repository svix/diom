use chrono::{DateTime, Duration, Utc};

pub struct FixedWindow {
    /// Window size
    pub size: Duration,
    /// Max tokens allowed per window
    pub tokens: u64,
}

impl FixedWindow {
    pub(crate) fn get_window_start(&self, now: DateTime<Utc>) -> DateTime<Utc> {
        let size_ms = self.size.num_milliseconds();
        let now_ms = now.timestamp_millis();
        let window_start_ms = now_ms - (now_ms % size_ms);
        DateTime::from_timestamp_millis(window_start_ms).unwrap()
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
        now: DateTime<Utc>,
        last_refill: DateTime<Utc>,
    ) -> u64 {
        let mut capacity = current;
        if last_refill < now {
            let elapsed = now - last_refill;
            let periods = elapsed.num_milliseconds() / self.refill_interval.num_milliseconds();
            capacity += periods as u64 * self.refill_rate;
        }
        capacity.min(self.bucket_size)
    }
}

pub enum RateLimitConfig {
    FixedWindow(FixedWindow),
    TokenBucket(TokenBucket),
}
