# this file is @generated

from ..internal.base_model import BaseModel


class MsgPublishOutTopic(BaseModel):
    topic: str

    start_offset: int

    offset: int
