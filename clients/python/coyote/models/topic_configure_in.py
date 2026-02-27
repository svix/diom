# this file is @generated

from ..internal.base_model import BaseModel


class TopicConfigureIn(BaseModel):
    name: str

    partitions: int

    topic: str
