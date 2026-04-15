# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class IdempotencyConfigureNamespaceOut(BaseModel):
    name: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
