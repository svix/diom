# this file is @generated

from ..internal.base_model import BaseModel


class CacheGetOut(BaseModel):
    expiry: int | None = None
    """Time of expiry"""

    value: bytes | None = None
