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

    /// Build a copy of this range with either bound replaced by its override when set.
    pub(crate) fn with_overrides(
        &self,
        min: Option<FeatureVersion>,
        max: Option<FeatureVersion>,
    ) -> Self {
        Self {
            min: min.unwrap_or(self.min),
            max: max.unwrap_or(self.max),
        }
    }

    /// Validate this range as an operator override of the build's `compiled` supported range.
    ///
    /// The override may only narrow the range (never advertise a version the binary cannot run) and
    /// may hold the max back by at most one version. The max floor keeps a held node at or above the
    /// committed cluster version (which in a normal rollout is at most one below the compiled max),
    /// so the node does not shut itself down on restart.
    pub(crate) fn validate_as_override_of(&self, compiled: &Self) -> Result<(), &'static str> {
        if self.min > self.max {
            return Err("supported feature version min must be <= max");
        }
        if self.min < compiled.min || self.max > compiled.max {
            return Err("supported feature versions must be a subset of the compiled-in range");
        }
        if self.max < compiled.max.saturating_sub(1) {
            return Err("supported feature version max must be at most one below the compiled-in max");
        }
        Ok(())
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

    #[test]
    fn validate_as_override_of_enforces_subset_and_max_floor() {
        let compiled = VersionRange { min: 3, max: 10 };
        let ok = |min, max| VersionRange { min, max }.validate_as_override_of(&compiled).is_ok();

        // Matching the compiled range, holding the max back by one, and raising the min are allowed.
        assert!(ok(3, 10));
        assert!(ok(3, 9));
        assert!(ok(4, 10));

        // Widening either bound would advertise versions the build cannot run.
        assert!(!ok(2, 10));
        assert!(!ok(3, 11));
        // Holding the max back by more than one risks dropping below the committed version.
        assert!(!ok(3, 8));
        // An inverted range is nonsensical.
        assert!(!ok(5, 4));
    }

    #[test]
    fn with_overrides_falls_back_to_each_bound_when_unset() {
        let compiled = VersionRange { min: 3, max: 10 };

        assert_eq!(compiled.with_overrides(None, None), compiled);
        assert_eq!(compiled.with_overrides(None, Some(9)), VersionRange { min: 3, max: 9 });
        assert_eq!(compiled.with_overrides(Some(5), None), VersionRange { min: 5, max: 10 });
        assert_eq!(compiled.with_overrides(Some(4), Some(8)), VersionRange { min: 4, max: 8 });
    }
}
