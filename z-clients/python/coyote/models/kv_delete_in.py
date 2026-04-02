# this file is @generated
import typing as t

from pydantic import BaseModel


class KvDeleteIn(BaseModel):
    namespace: t.Optional[str] = None


class _KvDeleteIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str
