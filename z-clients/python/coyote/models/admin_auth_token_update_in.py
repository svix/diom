# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class AdminAuthTokenUpdateIn(BaseModel):
    id: str

    name: str | None = None

    expiry: TimeDeltaMs | None = Field(alias="expiry_ms", default=None)

    enabled: bool | None = None
