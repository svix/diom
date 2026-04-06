# this file is @generated

from pydantic import BaseModel

from .rate_limit_config import RateLimitConfig


class RateLimitResetIn(BaseModel):
    namespace: str | None = None

    key: str

    config: RateLimitConfig
    """Rate limiter configuration"""
