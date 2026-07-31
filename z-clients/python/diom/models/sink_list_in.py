# this file is @generated

from ..internal.base_model import BaseModel


class SinkListIn(BaseModel):
    namespace: str | None = None

    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""


class _SinkListIn(BaseModel):
    namespace: str | None = None

    topic: str

    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""
