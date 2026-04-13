# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class AdminAuthTokenRotateOut(BaseModel):
    id: str

    token: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
