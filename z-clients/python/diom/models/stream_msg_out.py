# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class StreamMsgOut(BaseModel):
    offset: int

    topic: str

    value: bytes

    headers: t.Dict[str, str]

    timestamp: int

    scheduled_at: int | None = None
