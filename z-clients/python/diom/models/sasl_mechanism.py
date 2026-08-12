# this file is @generated
from enum import Enum


class SaslMechanism(str, Enum):
    """The SASL mechanism, mapped onto librdkafka's `sasl.mechanism`."""

    PLAIN = "plain"
    SCRAM_SHA256 = "scram-sha256"
    SCRAM_SHA512 = "scram-sha512"

    def __str__(self) -> str:
        return str(self.value)
