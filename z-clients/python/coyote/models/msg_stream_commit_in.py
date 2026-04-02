# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgStreamCommitIn(BaseModel):
    namespace: t.Optional[str] = None

    offset: int


class _MsgStreamCommitIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    offset: int
