// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// The SASL mechanism, mapped onto librdkafka's `sasl.mechanism`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SaslMechanism {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "scram-sha256")]
    ScramSha256,
    #[serde(rename = "scram-sha512")]
    ScramSha512,
}

impl fmt::Display for SaslMechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Plain => "plain",
            Self::ScramSha256 => "scram-sha256",
            Self::ScramSha512 => "scram-sha512",
        };
        f.write_str(value)
    }
}
