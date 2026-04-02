# this file is @generated
import typing as t

from pydantic import BaseModel


class CacheSetIn(BaseModel):
    namespace: t.Optional[str] = None

    value: bytes

    ttl_ms: int
    """Time to live in milliseconds"""


class _CacheSetIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    value: bytes

    ttl_ms: int
    """Time to live in milliseconds"""
