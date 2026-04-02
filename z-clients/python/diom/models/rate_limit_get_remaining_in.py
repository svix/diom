# this file is @generated

from pydantic import BaseModel

from .rate_limit_token_bucket_config import RateLimitTokenBucketConfig


class RateLimitGetRemainingIn(BaseModel):
    namespace: str | None = None

    key: str

    config: RateLimitTokenBucketConfig
    """Rate limiter configuration"""
