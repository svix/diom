// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EvictionPolicy {
    #[serde(rename = "no-eviction")]
    NoEviction,
}

impl fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::NoEviction => "no-eviction",
        };
        f.write_str(value)
    }
}
