# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .admin_access_policy_out import AdminAccessPolicyOut


class ListResponseAdminAccessPolicyOut(BaseModel):
    data: t.List[AdminAccessPolicyOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
