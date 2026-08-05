# this file is @generated

from ..internal.base_model import BaseModel


class MsgFifoRedriveDlqIn(BaseModel):
    namespace: str | None = None


class _MsgFifoRedriveDlqIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str
