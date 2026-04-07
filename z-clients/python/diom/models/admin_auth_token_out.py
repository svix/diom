# this file is @generated

from ..internal.base_model import BaseModel


class AdminAuthTokenOut(BaseModel):
    id: str

    name: str

    created: int

    updated: int

    expiry: int | None = None

    role: str

    enabled: bool
    """Whether this token is currently enabled."""
