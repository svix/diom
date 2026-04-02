# this file is @generated

from pydantic import BaseModel


class AdminAuthTokenCreateIn(BaseModel):
    name: str

    role: str

    expiry_ms: int | None = None
    """Milliseconds from now until the token expires."""

    enabled: bool | None = None
    """Whether the token is enabled. Defaults to `true`."""
