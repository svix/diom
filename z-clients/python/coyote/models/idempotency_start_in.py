# this file is @generated

from pydantic import BaseModel


class IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    lock_period_ms: int
    """How long to hold the lock on start before releasing it."""


class _IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    key: str

    lock_period_ms: int
    """How long to hold the lock on start before releasing it."""
