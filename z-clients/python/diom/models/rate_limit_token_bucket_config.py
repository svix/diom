# this file is @generated
import typing as t

from pydantic import BaseModel


class RateLimitTokenBucketConfig(BaseModel):
    capacity: int
    """Maximum capacity of the bucket"""

    refill_amount: int
    """Number of tokens to add per refill interval"""

    refill_interval_ms: t.Optional[int] = None
    """Interval in milliseconds between refills (minimum 1 millisecond)"""
