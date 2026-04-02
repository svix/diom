# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class RateLimitCheckOut(BaseModel):
    allowed: bool
    """Whether the request is allowed"""

    remaining: int
    """Number of tokens remaining"""

    retry_after_ms: t.Optional[int] = None
    """Milliseconds until enough tokens are available (only present when allowed is false)"""
