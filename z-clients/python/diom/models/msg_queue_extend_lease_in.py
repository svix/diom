# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class MsgQueueExtendLeaseIn(BaseModel):
    namespace: str | None = None

    msg_ids: t.List[str]

    lease_duration: TimeDeltaMs | None = Field(alias="lease_duration_ms", default=None)


class _MsgQueueExtendLeaseIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    msg_ids: t.List[str]

    lease_duration: TimeDeltaMs | None = Field(alias="lease_duration_ms", default=None)
