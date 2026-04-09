# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class AdminAuthTokenCreateIn(BaseModel):
    name: str

    role: str

    expiry: TimeDeltaMs | None = Field(alias="expiry_ms", default=None)
    """Milliseconds from now until the token expires."""

    enabled: bool | None = None
    """Whether the token is enabled. Defaults to `true`."""
