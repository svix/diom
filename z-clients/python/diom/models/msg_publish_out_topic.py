# this file is @generated

from pydantic import BaseModel


class MsgPublishOutTopic(BaseModel):
    topic: str

    start_offset: int

    offset: int
