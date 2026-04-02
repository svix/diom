# this file is @generated
import typing as t

from pydantic import BaseModel


class MsgQueueConfigureOut(BaseModel):
    retry_schedule: t.List[int]

    dlq_topic: t.Optional[str] = None
