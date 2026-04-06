# this file is @generated
from datetime import datetime

from ..internal.base_model import BaseModel


class AdminAuthTokenOut(BaseModel):
    id: str

    name: str

    created: datetime

    updated: datetime

    expiry: datetime | None = None

    role: str

    enabled: bool
    """Whether this token is currently enabled."""
