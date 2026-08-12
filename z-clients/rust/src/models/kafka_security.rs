// this file is @generated
use serde::{Deserialize, Serialize};

use super::{sasl_mechanism::SaslMechanism, security_protocol::SecurityProtocol};

/// Connection security for a Kafka sink. Every field is optional and maps 1:1 onto a librdkafka
/// config key, so absent fields leave librdkafka at its defaults. Certificates and keys are inline
/// PEMs so credentials live in the config rather than in per-node files.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KafkaSecurity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_protocol: Option<SecurityProtocol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_mechanism: Option<SaslMechanism>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_username: Option<String>,

    /// Secret. Obfuscated in list responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_password: Option<String>,

    /// Inline CA certificate PEM. When absent, the system trust roots are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca_pem: Option<String>,

    /// Inline client certificate PEM for mutual TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_pem: Option<String>,

    /// Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_key_pem: Option<String>,

    /// Password for an encrypted client key. Secret. Fully redacted in list responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_key_password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_certificate_verification: Option<bool>,
}

impl KafkaSecurity {
    pub fn new() -> Self {
        Self {
            security_protocol: None,
            sasl_mechanism: None,
            sasl_username: None,
            sasl_password: None,
            ssl_ca_pem: None,
            ssl_certificate_pem: None,
            ssl_key_pem: None,
            ssl_key_password: None,
            enable_ssl_certificate_verification: None,
        }
    }

    pub fn with_security_protocol(mut self, value: impl Into<Option<SecurityProtocol>>) -> Self {
        self.security_protocol = value.into();
        self
    }

    pub fn with_sasl_mechanism(mut self, value: impl Into<Option<SaslMechanism>>) -> Self {
        self.sasl_mechanism = value.into();
        self
    }

    pub fn with_sasl_username(mut self, value: impl Into<Option<String>>) -> Self {
        self.sasl_username = value.into();
        self
    }

    pub fn with_sasl_password(mut self, value: impl Into<Option<String>>) -> Self {
        self.sasl_password = value.into();
        self
    }

    pub fn with_ssl_ca_pem(mut self, value: impl Into<Option<String>>) -> Self {
        self.ssl_ca_pem = value.into();
        self
    }

    pub fn with_ssl_certificate_pem(mut self, value: impl Into<Option<String>>) -> Self {
        self.ssl_certificate_pem = value.into();
        self
    }

    pub fn with_ssl_key_pem(mut self, value: impl Into<Option<String>>) -> Self {
        self.ssl_key_pem = value.into();
        self
    }

    pub fn with_ssl_key_password(mut self, value: impl Into<Option<String>>) -> Self {
        self.ssl_key_password = value.into();
        self
    }

    pub fn with_enable_ssl_certificate_verification(
        mut self,
        value: impl Into<Option<bool>>,
    ) -> Self {
        self.enable_ssl_certificate_verification = value.into();
        self
    }
}
