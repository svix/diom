# this file is @generated

from ..internal.base_model import BaseModel

from .rate_limit_config import RateLimitConfig


class RateLimitCheckIn(BaseModel):
    namespace: str | None = None

    key: str

    tokens: int | None = None
    """Number of tokens to consume (default: 1)"""

    config: RateLimitConfig
    """Rate limiter configuration"""
