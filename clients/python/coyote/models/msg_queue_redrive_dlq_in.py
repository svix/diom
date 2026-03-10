# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueRedriveDlqIn(BaseModel):
    pass


class _MsgQueueRedriveDlqIn(BaseModel):
    topic: str

    consumer_group: str = Field(alias="consumer_group")
