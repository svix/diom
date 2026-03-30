# this file is @generated
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel

from .eviction_policy import EvictionPolicy


class CacheCreateNamespaceOut(BaseModel):
    name: str

    eviction_policy: EvictionPolicy = Field(alias="eviction_policy")

    created: datetime

    updated: datetime
