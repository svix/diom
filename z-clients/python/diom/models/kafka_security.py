# this file is @generated

from ..internal.base_model import BaseModel

from .sasl_mechanism import SaslMechanism
from .security_protocol import SecurityProtocol


class KafkaSecurity(BaseModel):
    """Connection security for a Kafka sink. Every field is optional and maps 1:1 onto a librdkafka
    config key, so absent fields leave librdkafka at its defaults. Certificates and keys are inline
    PEMs so credentials live in the config rather than in per-node files."""

    security_protocol: SecurityProtocol | None = None

    sasl_mechanism: SaslMechanism | None = None

    sasl_username: str | None = None

    sasl_password: str | None = None
    """Secret. Obfuscated in list responses."""

    ssl_ca_pem: str | None = None
    """Inline CA certificate PEM. When absent, the system trust roots are used."""

    ssl_certificate_pem: str | None = None
    """Inline client certificate PEM for mutual TLS."""

    ssl_key_pem: str | None = None
    """Inline client key PEM for mutual TLS. Secret. Fully redacted in list responses."""

    ssl_key_password: str | None = None
    """Password for an encrypted client key. Secret. Fully redacted in list responses."""

    enable_ssl_certificate_verification: bool | None = None
