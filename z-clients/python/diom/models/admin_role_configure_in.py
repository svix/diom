# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule import AccessRule


class AdminRoleConfigureIn(BaseModel):
    id: str

    description: str

    rules: t.List[AccessRule] | None = None

    policies: t.List[str] | None = None

    context: t.Dict[str, str] | None = None
