# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .http_method import HttpMethod


class HttpSinkConfig(BaseModel):
    """Configuration for an HTTP sink. The `url`, `headers`, and `body` are templates rendered
    per-message (see [`diom_core::template_str`])."""

    url: str
    """Destination URL."""

    method: HttpMethod | None = None

    headers: t.Dict[str, str] | None = None

    body: str | None = None
    """Templated request body. When absent, the raw message value bytes are sent unchanged."""
