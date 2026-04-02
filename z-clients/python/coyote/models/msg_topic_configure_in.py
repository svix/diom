# this file is @generated

from pydantic import BaseModel


class MsgTopicConfigureIn(BaseModel):
    namespace: str | None = None

    partitions: int


class _MsgTopicConfigureIn(BaseModel):
    namespace: str | None = None

    topic: str

    partitions: int
