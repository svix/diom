# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_publish_out_msg import MsgPublishOutMsg


class MsgPublishOut(BaseModel):
    msgs: t.List[MsgPublishOutMsg]
