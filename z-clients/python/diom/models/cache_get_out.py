# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class CacheGetOut(BaseModel):
    expiry: UnixTimestampMs | None = None
    """Time of expiry"""

    value: bytes | None = None
