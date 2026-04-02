# this file is @generated
import typing as t

from pydantic import BaseModel


class MsgQueueNackIn(BaseModel):
    namespace: t.Optional[str] = None

    msg_ids: t.List[str]


class _MsgQueueNackIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    msg_ids: t.List[str]
