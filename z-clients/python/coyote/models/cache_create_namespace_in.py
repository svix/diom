# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceIn(BaseModel):
    name: str

    eviction_policy: t.Optional[EvictionPolicy] = None
