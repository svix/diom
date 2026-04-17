# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs

from .seek_position import SeekPosition


class MsgStreamSeekIn(BaseModel):
    namespace: str | None = None

    offset: int | None = None

    position: SeekPosition | None = None

    timestamp: UnixTimestampMs | None = None


class _MsgStreamSeekIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    offset: int | None = None

    position: SeekPosition | None = None

    timestamp: UnixTimestampMs | None = None
