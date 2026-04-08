# this file is @generated

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceIn(BaseModel):
    name: str

    eviction_policy: EvictionPolicy | None = None
