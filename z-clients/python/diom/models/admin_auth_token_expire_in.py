# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class AdminAuthTokenExpireIn(BaseModel):
    id: str

    expiry: TimeDeltaMs | None = Field(alias="expiry_ms", default=None)
    """Milliseconds from now until the token expires. `None` means expire immediately."""
