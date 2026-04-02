# this file is @generated

from pydantic import BaseModel

from .consistency import Consistency


class KvGetIn(BaseModel):
    namespace: str | None = None

    consistency: Consistency | None = None


class _KvGetIn(BaseModel):
    namespace: str | None = None

    key: str

    consistency: Consistency | None = None
