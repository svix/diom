# this file is @generated

from ..internal.base_model import BaseModel


class MsgStreamCancelLeaseIn(BaseModel):
    namespace: str | None = None


class _MsgStreamCancelLeaseIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str
