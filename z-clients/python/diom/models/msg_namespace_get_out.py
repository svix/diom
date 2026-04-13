# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs

from .retention import Retention


class MsgNamespaceGetOut(BaseModel):
    name: str

    retention: Retention

    created: UnixTimestampMs

    updated: UnixTimestampMs
