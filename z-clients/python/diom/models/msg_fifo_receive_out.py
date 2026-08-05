# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .fifo_msg_out import FifoMsgOut


class MsgFifoReceiveOut(BaseModel):
    msgs: t.List[FifoMsgOut]
