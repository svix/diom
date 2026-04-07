use std::{borrow::Cow, fmt};

use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnixTimestampMs(u64);

impl UnixTimestampMs {
    fn new(millis: u64) -> Result<Self, &'static str> {
        if millis > jiff::Timestamp::MAX.as_millisecond() as u64 {
            return Err("timestamp too large");
        }
        Ok(Self(millis))
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

impl From<jiff::Timestamp> for UnixTimestampMs {
    #[track_caller]
    fn from(value: jiff::Timestamp) -> Self {
        let value = u64::try_from(value.as_millisecond())
            .expect("timestamps in coyote must never be before the unix epoch");
        Self(value)
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

impl JsonSchema for UnixTimestampMs {
    fn schema_name() -> Cow<'static, str> {
        "UnixTimestampMs".into()
    }

    fn json_schema(g: &mut schemars::SchemaGenerator) -> Schema {
        u64::json_schema(g)
    }

    fn inline_schema() -> bool {
        true
    }
}
