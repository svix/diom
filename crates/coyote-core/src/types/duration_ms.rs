use std::{
    borrow::Cow,
    fmt,
    ops::{Add, AddAssign, Mul},
    time::Duration,
};

use schemars::{JsonSchema, Schema, json_schema};
use serde::{Deserialize, Serialize};
use validator::ValidateRange;

/// Duration as an unsigned 64-bit integer in millisecond precision.
///
/// Serialized just like the underlying integer representation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurationMs(u64);

impl DurationMs {
    /// Create a duration from the given number of seconds.
    ///
    /// # Panics
    ///
    /// Panics if `secs * 1000` overflows the `u64` range.
    #[track_caller]
    pub fn from_secs(secs: u64) -> Self {
        Self(secs.checked_mul(1000).expect("integer overflow"))
    }

    /// Create a duration from the given number of minutes.
    ///
    /// # Panics
    ///
    /// Panics if `mins * 60_000` overflows the `u64` range.
    #[track_caller]
    pub fn from_mins(mins: u64) -> Self {
        Self(mins.checked_mul(60_000).expect("integer overflow"))
    }

    /// Create a duration from the given number of hours.
    ///
    /// # Panics
    ///
    /// Panics if `hours * 3_600_000` overflows the `u64` range.
    #[track_caller]
    pub fn from_hours(hours: u64) -> Self {
        Self(hours.checked_mul(3_600_000).expect("integer overflow"))
    }

    pub fn as_millis(self) -> u64 {
        self.0
    }

    pub fn saturating_mul(&self, rhs: u32) -> Self {
        Self(self.0.saturating_mul(rhs.into()))
    }
}

impl From<DurationMs> for Duration {
    fn from(value: DurationMs) -> Self {
        Duration::from_millis(value.0)
    }
}

impl From<u64> for DurationMs {
    #[inline]
    fn from(millis: u64) -> Self {
        Self(millis)
    }
}

impl From<DurationMs> for jiff::TimestampArithmetic {
    fn from(value: DurationMs) -> Self {
        Duration::from(value).into()
    }
}

impl fmt::Debug for DurationMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add<DurationMs> for jiff::Timestamp {
    type Output = jiff::Timestamp;

    #[inline]
    fn add(self, rhs: DurationMs) -> Self::Output {
        self + Duration::from(rhs)
    }
}

impl AddAssign<DurationMs> for jiff::Timestamp {
    #[inline]
    fn add_assign(&mut self, rhs: DurationMs) {
        *self += Duration::from(rhs);
    }
}

impl Mul<DurationMs> for u64 {
    type Output = DurationMs;

    fn mul(self, rhs: DurationMs) -> DurationMs {
        DurationMs(self * rhs.0)
    }
}

impl Mul<u64> for DurationMs {
    type Output = DurationMs;

    fn mul(self, rhs: u64) -> DurationMs {
        DurationMs(self.0 * rhs)
    }
}

impl Serialize for DurationMs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DurationMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Self::from(millis))
    }
}

impl JsonSchema for DurationMs {
    fn schema_name() -> Cow<'static, str> {
        "DurationMs".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> Schema {
        json_schema!({
            "type": "integer",
            "format": "uint64",
        })
    }

    fn inline_schema() -> bool {
        true
    }
}

impl ValidateRange<u64> for DurationMs {
    fn greater_than(&self, max: u64) -> Option<bool> {
        Some(*self > Self::from(max))
    }

    fn less_than(&self, min: u64) -> Option<bool> {
        Some(*self < Self::from(min))
    }
}
