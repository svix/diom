# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule import AccessRule


class AdminRoleUpsertIn(BaseModel):
    id: str

    description: str

    rules: t.Optional[t.List[AccessRule]] = None

    policies: t.Optional[t.List[str]] = None

    context: t.Optional[t.Dict[str, str]] = None
