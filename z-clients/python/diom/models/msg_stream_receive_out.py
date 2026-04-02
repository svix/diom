# this file is @generated
import typing as t

from pydantic import BaseModel

from .stream_msg_out import StreamMsgOut


class MsgStreamReceiveOut(BaseModel):
    msgs: t.List[StreamMsgOut]
