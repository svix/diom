# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class AdminAuthTokenOut(BaseModel):
    id: str

    name: str

    created: UnixTimestampMs

    updated: UnixTimestampMs

    expiry: UnixTimestampMs | None = None

    role: str

    enabled: bool
    """Whether this token is currently enabled."""

    expired: bool
    """Whether this token has expired.

    Expired tokens may be pruned in the background at any time."""
