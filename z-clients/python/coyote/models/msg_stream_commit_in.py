# this file is @generated

from pydantic import BaseModel


class MsgStreamCommitIn(BaseModel):
    namespace: str | None = None

    offset: int


class _MsgStreamCommitIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    offset: int
