# this file is @generated

from ..internal.base_model import BaseModel


class IdempotencyStartIn(BaseModel):
    ttl: int
    """TTL in seconds for the lock/response"""


class _IdempotencyStartIn(BaseModel):
    key: str

    ttl: int
    """TTL in seconds for the lock/response"""
