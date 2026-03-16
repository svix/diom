# this file is @generated

from ..internal.base_model import BaseModel


class IdempotencyCompleteIn(BaseModel):
    response: bytes
    """The response to cache"""

    ttl: int
    """TTL in seconds for the cached response"""


class _IdempotencyCompleteIn(BaseModel):
    key: str

    response: bytes
    """The response to cache"""

    ttl: int
    """TTL in seconds for the cached response"""
