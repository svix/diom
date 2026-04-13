# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs

from .eviction_policy import EvictionPolicy


class CacheGetNamespaceOut(BaseModel):
    name: str

    eviction_policy: EvictionPolicy

    created: UnixTimestampMs

    updated: UnixTimestampMs
