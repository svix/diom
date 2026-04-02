# this file is @generated

from pydantic import BaseModel


class RateLimitGetRemainingOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: int | None = None
    """Milliseconds until at least one token is available (only present when remaining is 0)"""
