# this file is @generated

from pydantic import BaseModel


class MsgQueueReceiveIn(BaseModel):
    namespace: str | None = None

    batch_size: int | None = None

    lease_duration_ms: int | None = None

    batch_wait_ms: int | None = None
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgQueueReceiveIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    batch_size: int | None = None

    lease_duration_ms: int | None = None

    batch_wait_ms: int | None = None
    """Maximum time (in milliseconds) to wait for messages before returning."""
