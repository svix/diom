# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    lock_period: TimeDeltaMs = Field(alias="lock_period_ms")
    """How long to hold the lock on start before releasing it."""


class _IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    key: str

    lock_period: TimeDeltaMs = Field(alias="lock_period_ms")
    """How long to hold the lock on start before releasing it."""
