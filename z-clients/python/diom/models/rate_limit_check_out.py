# this file is @generated

from pydantic import BaseModel


class RateLimitCheckOut(BaseModel):
    allowed: bool
    """Whether the request is allowed"""

    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: int | None = None
    """Milliseconds until enough tokens are available (only present when allowed is false)"""
