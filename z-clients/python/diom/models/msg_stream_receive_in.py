# this file is @generated

from pydantic import BaseModel

from .seek_position import SeekPosition


class MsgStreamReceiveIn(BaseModel):
    namespace: str | None = None

    batch_size: int | None = None

    lease_duration_ms: int | None = None

    default_starting_position: SeekPosition | None = None

    batch_wait_ms: int | None = None
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgStreamReceiveIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    batch_size: int | None = None

    lease_duration_ms: int | None = None

    default_starting_position: SeekPosition | None = None

    batch_wait_ms: int | None = None
    """Maximum time (in milliseconds) to wait for messages before returning."""
