# this file is @generated
import typing as t

from .common import BaseModel

from .msg_in import MsgIn


class AppendToStreamIn(BaseModel):
    msgs: t.List[MsgIn]

    name: str
