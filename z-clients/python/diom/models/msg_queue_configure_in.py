# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgQueueConfigureIn(BaseModel):
    namespace: str | None = None

    retry_schedule: t.List[int] | None = None

    dlq_topic: str | None = None


class _MsgQueueConfigureIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    retry_schedule: t.List[int] | None = None

    dlq_topic: str | None = None
