//! Feature-version compatibility advertised between cluster nodes.
//!
//! Each build supports a contiguous range of feature versions. Nodes advertise their range during
//! discovery and health checks so an incompatible node parks instead of joining a cluster it cannot
//! interoperate with.
use diom_operations::FeatureVersion;
use serde::{Deserialize, Serialize};

/// The range of feature versions this build can operate at.
pub(crate) const SUPPORTED_VERSION_RANGE: VersionRange = VersionRange { min: 0, max: 0 };

/// An inclusive range of feature versions a node supports.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionRange {
    min: FeatureVersion,
    pub(crate) max: FeatureVersion,
}

impl VersionRange {
    /// Construct a range spanning `min..=max`.
    pub fn new(min: FeatureVersion, max: FeatureVersion) -> Self {
        Self { min, max }
    }

    /// Whether a node with this range can operate at feature version `v`.
    pub(crate) fn includes(&self, v: FeatureVersion) -> bool {
        self.min <= v && v <= self.max
    }
}

impl std::fmt::Display for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..={}", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_feature_version_within_range() {
        let a = VersionRange { min: 0, max: 2 };
        assert!(a.includes(0));
        assert!(a.includes(2));
        assert!(!a.includes(3));
    }
}
