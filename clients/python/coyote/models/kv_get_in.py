# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .consistency import Consistency


class KvGetIn(BaseModel):
    consistency: t.Optional[Consistency] = None


class _KvGetIn(BaseModel):
    key: str

    consistency: t.Optional[Consistency] = None
