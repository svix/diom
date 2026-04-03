# this file is @generated

from pydantic import BaseModel

from .rate_limit_config import RateLimitConfig


class RateLimitGetRemainingIn(BaseModel):
    namespace: str | None = None

    key: str

    config: RateLimitConfig
    """Rate limiter configuration"""
