package diom_models

// This file is @generated DO NOT EDIT

// Connection security for a Kafka sink. Every field is optional and maps 1:1 onto a librdkafka
// config key, so absent fields leave librdkafka at its defaults. Certificates and keys are inline
// PEMs so credentials live in the config rather than in per-node files.
type KafkaSecurity struct {
	SecurityProtocol                 *SecurityProtocol `msgpack:"security_protocol,omitempty"`
	SaslMechanism                    *SaslMechanism    `msgpack:"sasl_mechanism,omitempty"`
	SaslUsername                     *string           `msgpack:"sasl_username,omitempty"`
	SaslPassword                     *string           `msgpack:"sasl_password,omitempty"`       // Secret. Obfuscated in list responses.
	SslCaPem                         *string           `msgpack:"ssl_ca_pem,omitempty"`          // Inline CA certificate PEM. When absent, the system trust roots are used.
	SslCertificatePem                *string           `msgpack:"ssl_certificate_pem,omitempty"` // Inline client certificate PEM for mutual TLS.
	SslKeyPem                        *string           `msgpack:"ssl_key_pem,omitempty"`         // Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses.
	SslKeyPassword                   *string           `msgpack:"ssl_key_password,omitempty"`    // Password for an encrypted client key. Secret. Fully redacted in list responses.
	EnableSslCertificateVerification *bool             `msgpack:"enable_ssl_certificate_verification,omitempty"`
}
