# this file is @generated

from ..internal.base_model import BaseModel

from .consistency import Consistency


class MsgTopicListIn(BaseModel):
    namespace: str | None = None

    consistency: Consistency | None = None

    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""
