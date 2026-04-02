# this file is @generated

from pydantic import BaseModel


class AdminAuthTokenListIn(BaseModel):
    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""
