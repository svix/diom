# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgQueueConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    retry_schedule: t.Optional[t.List[int]] = None

    dlq_topic: t.Optional[str] = None


class _MsgQueueConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    retry_schedule: t.Optional[t.List[int]] = None

    dlq_topic: t.Optional[str] = None
