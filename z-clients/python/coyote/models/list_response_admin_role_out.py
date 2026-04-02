# this file is @generated
import typing as t

from pydantic import BaseModel

from .admin_role_out import AdminRoleOut


class ListResponseAdminRoleOut(BaseModel):
    data: t.List[AdminRoleOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
