# this file is @generated

from pydantic import BaseModel


class CacheDeleteIn(BaseModel):
    namespace: str | None = None


class _CacheDeleteIn(BaseModel):
    namespace: str | None = None

    key: str
