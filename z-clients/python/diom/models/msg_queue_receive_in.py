# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgQueueReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    batch_size: t.Optional[int] = None

    lease_duration_ms: t.Optional[int] = None

    batch_wait_ms: t.Optional[int] = None
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgQueueReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    batch_size: t.Optional[int] = None

    lease_duration_ms: t.Optional[int] = None

    batch_wait_ms: t.Optional[int] = None
    """Maximum time (in milliseconds) to wait for messages before returning."""
