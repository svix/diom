# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class AdminAccessPolicyConfigureOut(BaseModel):
    id: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
