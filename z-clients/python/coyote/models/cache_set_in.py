# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class CacheSetIn(BaseModel):
    namespace: t.Optional[str] = None

    value: bytes

    ttl_ms: int = Field(alias="ttl_ms")
    """Time to live in milliseconds"""


class _CacheSetIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str

    value: bytes

    ttl_ms: int = Field(alias="ttl_ms")
    """Time to live in milliseconds"""
