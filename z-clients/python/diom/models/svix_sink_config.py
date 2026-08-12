# this file is @generated

from ..internal.base_model import BaseModel


class SvixSinkConfig(BaseModel):
    """Configuration for a Svix sink. Each message is forwarded as a Svix message-create call
    (`POST {server_url}/api/v1/app/{app_id}/msg/`). This is a thin convenience over an HTTP sink.
    The `app_id`, `event_type`, and `payload` are templates rendered per-message
    (see [`diom_core::template_str`])."""

    token: str
    """Svix API token, sent as the bearer credential. Obfuscated in list responses."""

    app_id: str
    """Target Svix application. Can be optionally templated."""

    event_type: str
    """Svix event type. Can be optionally templated."""

    payload: str | None = None
    """Templated message payload. When absent, the raw message value bytes are used (must be JSON)."""

    idempotency_key: str | None = None
    """Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
    key derived from the sink and message identity (namespace, topic, consumer_group, partition,
    offset) is used so retries are de-duplicated by Svix."""

    server_url: str | None = None
    """Optional base URL override. When absent, the region is inferred from the token."""
