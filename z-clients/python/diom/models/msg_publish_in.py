# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_in import MsgIn


class MsgPublishIn(BaseModel):
    namespace: str | None = None

    msgs: t.List[MsgIn]

    idempotency_key: str | None = None


class _MsgPublishIn(BaseModel):
    namespace: str | None = None

    topic: str

    msgs: t.List[MsgIn]

    idempotency_key: str | None = None
