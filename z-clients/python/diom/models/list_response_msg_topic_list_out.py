# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_topic_list_out import MsgTopicListOut


class ListResponseMsgTopicListOut(BaseModel):
    data: t.List[MsgTopicListOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
