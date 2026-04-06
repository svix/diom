# this file is @generated

from ..internal.base_model import BaseModel


class AdminAuthTokenListIn(BaseModel):
    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""
