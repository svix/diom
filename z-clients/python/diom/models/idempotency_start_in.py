# this file is @generated

from pydantic import BaseModel


class IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    ttl_ms: int
    """TTL in milliseconds for the lock/response"""


class _IdempotencyStartIn(BaseModel):
    namespace: str | None = None

    key: str

    ttl_ms: int
    """TTL in milliseconds for the lock/response"""
