# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .kafka_security import KafkaSecurity


class KafkaSinkConfig(BaseModel):
    """Configuration for a Kafka sink. Each message is produced to `topic` on the target cluster. By
    default the message value and headers pass through unchanged, but each can be templated
    per-message (see [`diom_core::template_str`])."""

    bootstrap_servers: str
    """Comma-separated `host:port` list of the target cluster's bootstrap brokers."""

    topic: str
    """Destination Kafka topic."""

    key: str | None = None
    """Templated record key rendered per-message. When absent, records are produced without a key."""

    value: str | None = None
    """Templated record value. When absent, the raw message value bytes are produced unchanged."""

    headers: t.Dict[str, str] | None = None
    """Templated record headers merged on top of the message's own headers (which pass through by
    default). A templated header overrides a passed-through one with the same name."""

    security: KafkaSecurity | None = None
    """Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT)."""
