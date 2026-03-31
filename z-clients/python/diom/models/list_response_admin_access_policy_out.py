# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .admin_access_policy_out import AdminAccessPolicyOut


class ListResponseAdminAccessPolicyOut(BaseModel):
    data: t.List[AdminAccessPolicyOut]

    iterator: t.Optional[str] = None

    prev_iterator: t.Optional[str] = Field(default=None, alias="prev_iterator")

    done: bool
