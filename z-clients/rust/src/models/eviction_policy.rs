// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EvictionPolicy {
    #[serde(rename = "NoEviction")]
    NoEviction,
}

impl fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::NoEviction => "NoEviction",
        };
        f.write_str(value)
    }
}
