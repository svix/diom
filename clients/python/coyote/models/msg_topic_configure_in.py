# this file is @generated

from ..internal.base_model import BaseModel


class MsgTopicConfigureIn(BaseModel):
    topic: str

    partitions: int
