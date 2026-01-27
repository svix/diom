// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RateLimitResult {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "BLOCK")]
    Block,
}

impl fmt::Display for RateLimitResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Ok => "OK",
            Self::Block => "BLOCK",
        };
        f.write_str(value)
    }
}
