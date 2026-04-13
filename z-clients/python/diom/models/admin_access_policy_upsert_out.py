# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class AdminAccessPolicyUpsertOut(BaseModel):
    id: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
