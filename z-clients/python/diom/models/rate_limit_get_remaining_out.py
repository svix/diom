# this file is @generated
import typing as t

from pydantic import BaseModel


class RateLimitGetRemainingOut(BaseModel):
    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: t.Optional[int] = None
    """Milliseconds until at least one token is available (only present when remaining is 0)"""
