# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    batch_size: t.Optional[int] = Field(default=None, alias="batch_size")

    lease_duration_millis: t.Optional[int] = Field(
        default=None, alias="lease_duration_millis"
    )

    default_starting_position: t.Optional[str] = Field(
        default=None, alias="default_starting_position"
    )


class _MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")

    batch_size: t.Optional[int] = Field(default=None, alias="batch_size")

    lease_duration_millis: t.Optional[int] = Field(
        default=None, alias="lease_duration_millis"
    )

    default_starting_position: t.Optional[str] = Field(
        default=None, alias="default_starting_position"
    )
