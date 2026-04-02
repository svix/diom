# this file is @generated

from pydantic import BaseModel

from .rate_limit_token_bucket_config import RateLimitTokenBucketConfig


class RateLimitCheckIn(BaseModel):
    namespace: str | None = None

    key: str

    tokens: int | None = None
    """Number of tokens to consume (default: 1)"""

    config: RateLimitTokenBucketConfig
    """Rate limiter configuration"""
