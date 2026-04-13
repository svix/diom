# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class QueueMsgOut(BaseModel):
    msg_id: str

    value: bytes

    headers: t.Dict[str, str]

    timestamp: datetime

    scheduled_at: datetime | None = None
