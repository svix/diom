# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel


class KvGetOut(BaseModel):
    expiry: datetime | None = None
    """Time of expiry"""

    value: bytes | None = None

    version: int
    """Opaque version token for optimistic concurrency control.
    Pass as `version` in a subsequent `set` to perform a conditional write."""
