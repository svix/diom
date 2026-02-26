# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class CacheGetOut(BaseModel):
    expiry: t.Optional[datetime] = None
    """Time of expiry"""

    key: str

    value: t.List[int]
