use diom_core::types::UnixTimestampMs;

/// Monotonically increasing version flag, as recognized by the node and cluster. Used for gating
/// the writing of certain data across nodes that may have a different version during an upgrade.
pub type FeatureVersion = u32;

/// Every feature version this codebase knows about, in increasing order. Code gates new behavior by
/// comparing against a named entry here (for example `VERSIONS[1]`) rather than a bare number, so the
/// set of versions lives in one place.
pub const VERSIONS: [FeatureVersion; 2] = [0, 1];

#[derive(Debug, Clone)]
pub struct OpContext {
    /// The (monotonic) timestamp at which this object was enqueued for application.
    pub timestamp: UnixTimestampMs,
    /// The Raft log index. This is monotonically-increasing with every commit.
    pub log_index: u64,
    /// The raft term. This is monotonically-increasing with every leadership change.
    pub term: u64,
    /// The cluster feature version in effect when this operation is applied. Read this for
    /// deterministic apply-time gating of new behavior.
    pub feature_version: FeatureVersion,
}
