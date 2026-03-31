# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class AdminRoleListIn(BaseModel):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""

    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
