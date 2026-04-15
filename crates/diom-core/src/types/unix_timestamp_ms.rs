use std::{borrow::Cow, fmt};

use schemars::{JsonSchema, Schema, json_schema};
use serde::{Deserialize, Serialize};
use tap::Pipe;

use crate::types::DurationMs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimestampMs(u64);

impl UnixTimestampMs {
    pub const UNIX_EPOCH: Self = Self(0);
    pub const MIN: Self = Self(0);
    pub const MAX: Self = Self(253402207200998); // match jiff::Timestamp::MAX

    fn new(millis: u64) -> Result<Self, &'static str> {
        if millis > Self::MAX.0 {
            return Err("timestamp too large");
        }
        Ok(Self(millis))
    }

    /// Construct a new UnixTimestampMs from the given number of milliseconds since the Unix epoch
    ///
    /// Returns None if the input time is negative or extends beyond the representable size of a
    /// jiff::Timestamp.
    pub fn try_from_millisecond(millis: i64) -> Option<Self> {
        if millis < 0 {
            return None;
        }
        let millis = millis as u64;
        if millis > Self::MAX.0 {
            return None;
        }
        Some(Self(millis))
    }

    pub fn as_millisecond(self) -> u64 {
        self.0
    }

    pub fn saturating_mul(self, value: u64) -> Self {
        self.0
            .saturating_mul(value)
            .clamp(0, Self::MAX.0)
            .pipe(Self)
    }

    /// Returns the number of milliseconds between `other` and `now`, saturating at 0 if `other` is
    /// after now
    pub fn saturating_duration_since(&self, other: Self) -> DurationMs {
        DurationMs::from_millis(self.0.saturating_sub(other.0))
    }

    /// Returns the number of milliseconds between `other` and `now`, saturating at 0 if `other` is
    /// before now
    pub fn saturating_duration_until(&self, other: Self) -> DurationMs {
        DurationMs::from_millis(other.0.saturating_sub(self.0))
    }

    pub fn saturating_sub(&self, other: DurationMs) -> Self {
        self.0.saturating_sub(other.as_millis()).pipe(Self)
    }

    pub fn checked_add(&self, other: DurationMs) -> Option<Self> {
        let new = self.0.checked_add(other.as_millis())?;
        if new > Self::MAX.0 {
            return None;
        }
        Some(Self(new))
    }
}

impl fmt::Debug for UnixTimestampMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ts = jiff::Timestamp::from_millisecond(
            self.0
                .try_into()
                .expect("timestamps beyond year 9999 not supported"),
        );
        ts.fmt(f)
    }
}

impl fmt::Display for UnixTimestampMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        jiff::Timestamp::from(*self).fmt(f)
    }
}

impl From<jiff::Timestamp> for UnixTimestampMs {
    #[track_caller]
    fn from(value: jiff::Timestamp) -> Self {
        let value = u64::try_from(value.as_millisecond())
            .expect("timestamps in diom must never be before the unix epoch");
        Self(value)
    }
}

impl From<UnixTimestampMs> for jiff::Timestamp {
    #[track_caller]
    fn from(value: UnixTimestampMs) -> Self {
        Self::from_millisecond(value.0.try_into().expect("timestamps must be reasonable"))
            .expect("timestamps in diom must never be outside of ms range")
    }
}

impl Serialize for UnixTimestampMs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UnixTimestampMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Self::new(millis).map_err(serde::de::Error::custom)
    }
}

impl std::ops::Add<jiff::SignedDuration> for UnixTimestampMs {
    type Output = UnixTimestampMs;

    fn add(self, rhs: jiff::SignedDuration) -> Self::Output {
        (self.0 as i128)
            .checked_add(rhs.as_millis())
            .and_then(|u| u64::try_from(u).ok())
            .expect("result of addition must fit inside a UnixDurationMs")
            .pipe(Self)
    }
}

impl std::ops::Add<DurationMs> for UnixTimestampMs {
    type Output = Self;

    fn add(self, rhs: DurationMs) -> Self::Output {
        self.0
            .checked_add(rhs.as_millis())
            .expect("add result must fit inside a UnixDurationMs")
            .pipe(Self)
    }
}

impl std::ops::AddAssign<DurationMs> for UnixTimestampMs {
    fn add_assign(&mut self, rhs: DurationMs) {
        self.0 = self
            .0
            .checked_add(rhs.as_millis())
            .expect("add result must fit inside a UnixDurationMs")
    }
}

impl JsonSchema for UnixTimestampMs {
    fn schema_name() -> Cow<'static, str> {
        "UnixTimestampMs".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> Schema {
        json_schema!({
            "type": "integer",
            "format": "uint64",
            "minimum": 0,
            "x-subtype": "UnixTimestampMs",
        })
    }

    fn inline_schema() -> bool {
        true
    }
}
