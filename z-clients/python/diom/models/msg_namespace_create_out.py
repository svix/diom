# this file is @generated

from ..internal.base_model import BaseModel

from .retention import Retention


class MsgNamespaceCreateOut(BaseModel):
    name: str

    retention: Retention

    created: int

    updated: int
