# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgQueueRedriveDlqIn(BaseModel):
    namespace: t.Optional[str] = None


class _MsgQueueRedriveDlqIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str
