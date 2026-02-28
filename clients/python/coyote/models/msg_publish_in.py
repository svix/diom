# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_in import MsgIn


class MsgPublishIn(BaseModel):
    msgs: t.List[MsgIn]

    topic: str
