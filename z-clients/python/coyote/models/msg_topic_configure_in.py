# this file is @generated
import typing as t

from pydantic import BaseModel


class MsgTopicConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    partitions: int


class _MsgTopicConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    partitions: int
