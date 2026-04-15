# this file is @generated

from ..internal.base_model import BaseModel

from .retention import Retention


class MsgNamespaceConfigureIn(BaseModel):
    retention: Retention | None = None


class _MsgNamespaceConfigureIn(BaseModel):
    name: str

    retention: Retention | None = None
