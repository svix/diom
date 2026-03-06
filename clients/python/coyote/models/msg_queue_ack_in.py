# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueAckIn(BaseModel):
    msg_ids: t.List[str] = Field(alias="msg_ids")


class _MsgQueueAckIn(BaseModel):
    topic: str

    msg_ids: t.List[str] = Field(alias="msg_ids")
