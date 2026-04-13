use jiff::Timestamp;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) fn serialize<S: Serializer>(
    duration: &Timestamp,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    duration.as_millisecond().serialize(serializer)
}

pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Timestamp, D::Error> {
    let millis = i64::deserialize(deserializer)?;
    Timestamp::from_millisecond(millis).map_err(serde::de::Error::custom)
}

pub(crate) mod optional {
    use jiff::Timestamp;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S: Serializer>(
        duration: &Option<Timestamp>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        duration.map(|d| d.as_millisecond()).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Timestamp>, D::Error> {
        let millis = Option::<i64>::deserialize(deserializer)?;
        millis
            .map(Timestamp::from_millisecond)
            .transpose()
            .map_err(serde::de::Error::custom)
    }
}
