# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class CacheSetIn(BaseModel):
    namespace: str | None = None

    ttl: TimeDeltaMs = Field(alias="ttl_ms")
    """Time to live in milliseconds"""


class _CacheSetIn(BaseModel):
    namespace: str | None = None

    key: str

    value: bytes

    ttl: TimeDeltaMs = Field(alias="ttl_ms")
    """Time to live in milliseconds"""
