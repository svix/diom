# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class KvGetNamespaceOut(BaseModel):
    name: str

    created: UnixTimestampMs

    updated: UnixTimestampMs
