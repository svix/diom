// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// The connection security protocol, mapped onto librdkafka's `security.protocol`.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SecurityProtocol {
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "ssl")]
    Ssl,
    #[serde(rename = "sasl-plaintext")]
    SaslPlaintext,
    #[serde(rename = "sasl-ssl")]
    SaslSsl,
}

impl fmt::Display for SecurityProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Plaintext => "plaintext",
            Self::Ssl => "ssl",
            Self::SaslPlaintext => "sasl-plaintext",
            Self::SaslSsl => "sasl-ssl",
        };
        f.write_str(value)
    }
}
