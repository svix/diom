# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class KvGetOut(BaseModel):
    expiry: UnixTimestampMs | None = None
    """Time of expiry"""

    value: bytes | None = None

    version: int
    """Opaque version token for optimistic concurrency control.
    Pass as `version` in a subsequent `set` to perform a conditional write."""
