# this file is @generated

from ..internal.base_model import BaseModel

from .consistency import Consistency


class MsgTopicDescribeIn(BaseModel):
    namespace: str | None = None

    consistency: Consistency | None = None


class _MsgTopicDescribeIn(BaseModel):
    namespace: str | None = None

    topic: str

    consistency: Consistency | None = None
