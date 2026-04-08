# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class StreamMsgOut(BaseModel):
    offset: int

    topic: str

    value: bytes

    headers: t.Dict[str, str] | None = None

    timestamp: datetime

    scheduled_at: datetime | None = None
