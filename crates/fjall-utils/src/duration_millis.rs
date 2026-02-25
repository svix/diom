use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    (duration.as_millis() as u64).serialize(serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
    let millis = u64::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis))
}
