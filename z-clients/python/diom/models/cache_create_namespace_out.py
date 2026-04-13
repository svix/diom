# this file is @generated

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceOut(BaseModel):
    name: str

    eviction_policy: EvictionPolicy

    created: int

    updated: int
