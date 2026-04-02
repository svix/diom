# this file is @generated

from pydantic import BaseModel

from .retention import Retention


class MsgNamespaceCreateIn(BaseModel):
    retention: Retention | None = None


class _MsgNamespaceCreateIn(BaseModel):
    name: str

    retention: Retention | None = None
