# this file is @generated

from ..internal.base_model import BaseModel


class CacheDeleteIn(BaseModel):
    namespace: str | None = None


class _CacheDeleteIn(BaseModel):
    namespace: str | None = None

    key: str
