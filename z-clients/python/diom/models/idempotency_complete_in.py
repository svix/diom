# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class IdempotencyCompleteIn(BaseModel):
    namespace: str | None = None

    response: bytes
    """The response to cache"""

    context: t.Dict[str, str] | None = None
    """Optional metadata to store alongside the response"""

    ttl: TimeDeltaMs = Field(alias="ttl_ms")
    """How long to keep the idempotency response for."""


class _IdempotencyCompleteIn(BaseModel):
    namespace: str | None = None

    key: str

    response: bytes
    """The response to cache"""

    context: t.Dict[str, str] | None = None
    """Optional metadata to store alongside the response"""

    ttl: TimeDeltaMs = Field(alias="ttl_ms")
    """How long to keep the idempotency response for."""
