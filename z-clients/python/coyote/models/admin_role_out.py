# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule import AccessRule


class AdminRoleOut(BaseModel):
    id: str

    description: str

    rules: t.List[AccessRule]

    policies: t.List[str]

    context: t.Dict[str, str]

    created: int

    updated: int
