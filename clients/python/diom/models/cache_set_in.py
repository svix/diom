# this file is @generated

from ..internal.base_model import BaseModel


class CacheSetIn(BaseModel):
    key: str

    ttl: int
    """Time to live in milliseconds"""

    value: bytes
