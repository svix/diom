# this file is @generated

from pydantic import BaseModel


class MsgQueueRedriveDlqIn(BaseModel):
    namespace: str | None = None


class _MsgQueueRedriveDlqIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str
