# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs

from .seek_position import SeekPosition


class MsgStreamReceiveIn(BaseModel):
    namespace: str | None = None

    batch_size: int | None = None

    lease_duration: TimeDeltaMs | None = Field(alias="lease_duration_ms", default=None)

    default_starting_position: SeekPosition | None = None

    batch_wait: TimeDeltaMs | None = Field(alias="batch_wait_ms", default=None)
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgStreamReceiveIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    batch_size: int | None = None

    lease_duration: TimeDeltaMs | None = Field(alias="lease_duration_ms", default=None)

    default_starting_position: SeekPosition | None = None

    batch_wait: TimeDeltaMs | None = Field(alias="batch_wait_ms", default=None)
    """Maximum time (in milliseconds) to wait for messages before returning."""
