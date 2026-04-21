use std::{borrow::Cow, fmt, num::NonZeroU64, time::Duration};

use schemars::{JsonSchema, Schema, json_schema};
use serde::{Deserialize, Serialize};
use tap::Pipe;
use validator::ValidateRange;

use super::DurationMs;

/// Non-zero variation of [`DurationMs`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonZeroDurationMs(NonZeroU64);

impl NonZeroDurationMs {
    /// Construct a NonZeroDurationMs from the given integer number of seconds.
    ///
    /// Returns None if the input is 0; panics if the input overflows a u64
    #[track_caller]
    pub const fn from_secs(secs: u64) -> Option<Self> {
        if let Some(value) = NonZeroU64::new(secs.checked_mul(1000).expect("integer overflow")) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Construct a NonZeroDurationMs from the given integer number of minutes.
    ///
    /// Returns None if the input is 0; panics if the input overflows a u64
    #[track_caller]
    pub const fn from_mins(secs: u64) -> Option<Self> {
        if let Some(value) = NonZeroU64::new(secs.checked_mul(60_000).expect("integer overflow")) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Construct a NonZeroDurationMs from the given integer number of milliseconds.
    ///
    /// Returns None if the input is 0
    #[track_caller]
    pub const fn from_millis(millis: u64) -> Option<Self> {
        if let Some(value) = NonZeroU64::new(millis) {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn as_millis(&self) -> u64 {
        self.0.get()
    }

    #[inline(always)]
    pub const fn get(self) -> DurationMs {
        DurationMs::from_millis(self.as_millis())
    }

    pub const fn as_duration(&self) -> Duration {
        Duration::from_millis(self.as_millis())
    }
}

impl From<NonZeroDurationMs> for Duration {
    fn from(value: NonZeroDurationMs) -> Self {
        value.as_duration()
    }
}

impl From<NonZeroDurationMs> for DurationMs {
    fn from(value: NonZeroDurationMs) -> Self {
        Self::from_millis(value.as_millis())
    }
}

#[derive(Debug, Clone)]
pub enum NonZeroDurationMsParseErr {
    Zero,
    ParseIntError(std::num::ParseIntError),
}

impl fmt::Display for NonZeroDurationMsParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "value was unexpectedly zero"),
            Self::ParseIntError(e) => write!(f, "error parsing value: {e}"),
        }
    }
}

impl std::error::Error for NonZeroDurationMsParseErr {
    fn description(&self) -> &str {
        match self {
            Self::Zero => "value was unexpectedly zero",
            Self::ParseIntError(_) => "error parsing value as integer number of millis",
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Zero => None,
            Self::ParseIntError(e) => Some(e),
        }
    }
}

impl std::str::FromStr for NonZeroDurationMs {
    type Err = NonZeroDurationMsParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map_err(NonZeroDurationMsParseErr::ParseIntError)?
            .pipe(NonZeroU64::new)
            .ok_or(NonZeroDurationMsParseErr::Zero)
            .map(Self)
    }
}

impl From<NonZeroU64> for NonZeroDurationMs {
    /// Assume the given value represents an integer number of milliseconds
    /// and treat it as a `NonZeroDurationMs`.
    #[inline]
    fn from(millis: NonZeroU64) -> Self {
        Self(millis)
    }
}

impl TryFrom<DurationMs> for NonZeroDurationMs {
    type Error = NonZeroDurationMsParseErr;

    fn try_from(value: DurationMs) -> Result<Self, Self::Error> {
        Self::from_millis(value.as_millis()).ok_or(NonZeroDurationMsParseErr::Zero)
    }
}

impl fmt::Debug for NonZeroDurationMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for NonZeroDurationMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ValidateRange<u64> for NonZeroDurationMs {
    fn greater_than(&self, max: u64) -> Option<bool> {
        let max = Self::from_millis(max)?;
        Some(*self > max)
    }

    fn less_than(&self, min: u64) -> Option<bool> {
        let min = Self::from_millis(min)?;
        Some(*self < min)
    }
}

impl Serialize for NonZeroDurationMs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NonZeroDurationMs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let millis = NonZeroU64::deserialize(deserializer)?;
        Ok(Self::from(millis))
    }
}

impl JsonSchema for NonZeroDurationMs {
    fn schema_name() -> Cow<'static, str> {
        "NonZeroDurationMs".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> Schema {
        json_schema!({
            "type": "integer",
            "format": "uint64",
            "minimum": 1,
            "x-subtype": "DurationMs",
        })
    }

    fn inline_schema() -> bool {
        true
    }
}
