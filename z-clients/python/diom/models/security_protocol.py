# this file is @generated
from enum import Enum


class SecurityProtocol(str, Enum):
    """The connection security protocol, mapped onto librdkafka's `security.protocol`."""

    PLAINTEXT = "plaintext"
    SSL = "ssl"
    SASL_PLAINTEXT = "sasl-plaintext"
    SASL_SSL = "sasl-ssl"

    def __str__(self) -> str:
        return str(self.value)
