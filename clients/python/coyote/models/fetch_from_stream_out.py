# this file is @generated
import typing as t

from .common import BaseModel

from .msg_out import MsgOut


class FetchFromStreamOut(BaseModel):
    msgs: t.List[MsgOut]
