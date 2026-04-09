# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class RateLimitGetRemainingOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after: TimeDeltaMs | None = Field(alias="retry_after_ms", default=None)
    """Milliseconds until at least one token is available (only present when remaining is 0)"""
