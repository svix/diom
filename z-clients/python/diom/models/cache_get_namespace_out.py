# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheGetNamespaceOut(BaseModel):
    name: str

    eviction_policy: EvictionPolicy

    created: datetime

    updated: datetime
