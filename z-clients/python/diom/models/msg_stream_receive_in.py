# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .seek_position import SeekPosition


class MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    batch_size: t.Optional[int] = None

    lease_duration_ms: t.Optional[int] = None

    default_starting_position: t.Optional[SeekPosition] = None

    batch_wait_ms: t.Optional[int] = None
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    batch_size: t.Optional[int] = None

    lease_duration_ms: t.Optional[int] = None

    default_starting_position: t.Optional[SeekPosition] = None

    batch_wait_ms: t.Optional[int] = None
    """Maximum time (in milliseconds) to wait for messages before returning."""
