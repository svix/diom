# this file is @generated
import typing as t

from pydantic import BaseModel


class IdempotencyStartIn(BaseModel):
    namespace: t.Optional[str] = None

    ttl_ms: int
    """TTL in milliseconds for the lock/response"""


class _IdempotencyStartIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    ttl_ms: int
    """TTL in milliseconds for the lock/response"""
