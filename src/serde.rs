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

    /// Deserialize an Optional<Duration> as a number of milliseconds
    pub(crate) mod opt_ms {
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::time::Duration;

        pub(crate) fn serialize<S>(x: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let raw = x.map(|x| x.as_millis());
            raw.serialize(serializer)
        }

        pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let millis: Option<u64> = Deserialize::deserialize(deserializer)?;
            Ok(millis.map(Duration::from_millis))
        }
    }
}
