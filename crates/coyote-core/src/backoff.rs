use std::time::Duration;

/// An implementation of "FullJitter" as described in https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/.
///
/// Exponential backoff is a common enough pattern that it's useful to have a standardized approach.
pub struct ExponentialBackoffWithJitter {
    dur: Duration,
    /// Lower bound for maximum backoff.
    ///
    /// The actual maximum backoff is double this number, because for each backoff step,
    /// we use a random delay between `self.dur` and `2 * self.dur`.
    max_backoff_lower_bound: Duration,
    initial: Duration,
}

impl ExponentialBackoffWithJitter {
    pub fn new(initial_dur: Duration, max_backoff_lower_bound: Duration) -> Self {
        Self {
            dur: initial_dur,
            initial: initial_dur,
            max_backoff_lower_bound,
        }
    }

    pub async fn backoff(&mut self) {
        let delay = self.next_delay();
        tokio::time::sleep(delay).await;
    }

    pub fn next_delay(&mut self) -> Duration {
        self.dur = self.max_backoff_lower_bound.min(2 * self.dur);

        let range = self.dur..self.dur * 2;
        jitter(range)
    }

    pub fn reset(&mut self, initial: Duration) {
        self.dur = initial;
    }

    pub fn reset_to_initial(&mut self) {
        self.dur = self.initial;
    }
}

pub fn jitter(r: std::ops::Range<Duration>) -> Duration {
    if r.start >= r.end {
        tracing::error!("Mistakenly used invalid range for jitter");
        Duration::from_secs(0)
    } else {
        rand::random_range(r)
    }
}
