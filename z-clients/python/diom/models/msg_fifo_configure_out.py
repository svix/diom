# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgFifoConfigureOut(BaseModel):
    retry_schedule: t.List[int]

    dlq_topic: str | None = None
