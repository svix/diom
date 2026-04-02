# this file is @generated

from pydantic import BaseModel


class KvDeleteIn(BaseModel):
    namespace: str | None = None


class _KvDeleteIn(BaseModel):
    namespace: str | None = None

    key: str
