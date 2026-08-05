# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgFifoAckIn(BaseModel):
    namespace: str | None = None

    msg_ids: t.List[str]


class _MsgFifoAckIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    msg_ids: t.List[str]
