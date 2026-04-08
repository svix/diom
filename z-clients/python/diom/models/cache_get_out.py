# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel


class CacheGetOut(BaseModel):
    expiry: datetime | None = None
    """Time of expiry"""

    value: bytes | None = None
