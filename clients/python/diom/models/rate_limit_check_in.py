# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class RateLimitCheckIn(BaseModel):
    key: str

    tokens: t.Optional[int] = None
    """Number of tokens to consume (default: 1)"""

    capacity: int
    """Maximum capacity of the bucket"""

    refill_amount: int = Field(alias="refill_amount")
    """Number of tokens to add per refill interval"""

    refill_interval_millis: t.Optional[int] = Field(
        default=None, alias="refill_interval_millis"
    )
    """Interval in milliseconds between refills (minimum 1 millisecond)"""
