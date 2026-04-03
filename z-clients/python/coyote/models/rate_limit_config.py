# this file is @generated

from pydantic import BaseModel


class RateLimitConfig(BaseModel):
    capacity: int
    """Maximum capacity of the bucket"""

    refill_amount: int
    """Number of tokens to add per refill interval"""

    refill_interval_ms: int | None = None
    """Interval in milliseconds between refills (minimum 1 millisecond)"""
