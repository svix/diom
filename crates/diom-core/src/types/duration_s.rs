use std::{borrow::Cow, fmt, ops::Add, time::Duration};

use schemars::{JsonSchema, Schema, json_schema};
use serde::{Deserialize, Serialize};
use validator::ValidateRange;

/// Duration as an unsigned 64-bit integer in second precision.
///
/// Serialized just like the underlying integer representation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurationS(u64);

impl From<DurationS> for Duration {
    fn from(value: DurationS) -> Self {
        Duration::from_secs(value.0)
    }
}

impl From<u64> for DurationS {
    fn from(millis: u64) -> Self {
        Self(millis)
    }
}

impl fmt::Debug for DurationS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Add<DurationS> for jiff::Timestamp {
    type Output = jiff::Timestamp;

    #[inline]
    fn add(self, rhs: DurationS) -> Self::Output {
        self + Duration::from(rhs)
    }
}

impl Serialize for DurationS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DurationS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Self::from(millis))
    }
}

impl JsonSchema for DurationS {
    fn schema_name() -> Cow<'static, str> {
        "DurationS".into()
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

impl ValidateRange<u64> for DurationS {
    fn greater_than(&self, max: u64) -> Option<bool> {
        Some(*self > Self::from(max))
    }

    fn less_than(&self, min: u64) -> Option<bool> {
        Some(*self < Self::from(min))
    }
}
