# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgStreamReceiveIn(BaseModel):
    batch_size: t.Optional[int] = Field(default=None, alias="batch_size")

    consumer_group: str = Field(alias="consumer_group")

    lease_duration_millis: t.Optional[int] = Field(
        default=None, alias="lease_duration_millis"
    )

    topic: str
