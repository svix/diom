# this file is @generated

from pydantic import BaseModel


class CacheSetIn(BaseModel):
    namespace: str | None = None

    value: bytes

    ttl_ms: int
    """Time to live in milliseconds"""


class _CacheSetIn(BaseModel):
    namespace: str | None = None

    key: str

    value: bytes

    ttl_ms: int
    """Time to live in milliseconds"""
