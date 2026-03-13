# this file is @generated

from ..internal.base_model import BaseModel

from .rate_limit_token_bucket_config import RateLimitTokenBucketConfig


class RateLimitGetRemainingIn(BaseModel):
    key: str

    config: RateLimitTokenBucketConfig
    """Rate limiter configuration"""
