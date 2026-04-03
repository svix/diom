# this file is @generated
import typing as t

from pydantic import BaseModel


class MsgQueueExtendLeaseIn(BaseModel):
    namespace: str | None = None

    msg_ids: t.List[str]

    lease_duration_ms: int | None = None


class _MsgQueueExtendLeaseIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str

    msg_ids: t.List[str]

    lease_duration_ms: int | None = None
