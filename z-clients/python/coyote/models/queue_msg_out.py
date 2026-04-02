# this file is @generated
import typing as t
from datetime import datetime

from pydantic import BaseModel


class QueueMsgOut(BaseModel):
    msg_id: str

    value: bytes

    headers: t.Optional[t.Dict[str, str]] = None

    timestamp: datetime

    scheduled_at: t.Optional[datetime] = None
