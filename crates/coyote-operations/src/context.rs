#[derive(Debug, Clone)]
pub struct OpContext {
    /// The (monotonic) timestamp at which this object was enqueued for application.
    pub timestamp: jiff::Timestamp,
    /// The Raft log index. This is monotonically-increasing with every commit.
    pub log_index: u64,
    /// The raft term. This is monotonically-increasing with every leadership change.
    pub term: u64,
}
