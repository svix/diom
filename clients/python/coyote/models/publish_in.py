# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_in2 import MsgIn2


class PublishIn(BaseModel):
    msgs: t.List[MsgIn2]

    name: str

    topic: str
