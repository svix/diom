// this file is @generated
import {
    type SaslMechanism,
    SaslMechanismSerializer,
} from './saslMechanism';
import {
    type SecurityProtocol,
    SecurityProtocolSerializer,
} from './securityProtocol';
/**
* Connection security for a Kafka sink. Every field is optional and maps 1:1 onto a librdkafka
* config key, so absent fields leave librdkafka at its defaults. Certificates and keys are inline
* PEMs so credentials live in the config rather than in per-node files.
*/
export interface KafkaSecurity {
    securityProtocol?: SecurityProtocol | null;
    saslMechanism?: SaslMechanism | null;
    saslUsername?: string | null;
    /** Secret. Obfuscated in list responses. */
    saslPassword?: string | null;
    /** Inline CA certificate PEM. When absent, the system trust roots are used. */
    sslCaPem?: string | null;
    /** Inline client certificate PEM for mutual TLS. */
    sslCertificatePem?: string | null;
    /** Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses. */
    sslKeyPem?: string | null;
    /** Password for an encrypted client key. Secret. Fully redacted in list responses. */
    sslKeyPassword?: string | null;
    enableSslCertificateVerification?: boolean | null;
}

export const KafkaSecuritySerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KafkaSecurity {
        return {
            securityProtocol: object['security_protocol'] != null ? SecurityProtocolSerializer._fromJsonObject(object['security_protocol']): undefined,
            saslMechanism: object['sasl_mechanism'] != null ? SaslMechanismSerializer._fromJsonObject(object['sasl_mechanism']): undefined,
            saslUsername: object['sasl_username'],
            saslPassword: object['sasl_password'],
            sslCaPem: object['ssl_ca_pem'],
            sslCertificatePem: object['ssl_certificate_pem'],
            sslKeyPem: object['ssl_key_pem'],
            sslKeyPassword: object['ssl_key_password'],
            enableSslCertificateVerification: object['enable_ssl_certificate_verification'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KafkaSecurity): any {
        return {
            'security_protocol': self.securityProtocol != null ? SecurityProtocolSerializer._toJsonObject(self.securityProtocol) : undefined,
            'sasl_mechanism': self.saslMechanism != null ? SaslMechanismSerializer._toJsonObject(self.saslMechanism) : undefined,
            'sasl_username': self.saslUsername,
            'sasl_password': self.saslPassword,
            'ssl_ca_pem': self.sslCaPem,
            'ssl_certificate_pem': self.sslCertificatePem,
            'ssl_key_pem': self.sslKeyPem,
            'ssl_key_password': self.sslKeyPassword,
            'enable_ssl_certificate_verification': self.enableSslCertificateVerification,
        };
    }
}