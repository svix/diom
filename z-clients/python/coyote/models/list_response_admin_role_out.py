# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .admin_role_out import AdminRoleOut


class ListResponseAdminRoleOut(BaseModel):
    data: t.List[AdminRoleOut]

    iterator: t.Optional[str] = None

    prev_iterator: t.Optional[str] = Field(default=None, alias="prev_iterator")

    done: bool
