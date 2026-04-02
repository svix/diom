# this file is @generated
import typing as t

from pydantic import BaseModel


class AdminAuthTokenCreateIn(BaseModel):
    name: str

    role: str

    expiry_ms: t.Optional[int] = None
    """Milliseconds from now until the token expires."""

    enabled: t.Optional[bool] = None
    """Whether the token is enabled. Defaults to `true`."""
