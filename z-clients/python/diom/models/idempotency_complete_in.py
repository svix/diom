# this file is @generated

from pydantic import BaseModel


class IdempotencyCompleteIn(BaseModel):
    namespace: str | None = None

    response: bytes
    """The response to cache"""

    ttl_ms: int
    """How long to keep the idempotency response for."""


class _IdempotencyCompleteIn(BaseModel):
    namespace: str | None = None

    key: str

    response: bytes
    """The response to cache"""

    ttl_ms: int
    """How long to keep the idempotency response for."""
