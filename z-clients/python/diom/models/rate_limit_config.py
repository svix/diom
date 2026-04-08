# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class RateLimitConfig(BaseModel):
    capacity: int
    """Maximum capacity of the bucket"""

    refill_amount: int
    """Number of tokens to add per refill interval"""

    refill_interval: TimeDeltaMs | None = Field(
        alias="refill_interval_ms", default=None
    )
    """Interval in milliseconds between refills (minimum 1 millisecond)"""
