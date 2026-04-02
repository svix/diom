# this file is @generated
import typing as t

from pydantic import BaseModel

from .msg_publish_out_topic import MsgPublishOutTopic


class MsgPublishOut(BaseModel):
    topics: t.List[MsgPublishOutTopic]
