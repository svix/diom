# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class IdempotencyCreateNamespaceOut(BaseModel):
    name: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
