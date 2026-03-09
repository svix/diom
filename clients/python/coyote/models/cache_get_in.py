# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .consistency import Consistency


class CacheGetIn(BaseModel):
    consistency: t.Optional[Consistency] = None


class _CacheGetIn(BaseModel):
    key: str

    consistency: t.Optional[Consistency] = None
