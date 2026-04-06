# this file is @generated

from ..internal.base_model import BaseModel


class MsgQueueRedriveDlqIn(BaseModel):
    namespace: str | None = None


class _MsgQueueRedriveDlqIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str
