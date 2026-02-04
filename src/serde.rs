#![allow(unused)]

pub(crate) mod duration {
    /// Deserialize a Duration as a number of milliseconds
    pub(crate) mod millis {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::time::Duration;

        pub(crate) fn serialize<S>(x: &Duration, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let raw = x.as_millis();
            raw.serialize(serializer)
        }

        pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
        where
            D: Deserializer<'de>,
        {
            let millis: u64 = Deserialize::deserialize(deserializer)?;
            Ok(Duration::from_millis(millis))
        }
    }
}
