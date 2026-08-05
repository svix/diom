# this file is @generated
import typing as t

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class FifoMsgOut(BaseModel):
    msg_id: str

    key: str | None = None

    value: bytes

    headers: t.Dict[str, str]

    timestamp: UnixTimestampMs

    scheduled_at: UnixTimestampMs | None = None
