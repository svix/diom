# this file is @generated

from ..internal.base_model import BaseModel


class MsgTopicConfigureIn(BaseModel):
    partitions: int

    topic: str
