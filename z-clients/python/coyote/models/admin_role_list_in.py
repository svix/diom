# this file is @generated

from ..internal.base_model import BaseModel


class AdminRoleListIn(BaseModel):
    limit: int | None = None
    """Limit the number of returned items"""

    iterator: str | None = None
    """The iterator returned from a prior invocation"""
