use std::sync::Arc;

use parking_lot::RwLock;

#[derive(Clone)]
pub struct OpContext {
    /// The (monotonic) timestamp at which this object was enqueued for application.
    pub timestamp: jiff::Timestamp,
    /// The Raft log index. This is monotonically-increasing with every commit.
    pub log_index: u64,
    /// The raft term. This is monotonically-increasing with every leadership change.
    pub term: u64,
    /// The write batch for the persistent db - FIXME to make less ugly.
    pub batch: Arc<RwLock<fjall::OwnedWriteBatch>>,
}

impl std::fmt::Debug for OpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpContext")
            .field("timestamp", &self.timestamp)
            .field("log_index", &self.log_index)
            .field("term", &self.term)
            .finish()
    }
}
