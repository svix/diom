# this file is @generated
import typing as t

from pydantic import BaseModel

from .queue_msg_out import QueueMsgOut


class MsgQueueReceiveOut(BaseModel):
    msgs: t.List[QueueMsgOut]
