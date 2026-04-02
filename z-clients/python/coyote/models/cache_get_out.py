# this file is @generated
from datetime import datetime

from pydantic import BaseModel


class CacheGetOut(BaseModel):
    expiry: datetime | None = None
    """Time of expiry"""

    value: bytes | None = None
