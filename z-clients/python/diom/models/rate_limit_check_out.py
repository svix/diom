# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class RateLimitCheckOut(BaseModel):
    allowed: bool
    """Whether the request is allowed"""

    remaining: int
    """Number of tokens remaining"""

    retry_after: TimeDeltaMs | None = Field(alias="retry_after_ms", default=None)
    """Milliseconds until enough tokens are available (only present when allowed is false)"""
