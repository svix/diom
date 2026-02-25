use std::{num::NonZeroU64, time::Duration};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StreamConfig {
    #[serde(with = "fjall_utils::duration_millis")]
    pub retention_period: Duration,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub struct Retention {
    #[serde(default = "default_retention_millis")]
    pub millis: NonZeroU64,
    #[serde(default = "default_retention_bytes")]
    pub bytes: NonZeroU64,
}

pub fn default_retention_millis() -> NonZeroU64 {
    (Duration::from_hours(24 * 30).as_millis() as u64)
        .try_into()
        .unwrap()
}

fn default_retention_bytes() -> NonZeroU64 {
    NonZeroU64::new(1_000_000_000_000).expect("constant is non-zero")
}
