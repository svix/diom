# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class StreamMsgOut(BaseModel):
    headers: t.Optional[t.Dict[str, str]] = None

    offset: int

    timestamp: datetime

    topic: str

    value: bytes
