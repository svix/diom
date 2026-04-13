# this file is @generated
import typing as t

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class StreamMsgOut(BaseModel):
    offset: int

    topic: str

    value: bytes

    headers: t.Dict[str, str]

    timestamp: UnixTimestampMs

    scheduled_at: UnixTimestampMs | None = None
