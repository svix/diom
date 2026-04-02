# this file is @generated
import typing as t

from pydantic import BaseModel


class CacheDeleteIn(BaseModel):
    namespace: t.Optional[str] = None


class _CacheDeleteIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str
